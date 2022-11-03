use std::{rc::Rc, fmt::{Display, Debug}};
// use ux::u4;

use crate::utils::str_to_u8_arr;

type Parent = Option<Rc<State>>;

pub enum Direction {
    Top,
    Right,
    Left,
    Bottom
}

pub struct State {
    pub puzzle: [u8;16], // '0' -- hole
    pub parent: Parent
}

impl State {

    fn hole(&self) -> usize{
        self.puzzle.iter().position(|ch| *ch == 0).expect("no hole!!!")
    }
    pub fn new(input: &str) -> Self {
        
        assert!(input.len()==16);
        State{puzzle:str_to_u8_arr(input), parent:None}

    }

    pub fn is_solvable(&self) -> bool {
        let e = self.hole()/4 + 1;
        let sum = self.puzzle.iter().enumerate().fold(0, |acc, elem| {
            let count = self.puzzle.iter().enumerate().filter(|x|
                 x.1 < elem.1 && x.0 > elem.0 && *x.1!=0 && *elem.1 != 0 
            ).count();

            acc + count

        });
        (e+sum)%2==0
    }
    pub fn try_slide(&self, comp: fn(usize) -> bool, f:fn(usize) -> usize) -> Option<[u8;16]> {
        let hole = self.hole();
        if comp(hole){
            return None
        }
        let mut puzzle = self.puzzle.clone();
        puzzle[hole] = puzzle[f(hole)];
        puzzle[f(hole)] = 0;
        Some(puzzle)
    }

    pub fn try_branch(&self,d: Direction) -> Option<[u8;16]>{
        let hole = self.hole();
        match d {
            Direction::Top => self.try_slide(|x| x<4, |x| x - 4),
            Direction::Right => self.try_slide(|x| x%4 == 3,|x| x + 1),
            Direction::Bottom => self.try_slide(|x| x>11,|x| x + 4),
            Direction::Left => self.try_slide(|x| x%4 == 0, |x| x -1)
        }
    }
    pub fn print_path(&self) {
            let mut count = 0;
            print!("{:?} ",self);
            let mut root = &self.parent;
            while let Some(node) = &root{
                count+=1;
                print!("\n->\n\n{:?} ",node);
                root = &node.parent;
            }
            println!("\ncount: {}",count);
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

        if self.puzzle[i*4+j] == 0 {
            write!(f,"|{}"," ".repeat(11))?;
        }
        else if self.puzzle[i*4+j] < 10 {
        write!(f,"|{}{}{}"," ".repeat(5),self.puzzle[i*4+j]," ".repeat(5))?;
        } else {
            write!(f,"|{}{}{}"," ".repeat(5),self.puzzle[i*4+j]," ".repeat(4))?;
        }
    }
    writeln!(f,"|")?;
    write!(f,"{}",vertical)?;
    writeln!(f,"{}|",vertical_with_line.repeat(4))?;
    }
    write!(f,"")
    //write!(f,"{esc}c", esc = 27 as char)
    }


}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4{
            for j in 0..4 {
                if self.puzzle[i*4+j] < 0{
                    write!(f,"{}  ",self.puzzle[i*4+j])?;
                }
                else {
                    write!(f,"{} ",self.puzzle[i*4+j])?;
            }
            }
            write!(f,"\n")?;
        }
        write!(f,"")
    }
}