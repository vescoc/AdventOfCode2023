use lazy_static::lazy_static;

lazy_static! {
    static ref INPUT: &'static str = include_str!("../../input");
}

fn solve_1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let (game_part, sets_part) = line.split_once(':').expect("invalid game");
            let game_id = game_part["Game ".len()..]
                .parse::<u32>()
                .expect("invalid game id");

            for set in sets_part.split(';') {
                for cube in set.split(',') {
                    let (count, color) = cube[1..].split_once(' ').expect("invalid cube part");
                    let count = count.parse::<u32>().expect("invalid number of cubes");
                    let valid = match color {
                        "red" => count <= 12,
                        "green" => count <= 13,
                        "blue" => count <= 14,
                        _ => panic!("invalid color"),
                    };
                    if !valid {
                        return None;
                    }
                }
            }

            Some(game_id)
        })
        .sum()
}

fn solve_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, sets_part) = line.split_once(':').expect("invalid game");

            let (mut red, mut green, mut blue) = (0, 0, 0);
            for set in sets_part.split(';') {
                for cube in set.split(',') {
                    let (count, color) = cube[1..].split_once(' ').expect("invalid cube part");
                    let count = count.parse::<u32>().expect("invalid number of cubes");
                    match color {
                        "red" => red = red.max(count),
                        "green" => green = green.max(count),
                        "blue" => blue = blue.max(count),
                        _ => panic!("invalid color"),
                    }
                }
            }

            red * green * blue
        })
        .sum()
}

pub fn part_1() -> u32 {
    solve_1(&INPUT)
}

pub fn part_2() -> u32 {
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
        assert_eq!(solve_1(&EXAMPLE_1), 8);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 2286);
    }
}
