use std::{rc::Rc};

mod node;
mod state;
mod ui;
mod utils;

use ux::u4;

use crate::state::State;


fn main() {
   let s = "BAC0F478E19623D5";
   let mut state = State::new(s);
   //state.try_slide_left();
//    state.try_slide_top();
//    state.try_slide_right();
//    state.try_slide_bottom();
    println!("{state}");
    println!("{}", state.is_solvable());
}
