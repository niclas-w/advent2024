use std::collections::HashMap;

const SAMPLE: &str = "
3   4
4   3
2   5
1   3
3   9
3   3
";

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut ass = Vec::new();
    let mut bss = Vec::new();
    for line in input.lines() {
        let p = sscanf::sscanf!(line, "{}   {}", usize, usize);
        if let Ok((a, b)) = p {
            ass.push(a);
            bss.push(b);
        }
    }
    (ass, bss)
}

fn task1(input: &str) -> usize {
    let (mut ass, mut bss) = parse_input(input);
    ass.sort();
    bss.sort();
    let mut res: usize = 0;
    for (&a, b) in ass.iter().zip(bss) {
        let diff: usize = a.abs_diff(b);
        res += diff as usize;
    }
    res
}

fn task2(input: &str) -> usize {
    let (ass, bss) = parse_input(input);

    // count occurances in bss, build a map
    let mut counts: HashMap<usize, usize> = Default::default();
    for b in bss {
        let count: &usize = counts.get(&b).unwrap_or(&0);
        counts.insert(b, *count + 1);
    }
    let mut res: usize = 0;

    // calculate score
    for a in ass {
        let count: &usize = counts.get(&a).unwrap_or(&0);
        res += a * count;
    }
    res
}

#[test]
fn dec01_task1_sample() {
    let res = task1(&SAMPLE);
    assert_eq!(res, 11);
}

#[test]
fn dec01_task1() {
    let res = task1(include_str!("../input/dec01.txt"));
    assert_eq!(res, 1660292);
}

#[test]
fn dec01_task2_sample() {
    let res = task2(&SAMPLE);
    assert_eq!(res, 31);
}

#[test]
fn dec01_task2() {
    let res = task2(include_str!("../input/dec01.txt"));
    assert_eq!(res, 22776016);
}
