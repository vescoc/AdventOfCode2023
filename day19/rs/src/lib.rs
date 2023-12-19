#![deny(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use lazy_static::lazy_static;

use std::{
    collections::{HashMap, HashSet},
    ops,
    str::FromStr,
};

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum PartCategory {
    X,
    M,
    A,
    S,
}

impl FromStr for PartCategory {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err("invalid part"),
        }
    }
}

impl TryFrom<char> for PartCategory {
    type Error = &'static str;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            'x' => Ok(Self::X),
            'm' => Ok(Self::M),
            'a' => Ok(Self::A),
            's' => Ok(Self::S),
            _ => Err("invalid part"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Part<T>([T; 4]);

impl<T> Part<T> {
    fn new(x: T, m: T, a: T, s: T) -> Self {
        Self([x, m, a, s])
    }
}

impl Part<u64> {
    fn value(self) -> u64 {
        self.0.iter().sum()
    }
}

impl Part<ops::RangeInclusive<u64>> {
    fn value(&self) -> u64 {
        self.0.iter().map(|r| r.end() - r.start() + 1).product()
    }

    fn is_empty(&self) -> bool {
        self.0.iter().any(ops::RangeInclusive::is_empty)
    }

    fn intersect(&self, other: &Self) -> bool {
        [
            PartCategory::X,
            PartCategory::M,
            PartCategory::A,
            PartCategory::S,
        ]
        .iter()
        .all(|&category| {
            let a = &self[category];
            let b = &other[category];

            a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
        })
    }

    #[allow(clippy::range_minus_one)]
    fn fragments(&self, other: &Self) -> HashSet<Part<ops::RangeInclusive<u64>>> {
        let mut cuboids = HashSet::new();
        cuboids.insert(self.clone());

        for category in [
            PartCategory::X,
            PartCategory::M,
            PartCategory::A,
            PartCategory::S,
        ] {
            let mut new_cuboids = HashSet::new();

            let start = *other[category].start();
            let end = *other[category].end();

            for cuboid in cuboids.drain() {
                let range = &cuboid[category];

                if range.contains(&start) {
                    let r = *range.start()..=start - 1;
                    if !r.is_empty() {
                        let mut cuboid = cuboid.clone();
                        cuboid[category] = r;
                        new_cuboids.insert(cuboid);
                    }

                    if range.contains(&end) {
                        let r = end + 1..=*range.end();
                        if !r.is_empty() {
                            let mut cuboid = cuboid.clone();
                            cuboid[category] = r;
                            new_cuboids.insert(cuboid);
                        }

                        let r = start..=end;
                        if !r.is_empty() {
                            let mut cuboid = cuboid.clone();
                            cuboid[category] = r;
                            new_cuboids.insert(cuboid);
                        }
                    } else {
                        let r = start..=*range.end();
                        if !r.is_empty() {
                            let mut cuboid = cuboid.clone();
                            cuboid[category] = r;
                            new_cuboids.insert(cuboid);
                        }
                    }
                } else if range.contains(&end) {
                    let r = end + 1..=*range.end();
                    if !r.is_empty() {
                        let mut cuboid = cuboid.clone();
                        cuboid[category] = r;
                        new_cuboids.insert(cuboid);
                    }

                    let r = *range.start()..=end;
                    if !r.is_empty() {
                        let mut cuboid = cuboid.clone();
                        cuboid[category] = r;
                        new_cuboids.insert(cuboid);
                    }
                } else {
                    new_cuboids.insert(cuboid);
                }
            }

            cuboids = new_cuboids;
        }

        cuboids
    }

    fn difference(&self, other: &Self) -> HashSet<Part<ops::RangeInclusive<u64>>> {
        self.fragments(other)
            .difference(&other.fragments(self))
            .cloned()
            .collect::<HashSet<_>>()
    }
}

impl<T> ops::Index<PartCategory> for Part<T> {
    type Output = T;

    fn index(&self, part_category: PartCategory) -> &Self::Output {
        match part_category {
            PartCategory::X => &self.0[0],
            PartCategory::M => &self.0[1],
            PartCategory::A => &self.0[2],
            PartCategory::S => &self.0[3],
        }
    }
}

impl<T> ops::IndexMut<PartCategory> for Part<T> {
    fn index_mut(&mut self, part_category: PartCategory) -> &mut Self::Output {
        match part_category {
            PartCategory::X => &mut self.0[0],
            PartCategory::M => &mut self.0[1],
            PartCategory::A => &mut self.0[2],
            PartCategory::S => &mut self.0[3],
        }
    }
}

impl FromStr for Part<u64> {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, part) = input.split_once('{').ok_or("invalid part, cannot find {")?;
        let (part, _) = part.split_once('}').ok_or("invalid part, cannot find }")?;

        let mut values = [0; 4];
        for (i, part) in part.split(',').enumerate() {
            let (_, value) = part.split_once('=').ok_or("invalid part part")?;
            values[i] = value.parse().map_err(|_| "invalid value")?;
        }

        Ok(Self(values))
    }
}

