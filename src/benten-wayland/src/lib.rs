mod context;
use context::BentenContext;

use mio::{unix::SourceFd, Events as MioEvents, Interest, Poll, Token};
use mio_timerfd::{ClockId, TimerFd};

use wayland_client::{ event_enum, Display, Filter, GlobalManager };
use wayland_client::protocol::wl_seat::WlSeat;

use wayland_protocols::misc::zwp_input_method_v2::client::{
    zwp_input_method_v2::ZwpInputMethodV2,
    zwp_input_method_manager_v2::ZwpInputMethodManagerV2,
    zwp_input_method_keyboard_grab_v2::ZwpInputMethodKeyboardGrabV2,
};


use zwp_virtual_keyboard::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;

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
                ctx.handle_key_ev(event);
            },

            Events::Im { event, .. } => {
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

    // Required for hold event of engine values
    const POLL_TIMER: Token = Token(1);
    registry.register(
    	&mut timer, 
    	POLL_TIMER, 
    	Interest::READABLE
    ).expect("Register timer to the epoll()");

    // Initialize context
    let mut ctx = BentenContext::new(vk, im, timer);
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