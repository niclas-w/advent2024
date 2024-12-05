use std::collections::HashMap;

const SAMPLE: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

#[derive(Debug)]
struct Board {
    cols: usize,
    rows: usize,
    data: Vec<char>,
}

impl Board {
    fn parse(input: &str) -> Self {
        let line_len = input.lines().nth(1).unwrap().len();
        let arr: Vec<char> = input
            .lines()
            .filter(|l| !l.is_empty())
            .flat_map(|s| s.chars())
            .collect();

        Board {
            cols: line_len,
            rows: arr.len() / line_len,
            data: arr,
        }
    }

    fn check(&self, x: i32, y: i32, c: char) -> bool {
        self.data[x as usize + y as usize * 10] == c
    }

    fn check_xmas(&self, x0: usize, y0: usize, xk: i32, yk: i32) -> bool {
        for n in 0..4 {
            let x = x0 as i32 + n as i32 * xk;
            let y = y0 as i32 + n as i32 * yk;
            if x < 0 || x >= self.cols as i32 || y < 0 || y >= self.rows as i32 {
                return false;
            };
            if !self.check(x, y, XMAS[n]) {
                return false;
            };
        }
        true
    }
}

fn task1(input: &str) -> usize {
    let mut res = 0;
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let board = Board::parse(input);
    for x in 0..board.cols {
        for y in 0..board.rows {
            for dir in dirs {
                if board.check_xmas(x, y, dir.0, dir.1) {
                    res += 1
                };
            }
        }
    }
    res
}

fn task2(input: &str) -> usize {
    2
}

#[test]
fn dec04_parse_input() {
    let b = Board::parse(&SAMPLE);
    assert_eq!(b.cols, 10);
    assert_eq!(b.rows, 10);

    let b = Board::parse(include_str!("../input/dec04.txt"));
    assert_eq!(b.cols, 140);
    assert_eq!(b.rows, 140);
}

// ....XXMAS.
// .SAMXMS...
// ...S..A...
// ..A.A.MS.X
// XMASAMX.MM
// X.....XA.A
// S.S.S.S.SS
// .A.A.A.A.A
// ..M.M.M.MM
// .X.X.XMASX
#[test]
fn dec04_board_check_xmas() {
    let b = Board::parse(&SAMPLE);
    assert!(b.check_xmas(5, 0, 1, 0));
    assert!(b.check_xmas(4, 0, 1, 1));
}

#[test]
fn dec04_task1_sample() {
    let res = task1(&SAMPLE);
    assert_eq!(res, 18);
}

#[test]
fn dec04_task1() {
    let res = task1(include_str!("../input/dec04.txt"));
    assert_eq!(res, 835);
}

#[test]
fn dec04_task2_sample() {
    let res = task2(&SAMPLE);
    assert_eq!(res, 0);
}

#[test]
fn dec04_task2() {
    let res = task2(include_str!("../input/dec04.txt"));
    assert_eq!(res, 0);
}
