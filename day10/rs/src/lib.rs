use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

type DPoint = (isize, isize);

const NEIGHBORS: [(DPoint, [u8; 3], [u8; 3]); 4] = [
    ((-1, 0), *b"-J7", *b"-LF"),
    ((0, 1), *b"|7F", *b"|JL"),
    ((1, 0), *b"-LF", *b"-J7"),
    ((0, -1), *b"|JL", *b"|7F"),
];

fn solve(input: &str) -> (u32, Vec<&[u8]>, Vec<Vec<bool>>, u8) {
    let tiles = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let (nrows, ncols) = (tiles.len() as isize, tiles[0].len() as isize);

    let (x, y) = {
        let mut y = 0;
        loop {
            let row = tiles.get(y).expect("S not found");
            if let Some(x) = row.iter().position(|&c| c == b'S') {
                break (x as isize, y as isize);
            }
            y += 1;
        }
    };

    let mut visited = {
        let mut visited = Vec::with_capacity(nrows as usize);
        for _ in 0..nrows {
            visited.push(vec![false; ncols as usize]);
        }

        visited
    };

    visited[y as usize][x as usize] = true;

    let (mut current, s) = {
        let mut neighbors = NEIGHBORS.iter().filter_map(|((dx, dy), srcs, valid)| {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0
                && nx < ncols
                && ny >= 0
                && ny < nrows
                && valid.contains(&tiles[ny as usize][nx as usize])
            {
                Some(((nx, ny), srcs))
            } else {
                None
            }
        });

        let ((first, first_tile), (_, second_tile)) = (
            neighbors.next().expect("cannot find first path"),
            neighbors.next().expect("cannot find second path"),
        );

        let tile = {
            'outher: loop {
                for t in first_tile {
                    if second_tile.contains(t) {
                        break 'outher t;
                    }
                }

                panic!("cannot find S mapping");
            }
        };

        (first, tile)
    };

    let mut steps = 0;
    loop {
        steps += 1;
        let (x, y) = current;

        visited[y as usize][x as usize] = true;

        let pipe = tiles[y as usize][x as usize];

        if let Some(r) = NEIGHBORS
            .iter()
            .find_map(|((dx, dy), valid_src, valid_dest)| {
                if valid_src.contains(&pipe) {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx >= 0
                        && nx < ncols
                        && ny >= 0
                        && ny < nrows
                        && !visited[ny as usize][nx as usize]
                        && valid_dest.contains(&tiles[ny as usize][nx as usize])
                    {
                        Some((nx, ny))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        {
            current = r;
        } else {
            break;
        }
    }

    (steps, tiles, visited, *s)
}

pub fn solve_1(input: &str) -> u32 {
    let (steps, _, _, _) = solve(input);

    (steps + 1) / 2
}

pub fn solve_2(input: &str) -> u32 {
    let (_, tiles, visited, s) = solve(input);

    let mut area = 0;
    for (y, row) in visited.iter().enumerate() {
        let mut inside = false;
        for (x, &v) in row.iter().enumerate() {
            if v {
                match tiles[y][x] {
                    b'|' | b'L' | b'J' => inside = !inside,
                    b'S' => match s {
                        b'|' | b'L' | b'J' => inside = !inside,
                        _ => {}
                    },
                    _ => {}
                }
            } else if inside {
                area += 1;
            }
        }
    }

    area
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
        static ref EXAMPLE_2: &'static str = include_str!("../../example2");
        static ref EXAMPLE_3: &'static str = include_str!("../../example3");
        static ref EXAMPLE_4: &'static str = include_str!("../../example4");
        static ref EXAMPLE_5: &'static str = include_str!("../../example5");
    }

    #[test]
    fn same_results_1_1() {
        assert_eq!(solve_1(&EXAMPLE_1), 4);
    }

    #[test]
    fn same_results_1_2() {
        assert_eq!(solve_1(&EXAMPLE_2), 8);
    }

    #[test]
    fn same_results_2_3() {
        assert_eq!(solve_2(&EXAMPLE_3), 4);
    }

    #[test]
    fn same_results_2_4() {
        assert_eq!(solve_2(&EXAMPLE_4), 8);
    }

    #[test]
    fn same_results_2_5() {
        assert_eq!(solve_2(&EXAMPLE_5), 10);
    }

    #[test]
    fn same_results_2_1() {
        assert_eq!(solve_2(&EXAMPLE_1), 1);
    }
}
