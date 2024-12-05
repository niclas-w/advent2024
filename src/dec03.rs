use regex::Regex;

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::u32 as u32_p;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let mut res: Vec<(u32, u32)> = Default::default();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(input) {
        let a = cap[1].parse::<u32>().unwrap();
        let b = cap[2].parse::<u32>().unwrap();
        res.push((a, b));
    }
    res
}

fn task1(input: &str) -> usize {
    let nums = parse_input(input);
    let mut res = 0;
    for (a, b) in nums {
        res += a as usize * b as usize;
    }
    res
}

const SAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[derive(Debug)]
enum Instr {
    Mul(u32, u32),
    Do,
    Dont,
    Nop, // you know, for noise
}

// trying nom
fn parse_code(input: &str) -> IResult<&str, Vec<Instr>> {
    let do_p = map(tag("do()"), |_| Instr::Do);
    let dont_p = map(tag("don't()"), |_| Instr::Dont);
    let mul_p = map(
        tuple((tag("mul("), u32_p, tag(","), u32_p, tag(")"))),
        |(_, a, _, b, _)| Instr::Mul(a, b),
    );
    let nop_p = map(take(1usize), |_| Instr::Nop);
    let instr_p = alt((do_p, dont_p, mul_p, nop_p));
    let mut all_p = many1(instr_p);
    all_p(input)
}

fn task2(input: &str) -> usize {
    let (_, code) = parse_code(input).unwrap();
    let mut res = 0;
    let mut enabled = true;
    for op in code {
        match op {
            Instr::Do => enabled = true,
            Instr::Dont => enabled = false,
            Instr::Mul(a, b) => {
                if enabled {
                    res += a as usize * b as usize
                }
            }
            Instr::Nop => {}
        }
    }
    res
}

#[test]
fn dec03_task1_sample() {
    let res = task1(&SAMPLE);
    assert_eq!(res, 161);
}

#[test]
fn dec03_task1() {
    let res = task1(include_str!("../input/dec03.txt"));
    assert_eq!(res, 178886550);
}

#[test]
fn dec03_task2_sample() {
    let res = task2(&SAMPLE2);
    assert_eq!(res, 48);
}

#[test]
fn dec03_task2() {
    let res = task2(include_str!("../input/dec03.txt"));
    assert_eq!(res, 87163705);
}
