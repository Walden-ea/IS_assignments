use crate::state::Direction;

trait Puzzle {
    fn try_slide(&self, d:Direction) -> Self where
    Self: Sized;

    fn is_correct(&self); // 
}