struct Workflows<'a>(HashMap<&'a str, Vec<Rule<'a>>>);

impl<'a> Workflows<'a> {
    fn parse(input: &'a str) -> Result<Self, &'static str> {
        Ok(Workflows(
            input
                .lines()
                .map(|line| {
                    let (workflow_id, remainder) =
                        line.split_once('{').ok_or("cannot find workflow id")?;
                    let (rules, _) = remainder.split_once('}').ok_or("cannot find rules")?;
                    let rules = rules
                        .split(',')
                        .map(Rule::parse)
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok((workflow_id, rules))
                })
                .collect::<Result<HashMap<&str, Vec<Rule>>, _>>()?,
        ))
    }

    #[allow(clippy::never_loop)]
    fn check(&self, part: &Part<u64>) -> Result<WorkflowsApplyResult, &'static str> {
        let mut workflow = "in";
        while let Some(rules) = self.0.get(workflow) {
            'outher: loop {
                for rule in rules {
                    match rule.apply(part) {
                        Some(Action::Accept) => return Ok(WorkflowsApplyResult::Accept),
                        Some(Action::Reject) => return Ok(WorkflowsApplyResult::Reject),
                        Some(Action::JumpToWorkflow(new_workflow)) => {
                            workflow = new_workflow;
                            break 'outher;
                        }
                        _ => {}
                    }
                }

                return Err("invalid workflow entry");
            }
        }

        Err("cannot find workflow")
    }

    fn add_cuboid(
        cuboids: &mut HashSet<Part<ops::RangeInclusive<u64>>>,
        cuboid: Part<ops::RangeInclusive<u64>>,
    ) {
        let mut r = HashSet::new();

        {
            let drain = cuboids.drain();
            for current in drain {
                if current.intersect(&cuboid) {
                    for cuboid in current.difference(&cuboid) {
                        r.insert(cuboid);
                    }
                } else {
                    r.insert(current);
                }
            }
            r.insert(cuboid);
        }

        *cuboids = r;
    }

    fn combinations(&self, part: Part<ops::RangeInclusive<u64>>) -> u64 {
        let mut accepted = HashSet::new();

        let mut queue = vec![("in", part)];
        while let Some((workflow, mut part)) = queue.pop() {
            if let Some(rules) = self.0.get(workflow) {
                for rule in rules {
                    match rule.apply_range(&part) {
                        RuleApplyResult::Accept => {
                            Self::add_cuboid(&mut accepted, part);
                            break;
                        }
                        RuleApplyResult::Reject => {
                            // Self::add_cuboid(&mut rejected, part);
                            break;
                        }
                        RuleApplyResult::JumpToWorkflow(workflow) => {
                            queue.push((workflow, part));
                            break;
                        }
                        RuleApplyResult::Split(action, ok_part, ko_part) => {
                            match action {
                                Action::Accept => {
                                    Self::add_cuboid(&mut accepted, ok_part);
                                }
                                Action::Reject => {
                                    // Self::add_cuboid(&mut rejected, ok_part);
                                }
                                Action::JumpToWorkflow(workflow) => {
                                    if !ok_part.is_empty() {
                                        queue.push((workflow, ok_part));
                                    }
                                }
                            }

                            if ko_part.is_empty() {
                                break;
                            }

                            part = ko_part;
                        }
                    }
                }
            }
        }

        accepted
            .iter()
            .map(Part::<ops::RangeInclusive<u64>>::value)
            .sum::<u64>()
    }
}

#[derive(Debug, Copy, Clone)]
enum Action<'a> {
    Accept,
    Reject,
    JumpToWorkflow(&'a str),
}

