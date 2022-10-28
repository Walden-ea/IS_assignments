#[cfg(test)]

mod benches {
    use crate::solvers::{simple_solve, backwards_solve, uber_solve};

    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn simple_solve_bench(bench: &mut Bencher) {
        bench.iter(|| {
            for _ in 0..1000 {
                simple_solve(2, 1000);
            }
        })
    }
    #[bench]
    fn backwards_solve_bench(bench: &mut Bencher) {
        bench.iter(|| {
            for _ in 0..1000 {
                backwards_solve(2, 1000);
            }
        })
    }
    #[bench]
    fn simple_big_bench(bench: &mut Bencher) {
        bench.iter(|| {
                simple_solve(2, 10000001);
        })
    }
    #[bench]
    fn bidirectional_big_bench(bench: &mut Bencher) {
        bench.iter(|| {
            uber_solve(2, 10000001);
        })
    }
}