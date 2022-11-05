use std::{collections::{VecDeque, HashSet}, rc::Rc};

use crate::{State, utils::str_to_u8_arr, state::Direction};
pub fn bfs_search(state: State) -> State{

    let mut closed = HashSet::new();
    let mut open = VecDeque::from([state]);
    let goal = str_to_u8_arr("123456789ABCDEF0");

    while let Some(node) = open.pop_back(){
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

    panic!("HOW HOW NONE AAAAAAAAAAAAAAAAAA")
}


pub fn dfs_search(state: State, bound: u8) -> Option<State> {

    //let mut reses = Vec::new();
    let mut open = Vec::from([(state,u8::from(0))]);
    let goal = str_to_u8_arr("123456789ABCDEF0");

    //let depth = 0;

    while let Some((node,depth)) = open.pop(){
        if node.puzzle.iter().enumerate().all(|(i,x)| *x == goal[i]) && depth <= bound {
            //reses.push((node.clone(),depth));
            return Some(node)
        }
        let rc = Rc::new(node);
        let mut populate = |d:Direction| { 
            match rc.try_branch(d) {
                Some(new_puzzle) => {
                    open.push((State{puzzle:new_puzzle, parent: Some(Rc::clone(&rc))},depth+1));

                },
                None => return 
            }
            
        };
        if depth < bound {
        populate(Direction::Bottom);
        populate(Direction::Left);
        populate(Direction::Right);
        populate(Direction::Top);
        }
    }
    None
}

pub fn ids_search(state:State) -> State{
    for bound in (0..=80){
        if let Some(res) = dfs_search(state.clone(), bound) {
            return res
        }
    }
    panic!("HOW IS THIS *IDS* NOT WORKING IT SHOULD ")
}