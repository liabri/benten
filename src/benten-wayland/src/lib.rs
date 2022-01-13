use benten::{ BentenEngine, BentenResponse, BentenConfig };
use mio::{unix::SourceFd, Events as MioEvents, Interest, Poll, Token};
use mio_timerfd::{ClockId, TimerFd};
use std::time::{Duration, Instant};

use wayland_client::{ event_enum, DispatchData, Display, Filter, GlobalManager, Main };
use wayland_client::protocol::{ wl_keyboard::KeyState, wl_seat::WlSeat };

use wayland_protocols::misc::zwp_input_method_v2::client::{
    zwp_input_method_keyboard_grab_v2::{ Event as KeyEvent, ZwpInputMethodKeyboardGrabV2 },
    zwp_input_method_manager_v2::ZwpInputMethodManagerV2,
    zwp_input_method_v2::{ Event as ImEvent, ZwpInputMethodV2 },
};

use zwp_virtual_keyboard::virtual_keyboard_unstable_v1::{
    zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1,
    zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1,
};

event_enum! {
    Events |
    Key => ZwpInputMethodKeyboardGrabV2,
    Im => ZwpInputMethodV2
}

pub fn init() {
    let display = Display::connect_to_env().map_err(|e| log::error!("Failed to connect to wayland display: {}", e)).unwrap();
    let mut event_queue = display.create_event_queue();
    let attached_display = display.attach(event_queue.token());
    let globals = GlobalManager::new(&attached_display);

    event_queue.sync_roundtrip(&mut (), |_, _, _| ()).unwrap();

    let seat = globals.instantiate_exact::<WlSeat>(1).expect("Failed to load Seat");
    let im_manager = globals.instantiate_exact::<ZwpInputMethodManagerV2>(1).expect("Failed to load InputManager");
    let vk_manager = globals.instantiate_exact::<ZwpVirtualKeyboardManagerV1>(1).expect("Failed to load VirtualKeyboardManager");

    let filter = Filter::new(|ev, _filter, mut data| {
        let ctx = BentenContext::new_data(&mut data);
        match ev {
            Events::Key { event, .. } => {
            	println!("key event");
                ctx.handle_key_ev(event);
            }
            Events::Im { event, .. } => {
            	println!("im event");
                ctx.handle_im_ev(event);
            }
        }
    });

    let vk = vk_manager.create_virtual_keyboard(&seat);
    let im = im_manager.get_input_method(&seat);
    let grab = im.grab_keyboard();
    grab.assign(filter.clone());
    im.assign(filter);

    let mut timer = TimerFd::new(ClockId::Monotonic).expect("Initialize timer");
    let mut poll = Poll::new().expect("Initialize epoll()");
    
    let registry = poll.registry();

    const POLL_WAYLAND: Token = Token(0);
    registry.register(
        &mut SourceFd(&display.get_connection_fd()),
        POLL_WAYLAND,
        Interest::READABLE | Interest::WRITABLE,
    ).expect("Register wayland socket to the epoll()");

    const POLL_TIMER: Token = Token(1);
    registry.register(
    	&mut timer, 
    	POLL_TIMER, 
    	Interest::READABLE
    ).expect("Register timer to the epoll()");

    // Initialize context
    let mut ctx = BentenContext::new(vk, im, grab, timer);
    event_queue.sync_roundtrip(&mut ctx, |_, _, _| ()).unwrap();
    log::info!("Server successfully initialised !");

    // Non-blocking event loop
    let mut events = MioEvents::with_capacity(1024);
    let stop_reason = 'main: loop {
        use std::io::ErrorKind;

        // Sleep until next event
        if let Err(e) = poll.poll(&mut events, None) {
            // Should retry on EINTR
            if e.kind() == ErrorKind::Interrupted {
                continue;
            }

            break Err(e);
        }

        for event in &events {
            match event.token() {
                POLL_WAYLAND => {}
                POLL_TIMER => {
                    if let Err(e) = ctx.handle_timer_ev() {
                        break 'main Err(e);
                    }
                }
                _ => unreachable!(),
            }
        }

        // Perform read() only when it's ready, returns None when there're already pending events
        if let Some(guard) = event_queue.prepare_read() {
            if let Err(e) = guard.read_events() {
                // ErrorKind::WouldBlock here means there's no new messages to read
                if e.kind() != ErrorKind::WouldBlock {
                    break Err(e);
                }
            }
        }

        if let Err(e) = event_queue.dispatch_pending(&mut ctx, |_, _, _| {}) {
            break Err(e);
        }

        // Flush pending writes
        if let Err(e) = display.flush() {
            // ErrorKind::WouldBlock here means there're so many to write, retry later
            if e.kind() != ErrorKind::WouldBlock {
                break Err(e);
            }
        }
    };

    match stop_reason {
        Ok(()) => log::info!("Server closed gracefully"),
        Err(e) => log::error!("Server aborted: {}", e),
    }
}

