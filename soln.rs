// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 8.  
//! Bart Massey 2022

use aoc::*;

type Heights = Vec<Vec<i8>>;
type Visibilities = Vec<Vec<bool>>;
type ViewDists = Vec<Vec<usize>>;

fn parse_input() -> (usize, usize, Heights) {
    let result: Heights = input_lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();
    let ncols = result[1].len();
    for r in &result[1..] {
        assert_eq!(ncols, r.len());
    }
    (result.len(), ncols, result)
}

fn ix<'a, I1, I2>(
    i1: I1,
    i2: I2,
) -> Box<dyn Iterator<Item = (usize, usize)> + 'a>
where
    I1: Iterator<Item = usize> + 'a,
    I2: Iterator<Item = usize> + 'a,
{
    Box::new(i1.zip(i2))
}

fn find_visible(
    (nrows, ncols, height): (usize, usize, Heights),
) -> Visibilities {
    use std::iter::repeat as rep;

    let mut visible = vec![vec![false; ncols]; nrows];

    fn mark_visible<S>(
        height: &Heights,
        visible: &mut Visibilities,
        stripe: S,
    ) where
        S: Iterator<Item = (usize, usize)>,
    {
        let mut max = -1;
        for (r, c) in stripe {
            let h = height[r][c];
            if h > max {
                visible[r][c] = true;
                max = h;
            }
        }
    }

    let row_stripes = (0..ncols).flat_map(|c| {
        [ix(0..nrows, rep(c)), ix((0..nrows).rev(), rep(c))]
    });

    let col_stripes = (0..nrows).flat_map(|r| {
        [ix(rep(r), 0..ncols), ix(rep(r), (0..ncols).rev())]
    });

    for s in row_stripes.chain(col_stripes) {
        mark_visible(&height, &mut visible, s);
    }

    visible
}

fn find_view_dists(
    (nrows, ncols, height): (usize, usize, Heights),
) -> ViewDists {
    use std::iter::repeat as rep;

    let mut view_dist = vec![vec![0; ncols]; nrows];

    fn ray_view_dist<R>(height: &Heights, mut coords: R) -> usize
    where
        R: Iterator<Item = (usize, usize)>,
    {
        let (r0, c0) = coords.next().unwrap();
        let h0 = height[r0][c0];
        let mut n = 0;
        for (r, c) in coords {
            let h = height[r][c];
            n += 1;
            if h >= h0 {
                return n;
            }
        }
        n
    }

    #[allow(clippy::needless_range_loop)]
    for r0 in 0..nrows {
        for c0 in 0..ncols {
            let rays = [
                ix((0..=r0).rev(), rep(c0)),
                ix(r0..nrows, rep(c0)),
                ix(rep(r0), (0..=c0).rev()),
                ix(rep(r0), c0..ncols),
            ];
            let d = rays
                .into_iter()
                .map(|ray| ray_view_dist(&height, ray))
                .product();
            view_dist[r0][c0] = d;
        }
    }

    view_dist
}

fn main() {
    let trees = parse_input();
    match get_part() {
        Part1 => {
            let visible = find_visible(trees);
            let nvisible: usize = visible
                .into_iter()
                .map(|r| {
                    r.into_iter().map(|v| v as usize).sum::<usize>()
                })
                .sum();
            println!("{nvisible}");
        }
        Part2 => {
            let view_dist = find_view_dists(trees);
            let max_dist: usize = view_dist
                .into_iter()
                .map(|r| r.into_iter().max().unwrap())
                .max()
                .unwrap();
            println!("{max_dist}");
        }
    }
}
