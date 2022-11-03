use std::{rc::Rc};

mod node;
mod state;
mod ui;
mod utils;

use ux::u4;

use crate::state::State;


fn main() {
//     let node = node::Node{val:0,parent: Some(Rc::new(node::Node{val:5, parent:None}))};
//    println!("{node}");

//    let mut arr:[ux::u4;16] = [u4::new(0);16];

//    for i in u8::from(0)..u8::from(16){
//     arr[usize::from(i)] = u4::new(i);
//    }
//    ui::print_field(arr);
//    println!();

   let s = "123456789abcdef0";
   //println!("{:?}",utils::input_to_u4_arr(s));
   //ui::print_field(utils::input_to_u4_arr(s));

   let state = State {puzzle: utils::input_to_u4_arr(s), parent: None};
   println!("{state}");
}
