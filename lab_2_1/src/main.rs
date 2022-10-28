#![feature(test)]
mod solvers;
pub mod benches;


fn main() {
    solvers::unwrap1(solvers::uber_solve(2,10000001));
}