struct BentenContext {
    engine: BentenEngine,
    current_state: InputMethodState,
    pending_state: InputMethodState,
    vk: Main<ZwpVirtualKeyboardV1>,
    im: Main<ZwpInputMethodV2>,
    grab: Main<ZwpInputMethodKeyboardGrabV2>,
    grab_activate: bool,
    keymap_init: bool,
    serial: u32,
    timer: TimerFd,
    repeat_state: Option<(RepeatInfo, PressState)>,
}

#[derive(PartialEq)]
pub enum InputMethodState {
    Active,
    Inactive,
}

impl Default for InputMethodState {
    fn default() -> Self {
        InputMethodState::Inactive
    }
}

#[derive(Clone, Copy)]
struct RepeatInfo {
    rate: i32,
    delay: i32,
}

#[derive(Clone, Copy)]
enum PressState {
    NotPressing,
    Pressing {
        pressed_at: Instant,
        is_repeating: bool,
        key: u32,
        wayland_time: u32,
    },
}

impl PressState {
    fn is_pressing(&self, query_key: u32) -> bool {
        if let PressState::Pressing { key, .. } = self {
            *key == query_key
        } else {
            false
        }
    }
}

impl BentenContext {
    pub fn new(vk: Main<ZwpVirtualKeyboardV1>, im: Main<ZwpInputMethodV2>, grab: Main<ZwpInputMethodKeyboardGrabV2>, timer: TimerFd) -> Self { 
        Self {
            engine: BentenEngine::new(BentenConfig { id: "japanese".to_string() }),
            current_state: InputMethodState::Inactive,
            pending_state: InputMethodState::Inactive,
            serial: 0,
            grab_activate: false,
            keymap_init: false,
            vk,
            im,
            grab,
            timer,
            repeat_state: None,
        }
    }

