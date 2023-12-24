use super::{Hail3, INPUT};

use z3::{Config, Context, Solver, SatResult};
use z3::ast::{Int, Ast};

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(input: &str) -> i64 {
    let cfg = Config::default();
    let ctx = Context::new(&cfg);

    let (v0, v1, v2) = (Int::new_const(&ctx, "v0"), Int::new_const(&ctx, "v1"), Int::new_const(&ctx, "v2"));
    let (p0, p1, p2) = (Int::new_const(&ctx, "p0"), Int::new_const(&ctx, "p1"), Int::new_const(&ctx, "p2"));
    let (t0, t1, t2) = (Int::new_const(&ctx, "t0"), Int::new_const(&ctx, "t1"), Int::new_const(&ctx, "t2"));
    
    let solver = Solver::new(&ctx);

    input
        .lines()
        .map(|line| line.parse::<Hail3>().expect("invalid input"))
        .take(3)
        .zip([&t0, &t1, &t2])
        .for_each(|(Hail3 { position, velocity }, t)| {
            position
                .iter()
                .zip(velocity)
                .zip([&v0, &v1, &v2])
                .zip([&p0, &p1, &p2])
                .for_each(|(((p, v), tv), tp)| {
                    solver.assert(&(t * v as i64 + *p as i64)._eq(&(tv * t + tp)));
                });
        });

    if solver.check() != SatResult::Sat {
        panic!("invalid solution")
    }

    let model = solver.get_model().expect("invalid model");

    let p0 = model.eval(&p0, true).expect("cannot get p0").as_i64().expect("cannot convert p0 to i64");
    let p1 = model.eval(&p1, true).expect("cannot get p1").as_i64().expect("cannot convert p1 to i64");
    let p2 = model.eval(&p2, true).expect("cannot get p2").as_i64().expect("cannot convert p2 to i64");

    p0 + p1 + p2
}

pub fn part_2() -> i64 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;
    
    lazy_static! {
        static ref EXAMPLE_1: &'static str = include_str!("../../example1");
    }

    #[test]
    fn same_result_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 47);
    }
}
