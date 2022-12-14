// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 8.  
//! Bart Massey 2022

use aoc::*;

type Heights = Vec<Vec<i8>>;
type Visibilities = Vec<Vec<bool>>;

fn parse_input() -> (usize, usize, Heights) {
    let result: Heights = input_lines()
        .map(|l| {
            l
                .chars()
                .map(|c| {
                    c
                        .to_digit(10)
                        .unwrap()
                        .try_into()
                        .unwrap()
                })
                .collect()
        })
        .collect();
    let ncols = result[1].len();
    for r in &result[1..] {
        assert_eq!(ncols, r.len());
    }
    (result.len(), ncols, result)
}

fn find_visible((nrows, ncols, height): (usize, usize, Heights)) -> Visibilities {
    use std::iter::repeat as rep;

    let mut visible = vec![vec![false; ncols]; nrows];

    fn mark_visible<R, C>(
        height: &Heights,
        visible: &mut Visibilities,
        rs: R,
        cs: C,
    )
        where R: Iterator<Item = usize>, C: Iterator<Item = usize>
    {
        let mut max = -1;
        for (r, c) in rs.zip(cs) {
            let h = height[r][c];
            if h > max {
                visible[r][c] = true;
                max = h;
            }
        }
    }

    for c in 0..ncols {
        mark_visible(&height, &mut visible, 0..nrows, rep(c));
        mark_visible(&height, &mut visible, (0..nrows).rev(), rep(c));
    }
    for r in 0..nrows {
        mark_visible(&height, &mut visible, rep(r), 0..ncols);
        mark_visible(&height, &mut visible, rep(r), (0..ncols).rev());
    }

    visible
}

fn main() {
    let trees = parse_input();
    match get_part() {
        Part1 => {
            let visible = find_visible(trees);
            let nvisible: usize = visible
                .into_iter().map(|r| {
                    r.into_iter().map(|v| v as usize).sum::<usize>()
                })
                .sum();
            println!("{nvisible}");
        }
        Part2 => todo!(),
    }
}
