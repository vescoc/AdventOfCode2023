use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

fn find_min(time: u64, distance: u64) -> u64 {
    let mut upper_t = time / 2;
    assert!(upper_t * (time - upper_t) > distance);

    let mut lower_t = {
        let mut pivot = upper_t / 2;
        loop {
            if pivot * (time - pivot) < distance {
                break pivot;
            }

            pivot /= 2;
        }
    };
    assert!(lower_t < upper_t);

    loop {
        if upper_t - lower_t == 1 {
            break upper_t;
        }

        let pivot = (upper_t - lower_t) / 2 + lower_t;
        if pivot * (time - pivot) > distance {
            upper_t = pivot;
        } else {
            lower_t = pivot;
        }
    }
}

fn find_max(time: u64, distance: u64) -> u64 {
    let mut lower_t = time / 2;
    assert!(lower_t * (time - lower_t) > distance);

    let mut upper_t = {
        let mut pivot = (time - lower_t) / 2 + lower_t;
        loop {
            if pivot * (time - pivot) < distance {
                break pivot;
            }

            pivot = (time - pivot) / 2 + pivot;
        }
    };
    assert!(lower_t < upper_t);

    loop {
        if upper_t - lower_t == 1 {
            break lower_t;
        }

        let pivot = (upper_t - lower_t) / 2 + lower_t;
        if pivot * (time - pivot) > distance {
            lower_t = pivot;
        } else {
            upper_t = pivot;
        }
    }
}

#[inline(always)]
fn solve(time: u64, distance: u64) -> u64 {
    let min = find_min(time, distance);
    let max = find_max(time, distance);

    max - min + 1
}

pub fn solve_1(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut parse = move |msg| {
        lines.next().expect(msg)[10..]
            .split_whitespace()
            .map(|number| number.parse::<u64>().expect("invalid number"))
    };

    let time = parse("cannot find time part");
    let distance = parse("cannot find distance part");

    time.zip(distance).map(|(t, d)| solve(t, d)).product()
}

pub fn solve_2(input: &str) -> u64 {
    let mut lines = input.lines();

    let mut parse = move |msg| {
        lines.next().expect(msg)[10..]
            .split_whitespace()
            .collect::<String>()
            .parse::<u64>()
            .expect("invalid number")
    };

    let time = parse("cannot find time part");
    let distance = parse("cannot find distance part");

    solve(time, distance)
}

pub fn part_1() -> u64 {
    solve_1(&INPUT)
}

pub fn part_2() -> u64 {
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
        assert_eq!(solve_1(&EXAMPLE_1), 288);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 71503);
    }

    #[test]
    fn test_find_min_7_9() {
        assert_eq!(find_min(7, 9), 2);
    }

    #[test]
    fn test_find_max_7_9() {
        assert_eq!(find_max(7, 9), 5);
    }

    #[test]
    fn test_find_min_15_40() {
        assert_eq!(find_min(15, 40), 4);
    }

    #[test]
    fn test_find_max_15_40() {
        assert_eq!(find_max(15, 40), 11);
    }

    #[test]
    fn test_find_min_30_200() {
        assert_eq!(find_min(30, 200), 11);
    }

    #[test]
    fn test_find_max_30_200() {
        assert_eq!(find_max(30, 200), 19);
    }
}
