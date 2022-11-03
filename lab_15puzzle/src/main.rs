mod state;
mod ui;
mod utils;
mod simple_solve;

use crate::{state::{State, Direction:: {Top, Bottom, Left, Right}}, utils::str_to_u8_arr, simple_solve::bfs_search};
use std::{mem, rc::Rc};
// use ux::u4;

fn main() {
    // let s = "123456789ABCDEF0";
    // let state = State::new(s);
    // println!("{:?}",state);
    // println!("{}",State{puzzle:state.try_branch(Left).expect("None!"),parent:None});

    // //let state = State{puzzle:str_to_u4_arr("123456789ABCDEF0"), parent: Some(Rc::new(State{puzzle:str_to_u4_arr("1234567890123456"), parent:None}))};
    //println!("{}", state.is_solvable());
    let state = State::new("12345678A0BE9FCD");
    // println!("{state}");
    let state = bfs_search(state);
    state.print_path();
}
