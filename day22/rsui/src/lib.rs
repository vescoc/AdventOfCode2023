#![deny(clippy::pedantic)]

use std::fmt::Debug;

use rsui::Function;

pub struct Part1;

impl Function for Part1 {
    fn f(input: &str) -> impl Debug {
        rs::solve_1(input)
    }
}

pub struct Part2;

impl Function for Part2 {
    fn f(input: &str) -> impl Debug {
        rs::solve_2(input)
    }
}
