use std::{collections::{VecDeque, HashSet}, rc::Rc};

use crate::{State, utils::str_to_u8_arr, state::Direction};
pub fn bfs_search(state: State) -> State{

    let mut closed = HashSet::new();
    let mut open = VecDeque::from([state]);
    let goal = str_to_u8_arr("123456789ABCDEF0");

    while let Some(node) = open.pop_back(){
        //println!("{}", node);
        if node.puzzle.iter().enumerate().all(|(i,x)| *x == goal[i]) {
            return node
        }
        let rc = Rc::new(node);
        let mut populate = |d:Direction| { 
            match rc.try_branch(d) {
                Some(new_puzzle) => {
                    if !closed.contains(&new_puzzle) {
                        open.push_front(State{puzzle:new_puzzle, parent: Some(Rc::clone(&rc))});
                        closed.insert(new_puzzle);
                    }
                },
                None => return 
            }
            
        };
        populate(Direction::Bottom);
        populate(Direction::Left);
        populate(Direction::Right);
        populate(Direction::Top);
    
    }

    State::new("hui")
}
