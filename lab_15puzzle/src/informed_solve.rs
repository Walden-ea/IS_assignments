use std::{collections::{HashSet, BinaryHeap}, cmp::Ordering, rc::Rc};
use crate::{state::{State, Direction}, utils::str_to_u8_arr};

#[derive(Hash, Clone)]
struct UberState {
    pub state: State,
    pub depth: u8
}

impl UberState{
    fn  default()-> UberState{
        UberState { state: State{puzzle:[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0], parent:None}, depth: 81 }
    }
}

pub fn a_star(state: State, bound: Option<u8>) -> Option<State>{
    let bound = bound.unwrap_or(80);
    let mut closed = HashSet::new();
    let mut open = BinaryHeap::from([UberState{state,depth: 0}]);
    let goal = str_to_u8_arr("123456789ABCDEF0");

    let mut res = UberState::default();

    while let Some(node) = open.pop(){
        // let dn = node.depth as i32;
        // let rd = res.depth as i32;
        println!("md:{},depth:{},sum{}",node.state.calc_heuristic(),node.depth,node.state.calc_heuristic()+node.depth);
        if !closed.contains(&node) && (node.depth <= res.depth) {
            if node.state.puzzle.iter().enumerate().all(|(i,x)| *x == goal[i]) && node.depth < res.depth{
                res = node.clone();
                
            }
        closed.insert(node.clone());
        

        let rc = Rc::new(node.state);
        let mut populate = |d:Direction| { 
            match rc.try_branch(d) {
                Some(new_puzzle) => {
                    open.push(UberState{state:State{puzzle:new_puzzle, parent: Some(Rc::clone(&rc))},depth: node.depth+1});

                },
                None => return 
            }
            
        };
        if node.depth < bound || node.depth > res.depth {
        populate(Direction::Bottom);
        populate(Direction::Left);
        populate(Direction::Right);
        populate(Direction::Top);
        }
    }
    }
    if res.depth < 81 
    {
        return Some(res.state)
    }
    None
}


//__________________________________
impl Ord for UberState {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.state.calc_heuristic()+self.depth).cmp(&(other.state.calc_heuristic()+other.depth))
    }
}

impl PartialOrd for UberState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.state.calc_heuristic()+self.depth).partial_cmp(&(other.state.calc_heuristic()+other.depth))
    }
}

impl PartialEq for UberState {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.depth == other.depth
    }
}

impl Eq for UberState { }

