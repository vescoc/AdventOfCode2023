use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

fn extrapolate(mut i: impl Iterator<Item = i64>) -> i64 {
    let current = i.next().unwrap();

    let bottom = i
        .scan(current, |state, value| {
            let r = Some(*state - value);
            *state = value;
            r
        })
        .collect::<Vec<_>>();

    if bottom.iter().all(|&value| value == 0) {
        current
    } else {
        current + extrapolate(bottom.into_iter())
    }
}

fn parse_rev(line: &str) -> impl Iterator<Item = i64> + '_ {
    line.split_whitespace()
        .map(|n| n.parse::<i64>().expect("invalid number"))
        .rev()
}

fn parse_fwd(line: &str) -> impl Iterator<Item = i64> + '_ {
    line.split_whitespace()
        .map(|n| n.parse::<i64>().expect("invalid number"))
}

#[inline(always)]
fn solve<'a, F, I>(input: &'a str, parse: F) -> i64
where
    F: Fn(&'a str) -> I + Sync + Send + 'a,
    I: Iterator<Item = i64>,
{
    #[cfg(not(feature = "rayon"))]
    let r = input.lines().map(|line| extrapolate(parse(line))).sum();

    #[cfg(feature = "rayon")]
    let r = {
        use rayon::prelude::*;

        input
            .lines()
            .par_bridge()
            .map(|line| extrapolate(parse(line)))
            .sum()
    };

    r
}

pub fn solve_1(input: &str) -> i64 {
    solve(input, parse_rev)
}

pub fn solve_2(input: &str) -> i64 {
    solve(input, parse_fwd)
}

pub fn part_1() -> i64 {
    solve_1(&INPUT)
}

pub fn part_2() -> i64 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_1: &'static str = include_str!("../../example1");
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&EXAMPLE_1), 114);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 2);
    }

    #[test]
    fn example_1_1() {
        let example = parse_rev("0 3 6 9 12 15");

        assert_eq!(extrapolate(example), 18);
    }

    #[test]
    fn example_1_2() {
        let example = parse_rev("1 3 6 10 15 21");

        assert_eq!(extrapolate(example), 28);
    }

    #[test]
    fn example_1_3() {
        let example = parse_rev("10 13 16 21 30 45");

        assert_eq!(extrapolate(example), 68);
    }
}
