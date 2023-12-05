use lazy_static::lazy_static;

use std::ops::Range;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

trait Chunks2<'a, O>: Iterator {
    fn chunks_2<F: FnMut(<Self as Iterator>::Item, <Self as Iterator>::Item) -> O>(
        &'a mut self,
        f: F,
    ) -> Chunks2Info<'a, Self, O, F>;
}

struct Chunks2Info<'a, I: Iterator + ?Sized, O, F: FnMut(I::Item, I::Item) -> O> {
    i: &'a mut I,
    v: Option<I::Item>,
    f: F,
}

impl<'a, I: Iterator + ?Sized, O, F: FnMut(I::Item, I::Item) -> O> Iterator
    for Chunks2Info<'a, I, O, F>
{
    type Item = O;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.v.take() {
            if let Some(n) = self.i.next() {
                let r = Some((self.f)(v, n));
                self.v = self.i.next();
                r
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a, I: Iterator + ?Sized, O> Chunks2<'a, O> for I {
    fn chunks_2<F: FnMut(I::Item, I::Item) -> O>(
        &'a mut self,
        f: F,
    ) -> Chunks2Info<'a, Self, O, F> {
        let v = self.next();

        Chunks2Info { i: self, v, f }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Seeds(Range<u64>);

impl Seeds {
    fn intersection(&self, range: &Range<u64>) -> Option<Range<u64>> {
        let range = self.0.start.max(range.start)..self.0.end.min(range.end);
        if range.is_empty() {
            None
        } else {
            Some(range)
        }
    }
}

#[derive(Debug)]
struct MapEntry {
    source: Range<u64>,
    destination: Range<u64>,
}

impl MapEntry {
    fn get(&self, seed: u64) -> Option<u64> {
        if self.source.contains(&seed) {
            Some(self.destination.start + (seed - self.source.start))
        } else {
            None
        }
    }

    fn map(&self, seeds: &Seeds) -> (Option<Seeds>, Vec<Seeds>) {
        if let Some(intersection) = seeds.intersection(&self.source) {
            let mut unmapped = Vec::with_capacity(2);

            let low = seeds.0.start..intersection.start;
            if !low.is_empty() {
                unmapped.push(Seeds(low));
            }

            let high = intersection.end..seeds.0.end;
            if !high.is_empty() {
                unmapped.push(Seeds(high));
            }

            let mapped = (self.destination.start + (intersection.start - self.source.start))
                ..(self.destination.start + (intersection.end - self.source.start));

            (Some(Seeds(mapped)), unmapped)
        } else {
            (None, vec![seeds.clone()])
        }
    }
}

#[derive(Debug)]
struct Map(Vec<MapEntry>);

impl Map {
    fn get(&self, seed: u64) -> u64 {
        if let Some(result) = self.0.iter().find_map(|map_entry| map_entry.get(seed)) {
            result
        } else {
            seed
        }
    }

    fn map(&self, seeds: &Seeds) -> Vec<Seeds> {
        let mut result = vec![];
        let mut list = vec![seeds.clone()];
        for map_entry in self.0.iter() {
            let mut new_list = vec![];
            for seeds in &list {
                let (mapped, mut unmapped) = map_entry.map(seeds);
                if let Some(mapped) = mapped {
                    result.push(mapped);
                }
                new_list.append(&mut unmapped);
            }
            list = new_list;
        }

        result.append(&mut list);

        result
    }

    fn parse<'a>(parts: impl Iterator<Item = &'a str>) -> Vec<Self> {
        parts
            .map(|part| {
                Map(part
                    .lines()
                    .skip(1)
                    .map(|map_entry| {
                        let mut i = map_entry.split_whitespace();

                        let destination_range_start = i
                            .next()
                            .expect("cannot find destination range start")
                            .parse()
                            .expect("invalid destination start");
                        let source_range_start = i
                            .next()
                            .expect("cannot find source range start")
                            .parse()
                            .expect("invalid range start");
                        let range_length: u64 = i
                            .next()
                            .expect("cannot find range length")
                            .parse()
                            .expect("invalid range length");

                        MapEntry {
                            source: source_range_start..(source_range_start + range_length),
                            destination: destination_range_start
                                ..(destination_range_start + range_length),
                        }
                    })
                    .collect::<Vec<_>>())
            })
            .collect::<Vec<_>>()
    }
}

pub fn solve_1(input: &str) -> u64 {
    let mut parts = input.split("\n\n");

    let seeds = parts.next().expect("cannot find seeds")["seeds: ".len()..]
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().expect("invalid seed"));

    let maps = Map::parse(parts);

    seeds
        .map(|seed| maps.iter().fold(seed, |s, map| map.get(s)))
        .min()
        .expect("invalid input")
}

pub fn solve_2(input: &str) -> u64 {
    let mut parts = input.split("\n\n");

    let seeds_part = parts.next().expect("cannot find seeds");

    let maps = Map::parse(parts);

    seeds_part["seeds: ".len()..]
        .split_whitespace()
        .map(|value| value.parse::<u64>().expect("invalid value"))
        .chunks_2(|a, b| Seeds(a..(a + b)))
        .flat_map(|seeds| {
            maps.iter().fold(vec![seeds], |list, map| {
                list.into_iter()
                    .flat_map(|seeds| map.map(&seeds.clone()))
                    .collect::<Vec<_>>()
            })
        })
        .map(|Seeds(r)| r.start)
        .min()
        .unwrap()
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
        assert_eq!(solve_1(&EXAMPLE_1), 35);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 46);
    }

    #[test]
    fn test_map_entry_none() {
        let map_entry = MapEntry {
            source: 0..10,
            destination: 100..110,
        };

        assert_eq!(map_entry.map(&Seeds(10..20)), (None, vec![Seeds(10..20)]));
    }

    #[test]
    fn test_map_entry_full() {
        let map_entry = MapEntry {
            source: 0..10,
            destination: 100..110,
        };

        assert_eq!(map_entry.map(&Seeds(5..7)), (Some(Seeds(105..107)), vec![]));
    }

    #[test]
    fn test_map_entry_low() {
        let map_entry = MapEntry {
            source: 0..10,
            destination: 100..110,
        };

        assert_eq!(
            map_entry.map(&Seeds(8..12)),
            (Some(Seeds(108..110)), vec![Seeds(10..12)])
        );
    }

    #[test]
    fn test_map_entry_high() {
        let map_entry = MapEntry {
            source: 10..20,
            destination: 100..110,
        };

        assert_eq!(
            map_entry.map(&Seeds(8..12)),
            (Some(Seeds(100..102)), vec![Seeds(8..10)])
        );
    }

    #[test]
    fn test_map_entry_middle_inner() {
        let map_entry = MapEntry {
            source: 10..20,
            destination: 100..110,
        };

        assert_eq!(
            map_entry.map(&Seeds(0..30)),
            (Some(Seeds(100..110)), vec![Seeds(0..10), Seeds(20..30)])
        );
    }

    #[test]
    fn test_map() {
        let map = Map(vec![
            MapEntry {
                source: 10..20,
                destination: 100..110,
            },
            MapEntry {
                source: 20..30,
                destination: 50..60,
            },
        ]);

        assert_eq!(
            map.map(&Seeds(0..30)),
            vec![Seeds(100..110), Seeds(50..60), Seeds(0..10)]
        );
    }
}
