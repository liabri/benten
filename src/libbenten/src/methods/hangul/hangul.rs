use crate::method::*;
use crate::layout::parser::*;
use crate::layout::layout::*;
use std::collections::HashSet;
use crate::request::{BentenRequest, BentenRequestType, self};
use crate::key::KeyState;

pub struct HangulMethod {
    pub layout: Layout,
    pub modifiers_pressed: HashSet<u16>,
    pub combination: Vec<char>,
    pub buffer: HangulBuffer,
}

pub struct HangulBuffer {
    pub choseong: Option<char>,
    pub jungseong: Option<char>,
    pub jongseong: Option<char>,
}

impl LayoutMethodTrait for HangulMethod {}

impl GenericMethod for HangulMethod {
	fn new(id: &str) -> Self {
        let layout: Layout = Layout::new(id);

        HangulMethod {
            layout,
            modifiers_pressed: HashSet::new(),
            combination: Vec::with_capacity(4),
            buffer: HangulBuffer {
            	choseong: None,
            	jungseong: None,
            	jongseong: None,
            }
        }
    }

    fn on_key_press(&mut self, key_code: u16) -> BentenRequest {
    	request::null()
    }

    fn on_key_release(&mut self, key_code: u16) -> BentenRequest {
    	request::null()
    }
}

impl LayoutHelper for HangulMethod {
    fn layout(&mut self) -> &Layout {
        &self.layout
    }

    fn modifiers_pressed(&mut self) -> &mut HashSet<u16> {
        &mut self.modifiers_pressed
    }
        
    fn layout_n_modifiers_pressed(&mut self) -> (&Layout, &mut HashSet<u16>) {
        (&self.layout, &mut self.modifiers_pressed)
    }
}

trait HangulHelper {
	fn buffer(&mut self) -> &HangulBuffer;
}

impl HangulHelper for HangulMethod {
	fn buffer(&mut self) -> &HangulBuffer {
		&self.buffer
	}
}

impl HangulMethod {
	fn choseong_to_jongseong() {}
	fn is_jungseong() {}
	// fn is_choseong(jamo: char) {
	// 	if char 
	// }
	fn is_jongseong() {}
}

// hangul_is_choseong(ucschar c)
// {
//     return (c >= 0x1100 && c <= 0x115f) ||
// 	   (c >= 0xa960 && c <= 0xa97c);
// ;
// }


trait Hangul {
	fn is_choseong(&self) -> bool;
	fn is_jungseong(&self) -> bool;
	fn is_jongseong(&self) -> bool;
	fn is_conjoinable(&self) -> bool;
	fn choseong_to_jongseong(&mut self) -> Self;
	fn jongseong_to_choseong(&mut self) -> Self;
}

impl Hangul for char {
	fn is_choseong(&self) -> bool {
        matches!(*self, 'ᄀ'..='ᅞ' | 'ꥠ'..='ꥼ')
    }

	fn is_jungseong(&self) -> bool {
        matches!(*self, 'ᅡ'..='ᆧ' | 'ힰ'..='ퟆ')
    }

	fn is_jongseong(&self) -> bool {
        matches!(*self, 'ᆨ'..='ᇿ' | 'ퟋ'..='ퟻ')
    }

    fn is_conjoinable(&self) -> bool {
        // return c >= 0x1100 && c <= 0x1112;
    	// return c >= 0x1161 && c <= 0x1175;
   	    // return c >= 0x11a7 && c <= 0x11c2;
   	    true
    }

    fn choseong_to_jongseong(&mut self) -> Self {
    	*self
    }

    fn jongseong_to_choseong(&mut self) -> Self {
    	*self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
    	let x: char = 'ᄀ';
    	assert_eq!(x.is_choseong(), true);
    }
}