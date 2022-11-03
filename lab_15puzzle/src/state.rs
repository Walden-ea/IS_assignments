use std::{rc::Rc, fmt::Display};
use ux::u4;

use crate::utils::input_to_u4_arr;

type Parent = Option<Rc<State>>;

pub struct State {
    pub puzzle: [u4;16], // '0' -- hole
    pub parent: Parent
}

impl State {
    pub fn new(input: &str) -> Self {
        
        assert!(input.len()==16);
        State{puzzle:input_to_u4_arr(input), parent:None}

    }

    pub fn try_slide_top(&mut self) -> bool {
        let hole = self.hole();
        if hole<4 {
            return false
        } 
        self.puzzle[hole] = self.puzzle[hole - 4];
        self.puzzle[hole - 4] = u4::new(0);
        true
    }

    pub fn try_slide_bottom(&mut self) -> bool {
        let hole = self.hole();
        if hole>11 {
            return false
        } 
        self.puzzle[hole] = self.puzzle[hole + 4];
        self.puzzle[hole + 4] = u4::new(0);
        true
    }
    pub fn try_slide_left(&mut self) -> bool {
        let hole = self.hole();
        if hole%4 == 0 {
            return false
        } 
        self.puzzle[hole] = self.puzzle[hole - 1];
        self.puzzle[hole - 1] = u4::new(0);
        true
    }

    pub fn try_slide_right(&mut self) -> bool {
        let hole = self.hole();
        if hole%4 == 3 {
            return false
        } 
        self.puzzle[hole] = self.puzzle[hole + 1];
        self.puzzle[hole + 1] = u4::new(0);
        true
    }
    
    pub fn is_solvable(&self) -> bool {
        let e = self.hole()/4 + 1;
        let sum = self.puzzle.iter().enumerate().fold(0, |acc, elem| {
            let count = self.puzzle.iter().enumerate().filter(|x|
                 x.1 < elem.1 && x.0 > elem.0 && *x.1!=u4::new(0) && *elem.1 != u4::new(0)  
            ).count();

            acc + count

        });
        (e+sum)%2==0
    }
    
    fn hole(&self) -> usize{
        self.puzzle.iter().position(|ch| *ch == u4::new(0)).expect("no hole!!!")
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vertical = "|".to_string();
    vertical.push_str(&" ".repeat(11));
    vertical = vertical.repeat(4);
    vertical.push_str("|\n");
    let mut vertical_with_line = "|".to_string();
    vertical_with_line.push_str(&"_".repeat(11));

    
    write!(f," ")?;
    writeln!(f,"{}","_".repeat(47))?;
    for i in 0..4 {
        write!(f,"{}",vertical.repeat(2))?;
    for j in 0..4 {

        if self.puzzle[i*4+j] == u4::new(0) {
            write!(f,"|{}"," ".repeat(11))?;
        }
        else if self.puzzle[i*4+j] < u4::new(10) {
        write!(f,"|{}{}{}"," ".repeat(5),self.puzzle[i*4+j]," ".repeat(5))?;
        } else {
            write!(f,"|{}{}{}"," ".repeat(5),self.puzzle[i*4+j]," ".repeat(4))?;
        }
    }
    writeln!(f,"|");
    write!(f,"{}",vertical);
    writeln!(f,"{}|",vertical_with_line.repeat(4));
    }
    write!(f,"")
    }
}