    pub fn new_data<'a>(data: &'a mut DispatchData) -> &'a mut Self {
        data.get::<Self>().unwrap()
    }

    pub fn handle_im_ev(&mut self, ev: ImEvent) {
        match ev {
            ImEvent::Activate => {
                self.pending_state = InputMethodState::Active
            },

            ImEvent::Deactivate => {
                self.pending_state = InputMethodState::Inactive;
            },

            ImEvent::Unavailable => {
                log::error!("input method unavailable, is another server already running ?");
                panic!("unavailable")
            },

            ImEvent::Done => {
                if self.current_state==InputMethodState::Inactive 
                  && self.pending_state==InputMethodState::Active { 
                    self.grab_activate = true;
                } 

                else if self.current_state==InputMethodState::Active 
               	  && self.pending_state==InputMethodState::Inactive {
                    self.grab_activate = false;

                    // Focus lost, reset states
                    self.engine.reset();

                    // Input deactivated, stop repeating
                    self.timer.disarm().unwrap();
                    if let Some((_, ref mut press_state)) = self.repeat_state {
                        *press_state = PressState::NotPressing
                    }
                }

                self.current_state = std::mem::take(&mut self.pending_state);
            },

            _ => {}
        }
    }

    pub fn handle_key_ev(&mut self, ev: KeyEvent) {
        match ev {
            KeyEvent::Keymap { fd, format, size } => {
                if !self.keymap_init {
                    self.vk.keymap(format as _, fd, size);
                    self.keymap_init = true;
                }

                unsafe { libc::close(fd); }
            },

            KeyEvent::Key { state, key, time, .. } => {
            	if self.grab_activate {
	            	match state {
	            		KeyState::Pressed => {
					    	match self.engine.on_key_press((key + 8) as u16) {
					    		BentenResponse::Empty => {},
					    		BentenResponse::Commit(s) => { 
					    			self.im.commit_string(s);
					    			self.serial += 1;
					    			self.im.set_preedit_string(String::new(), -1, -1);
					    		},

					    		BentenResponse::Suggest(s) => {
					    			let len = s.len();
        							self.im.set_preedit_string(s, 0, len as _);
					    		},

					    		BentenResponse::Null => {
					    			self.vk.key(time, key, state as _);
					    			return;
					    		}
					    	}

					    	match self.repeat_state {
                                Some((info, ref mut press_state)) if !press_state.is_pressing(key) => {
                                    let duration = Duration::from_millis(info.delay as u64);
                                    self.timer.set_timeout(&duration).unwrap();
                                    *press_state = PressState::Pressing {
                                        pressed_at: Instant::now(),
                                        is_repeating: false,
                                        key,
                                        wayland_time: time,
                                    };
                                },

                                _ => {}
                            }
	            		},

	            		KeyState::Released => {
	            			let _rep = self.engine.on_key_release((key + 8) as u16);

	            			// If user released the last pressed key, clear the timer and state
		                    if let Some((.., ref mut press_state)) = self.repeat_state {
		                        if press_state.is_pressing(key) {
		                            self.timer.disarm().unwrap();
		                            *press_state = PressState::NotPressing;
		                        }
		                    }
	            		},

	            		_ => {}
	            	}
            	}

            	self.vk.key(time, key, state as _);
            },

            KeyEvent::Modifiers { mods_depressed, mods_latched, mods_locked, group, .. } => {
                //todo
                self.vk.modifiers(mods_depressed, mods_latched, mods_locked, group);
            },

            KeyEvent::RepeatInfo { rate, delay } => {
                self.repeat_state = if rate == 0 {
                    // Zero rate means disabled repeat
                    None
                } else {
                    let info = RepeatInfo { rate, delay };
                    let press_state = self.repeat_state.map(|pair| pair.1);
                    Some((info, press_state.unwrap_or(PressState::NotPressing)))
                }
            },

            _ => {}
        }
    }

    pub fn handle_timer_ev(&mut self) -> std::io::Result<()> {
        // Read timer, this MUST be called or timer will be broken
        let overrun_count = self.timer.read()?;
        if overrun_count != 1 {
            log::warn!("Some timer events were not properly handled!");
        }

        if let Some((
            info,
            PressState::Pressing {
                pressed_at,
                ref mut is_repeating,
                key,
                wayland_time,
            },
        )) = self.repeat_state {
            if !*is_repeating {
                // Start repeat
                log::trace!("Start repeating {}", key);
                let interval = &Duration::from_secs_f64(1.0 / info.rate as f64);
                self.timer.set_timeout_interval(interval)?;
                *is_repeating = true;
            }

            // Emit key repeat event
            let ev = KeyEvent::Key {
                serial: self.serial,
                time: wayland_time + pressed_at.elapsed().as_millis() as u32,
                key,
                state: KeyState::Pressed,
            };

            self.serial += 1;
            self.handle_key_ev(ev);
        } else {
            log::warn!("Received timer event when it has never received RepeatInfo.");
        }

        Ok(())
    }
}