enum Rule<'a> {
    Immediate(Action<'a>),
    PartLessThan(PartCategory, u64, Action<'a>),
    PartGreaterThan(PartCategory, u64, Action<'a>),
}

impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> Result<Self, &'static str> {
        match input {
            "A" => Ok(Rule::Immediate(Action::Accept)),
            "R" => Ok(Rule::Immediate(Action::Reject)),
            _ => {
                if let Some((pre, action)) = input.split_once(':') {
                    let mut chars = pre.chars();
                    let part_category = chars
                        .next()
                        .ok_or("cannot find part category")?
                        .try_into()?;
                    let op = chars.next().ok_or("cannot find operation")?;
                    let value = chars.as_str().parse().map_err(|_| "invalid value")?;
                    let action = match action {
                        "A" => Action::Accept,
                        "R" => Action::Reject,
                        workflow => Action::JumpToWorkflow(workflow),
                    };
                    match op {
                        '>' => Ok(Rule::PartGreaterThan(part_category, value, action)),
                        '<' => Ok(Rule::PartLessThan(part_category, value, action)),
                        _ => Err("invalid operation"),
                    }
                } else {
                    Ok(Rule::Immediate(Action::JumpToWorkflow(input)))
                }
            }
        }
    }

    fn apply(&self, part: &Part<u64>) -> Option<Action> {
        match self {
            Self::Immediate(action) => Some(*action),
            Self::PartLessThan(part_category, value, action) => {
                if part[*part_category] < *value {
                    Some(*action)
                } else {
                    None
                }
            }
            Self::PartGreaterThan(part_category, value, action) => {
                if part[*part_category] > *value {
                    Some(*action)
                } else {
                    None
                }
            }
        }
    }

    #[allow(clippy::range_minus_one)]
    fn apply_range(&self, part: &Part<ops::RangeInclusive<u64>>) -> RuleApplyResult {
        match self {
            Self::Immediate(Action::Accept) => RuleApplyResult::Accept,
            Self::Immediate(Action::Reject) => RuleApplyResult::Reject,
            Self::Immediate(Action::JumpToWorkflow(workflow)) => {
                RuleApplyResult::JumpToWorkflow(workflow)
            }
            Self::PartLessThan(part_category, value, action) => {
                let range = &part[*part_category];

                let range_ok = *range.start()..=*value - 1;
                let range_ko = *value..=*range.end();

                let mut part_ok = part.clone();
                part_ok[*part_category] = range_ok;

                let mut part_ko = part.clone();
                part_ko[*part_category] = range_ko;

                RuleApplyResult::Split(*action, part_ok, part_ko)
            }
            Self::PartGreaterThan(part_category, value, action) => {
                let range = &part[*part_category];

                let range_ko = *range.start()..=*value;
                let range_ok = *value + 1..=*range.end();

                let mut part_ok = part.clone();
                part_ok[*part_category] = range_ok;

                let mut part_ko = part.clone();
                part_ko[*part_category] = range_ko;

                RuleApplyResult::Split(*action, part_ok, part_ko)
            }
        }
    }
}

enum WorkflowsApplyResult {
    Accept,
    Reject,
}

enum RuleApplyResult<'a> {
    Accept,
    Reject,
    JumpToWorkflow(&'a str),
    Split(
        Action<'a>,
        Part<ops::RangeInclusive<u64>>,
        Part<ops::RangeInclusive<u64>>,
    ),
}

/// Solve part 1
///
/// # Panics
/// Panic if invalid input
pub fn solve_1(input: &str) -> u64 {
    let (rules, parts) = input.split_once("\n\n").expect("invalid input");

    let workflows = Workflows::parse(rules).expect("invalid worfklow");

    parts
        .lines()
        .map(|line| {
            let part = line.parse().expect("invalid part");
            match workflows.check(&part).expect("invalid rule") {
                WorkflowsApplyResult::Accept => part.value(),
                WorkflowsApplyResult::Reject => 0,
            }
        })
        .sum()
}

/// Solve part 2
///
/// # Panics
/// Panic if invalid input
pub fn solve_2(input: &str) -> u64 {
    let (rules, _) = input.split_once("\n\n").expect("invalid input");

    let workflows = Workflows::parse(rules).expect("invalid rules");

    workflows.combinations(Part::new(1..=4000, 1..=4000, 1..=4000, 1..=4000))
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
        assert_eq!(solve_1(&EXAMPLE_1), 19114);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 167409079868000);
    }

    #[test]
    fn cuboid_difference() {
        let cuboid1 = Part::new(1..=3, 1..=3, 1..=3, 1..=3);
        let cuboid2 = Part::new(2..=2, 2..=2, 2..=2, 2..=2);

        assert_eq!(cuboid1.difference(&cuboid2).len(), 3 * 3 * 3 * 3 - 1);
    }
}
