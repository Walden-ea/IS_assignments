mod state;
mod ui;
mod utils;
mod simple_solve;
mod informed_solve;
use informed_solve::a_star;

use crate::{state::{State, Direction:: {Top, Bottom, Left, Right}}, utils::str_to_u8_arr, simple_solve::{bfs_search, dfs_search,ids_search}};
use std::{mem, rc::Rc};
// use ux::u4;

fn main() {
    // let s = "123456789ABCDEF0";
    // let state = State::new(s);
    // println!("{:?}",state);
    // println!("{}",State{puzzle:state.try_branch(Left).expect("None!"),parent:None});

    // //let state = State{puzzle:str_to_u4_arr("123456789ABCDEF0"), parent: Some(Rc::new(State{puzzle:str_to_u4_arr("1234567890123456"), parent:None}))};
    //println!("{}", state.is_solvable());
    let state = State::new("123456789A0BDEFC");
    println!("{}",state);
    state.calc_heuristic();
    // println!("{state}");
    let state = a_star(state, Some(2)).expect("None found!!");

    //let state = dfs_search(state, 11).expect("None!");
    state.print_path();
}
