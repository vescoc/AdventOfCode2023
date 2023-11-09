use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: &'static str = include_str!("../../input");
}

fn solve_1(_input: &str) -> usize {
    todo!()
}

fn solve_2(_input: &str) -> usize {
    todo!()
}

pub fn part_1() -> usize {
    solve_1(&INPUT)
}

pub fn part_2() -> usize {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref INPUT: &'static str = r#"XX"#;
    }

    #[test]
    fn same_results_1() {
        // assert_eq!(solve_1(&INPUT), 666);
        todo!();
    }

    #[test]
    fn same_results_2() {
        // assert_eq!(solve_2(&INPUT), 666);
        todo!();
    }
}
