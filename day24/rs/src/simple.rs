use super::{Hail3, INPUT};

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(input: &str) -> String {
    [
        "\n# run this on python3 with z3\n",
        "from z3 import *\n",
        "v0, v1, v2 = Ints('v0, v1, v2')\n",
        "p0, p1, p2 = Ints('p0, p1, p2')\n",
        "t0, t1, t2 = Ints('t0, t1, t2')\n",
        "answer = Int('answer')\n",
        "solve(\n",
    ]
    .into_iter()
    .map(str::to_string)
    .chain(
        input
            .lines()
            .map(|line| line.parse::<Hail3>().expect("invalid input"))
            .enumerate()
            .take(3)
            .flat_map(|(i, Hail3 { position, velocity })| {
                position
                    .iter()
                    .zip(velocity)
                    .enumerate()
                    .map(move |(j, (p, v))| format!("   {v} * t{i} + {p} == v{j} * t{i} + p{j},\n"))
                    .collect::<Vec<_>>()
            }),
    )
    .chain(["   answer == p0 + p1 + p2\n", ")\n"].map(str::to_string))
    .collect::<String>()
}

pub fn part_2() -> String {
    solve_2(&INPUT)
}
