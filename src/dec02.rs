use regex::Regex;

const SAMPLE: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

// ugly! need to find a better parsing strategy.
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Default::default();
    let re = Regex::new(r"(\d+)(?:\s+|$)").unwrap();
    for line in input.lines() {
        let mut lc: Vec<i32> = Default::default();
        for cap in re.captures_iter(line) {
            if let Ok(n) = cap[1].parse::<i32>() {
                lc.push(n)
            }
        }
        if !lc.is_empty() {
            res.push(lc);
        }
    }
    res
}

fn is_good(sign: i32, d: i32) -> bool {
    d.abs() >= 1 && d.abs() <= 3 && d.signum() == sign
}

fn fwd_diff(report: &Vec<i32>) -> Vec<i32> {
    report
        .iter()
        .zip(&report[1..])
        .map(|(a, b)| b - a)
        .collect()
}

fn eval_report(report: &Vec<i32>) -> bool {
    let diffs = fwd_diff(report);
    // the sign (= direction) is the majority vote of the diffs signums.
    // don't worry if its 0, that just means the report is super bad.
    let sign = diffs.iter().map(|d| d.signum()).sum::<i32>().signum();
    diffs.iter().all(|&d| is_good(sign, d))
}

fn task1(input: &str) -> usize {
    let reports = parse_input(input);
    let mut res = 0;
    for report in reports {
        if eval_report(&report) {
            res += 1;
        }
    }
    res
}

fn eval_report_with_dampener(report: &Vec<i32>) -> bool {
    let diffs = fwd_diff(report);
    let sign = diffs.iter().map(|d| d.signum()).sum::<i32>().signum();
    let bad_idx: Vec<usize> = diffs
        .iter()
        .enumerate()
        .filter(|(idx, &d)| !is_good(sign, d))
        .map(|(idx, _)| idx)
        .collect();
    let issues = bad_idx.len();
    if issues == 0 {
        return true;
    }

    // the first bad diff value (sign and/or size)
    let idx = bad_idx[0];

    // we're calculating each diff on two levels in the report.
    // sometimes it's the first level that should be removed,
    // sometimes the second. i have no good formula for this
    // so we just try both alternatives.
    let mut new_left_report = report.clone();
    new_left_report.remove(idx);
    let mut new_right_report = report.clone();
    new_right_report.remove(idx + 1);

    if eval_report(&new_left_report) {
        return true;
    }

    if eval_report(&new_right_report) {
        return true;
    }
    false
}

fn task2(input: &str) -> usize {
    let reports = parse_input(input);
    let mut res = 0;
    for report in reports {
        if eval_report_with_dampener(&report) {
            res += 1;
        }
    }
    res
}

#[test]
fn test_day02_task1_sample() {
    let res = task1(&SAMPLE);
    assert_eq!(res, 2);
}

#[test]
fn test_day02_task1() {
    let res = task1(include_str!("../input/dec02.txt"));
    assert_eq!(res, 524);
}

#[test]
fn test_day02_task2_sample() {
    let res = task2(&SAMPLE);
    assert_eq!(res, 4);
}

#[test]
fn test_day02_task2() {
    let res = task2(include_str!("../input/dec02.txt"));
    assert_eq!(res, 569);
}
