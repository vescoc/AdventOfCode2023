use lazy_static::lazy_static;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as parse_char, multispace1, u32 as parse_u32},
    combinator::opt,
    error::Error,
    sequence::{delimited, preceded, tuple},
    IResult,
};

lazy_static! {
    static ref INPUT: &'static str = include_str!("../../input");
}

fn parse_game_id<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, u32> {
    delimited(tag("Game "), parse_u32, parse_char(':'))
}

fn parse_cube<'a>() -> impl FnMut(&'a str) -> IResult<&'a str, (u32, &'a str, &'a str, Option<char>)>
{
    preceded(
        multispace1,
        tuple((
            parse_u32::<_, Error<_>>,
            multispace1,
            alt((tag("blue"), tag("red"), tag("green"))),
            opt(alt((parse_char(','), parse_char(';')))),
        )),
    )
}

fn solve_1_nom(input: &str) -> u32 {
    let mut parse_game_id = parse_game_id();
    let mut parse_cube = parse_cube();

    input
        .lines()
        .filter_map(|line| {
            let (line, game_id) = parse_game_id(line).expect("invalid format");
            let mut line = line;
            while !line.is_empty() {
                let (r, (value, _, color, _)) = parse_cube(line).expect("invalid cube format");
                let valid = match color {
                    "red" => value <= 12,
                    "green" => value <= 13,
                    "blue" => value <= 14,
                    _ => unreachable!(),
                };
                if !valid {
                    return None;
                }

                line = r;
            }

            Some(game_id)
        })
        .sum()
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

pub fn part_1_nom() -> u32 {
    solve_1_nom(&INPUT)
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
