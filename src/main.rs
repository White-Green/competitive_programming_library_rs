#![allow(unused_imports)]
#![allow(dead_code)]

use itertools::Itertools;
use std::any::Any;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::ControlFlow;
use std::time::{Duration, Instant};
use std::{array, panic};

mod solver;

pub fn main() {
    return solver::main();
    // const CASES: u64 = 50;
    // const TIME_LIMIT: Duration = Duration::from_secs(2);
    // let mut scores = (0..CASES)
    //     .into_par_iter()
    //     .map(|seed| (seed, tools::gen(seed)))
    //     .filter_map(|(seed, input)| {
    //         let &tools::Input {
    //             N,
    //             M,
    //             K,
    //             ref ps,
    //             ref wall_v,
    //             ref wall_h,
    //         } = &input;
    //         assert_eq!((N, M, K), (solver::N, solver::M, solver::K));
    //         let solver_input = solver::InputObject {
    //             initial_robots: ps
    //                 .iter()
    //                 .map(|&(i, j)| (i as u8, j as u8))
    //                 .collect::<Vec<_>>()
    //                 .try_into()
    //                 .unwrap(),
    //             walls_v: wall_v
    //                 .iter()
    //                 .map(|line| line.clone().try_into().unwrap())
    //                 .collect::<Vec<_>>()
    //                 .try_into()
    //                 .unwrap(),
    //             walls_h: wall_h
    //                 .iter()
    //                 .map(|line| line.clone().try_into().unwrap())
    //                 .collect::<Vec<_>>()
    //                 .try_into()
    //                 .unwrap(),
    //         };
    //         let start = Instant::now();
    //         let solver_output = match panic::catch_unwind(|| solver::solve(solver_input)) {
    //             Ok(output) => output,
    //             Err(_) => {
    //                 eprintln!("[RE]  seed = {seed}");
    //                 return None;
    //             }
    //         };
    //         let elapsed = start.elapsed();
    //         if elapsed > TIME_LIMIT {
    //             eprintln!("[TLE] seed = {seed}, elapsed = {elapsed:?}");
    //         }
    //         let output = tools::Output {
    //             cs: solver_output
    //                 .buttons
    //                 .into_iter()
    //                 .map(|buttons| {
    //                     buttons
    //                         .into_iter()
    //                         .map(|b| match b {
    //                             Moving::Up => 0,
    //                             Moving::Down => 1,
    //                             Moving::Left => 2,
    //                             Moving::Right => 3,
    //                             Moving::Stop => 4,
    //                         })
    //                         .collect()
    //                 })
    //                 .collect(),
    //             actions: solver_output.controls.into_iter().map(Into::into).collect(),
    //         };
    //         let (score, err) = tools::compute_score(&input, &output);
    //         if !err.is_empty() {
    //             eprintln!("[WA] seed = {seed}, err = {err:?}");
    //             return None;
    //         }
    //         Some((score, seed))
    //     })
    //     .collect::<Vec<_>>();
    // scores.sort_unstable();
    // let min = scores[0];
    // let q1 = scores[scores.len() / 4];
    // let median = scores[scores.len() / 2];
    // let q3 = scores[scores.len() * 3 / 4];
    // let max = scores[scores.len() - 1];
    // let avg = scores.iter().map(|&(score, _)| score).sum::<i64>() as f64 / scores.len() as f64;
    // println!("Min          = {} [{}]", min.0, min.1);
    // println!("1st quartile = {} [{}]", q1.0, q1.1);
    // println!("Median       = {} [{}]", median.0, median.1);
    // println!("3rd quartile = {} [{}]", q3.0, q3.1);
    // println!("Max          = {} [{}]", max.0, max.1);
    // println!("Average      = {}", avg);
    // println!("Score of 50 Cases = {}", avg * 50.0);
}
