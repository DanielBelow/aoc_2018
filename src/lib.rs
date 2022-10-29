#![deny(unsafe_code)]
#![warn(nonstandard_style, rust_2018_idioms)]
#![allow(
    clippy::implicit_hasher,
    clippy::similar_names,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day14;
mod day16;
mod day18;
mod day19;
mod day21;
mod day23;
mod day25;

aoc_runner_derive::aoc_lib! { year = 2018 }
