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

fn find_view_dists((nrows, ncols, height): (usize, usize, Heights)) -> ViewDists {
    use std::iter::repeat as rep;

    let mut view_dist = vec![vec![0; ncols]; nrows];

    fn ray_view_dist<R, C>(height: &Heights, (rs, cs): (R, C)) -> usize
        where R: Iterator<Item = usize>, C: Iterator<Item = usize>
    {
        let mut coords = rs.zip(cs);
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

    fn ra<'a, E, I>(i: I) -> Box<dyn Iterator<Item = E> + 'a>
    where I: Iterator<Item = E> + 'a
    {
        Box::new(i)
    }

    #[allow(clippy::needless_range_loop)]
    for r0 in 0..nrows {
        for c0 in 0..ncols {
            let rays = [
                (ra((0..=r0).rev()), ra(rep(c0))),
                (ra(r0..nrows), ra(rep(c0))),
                (ra(rep(r0)), ra((0..=c0).rev())),
                (ra(rep(r0)), ra(c0..ncols)),
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
                .into_iter().map(|r| {
                    r.into_iter().map(|v| v as usize).sum::<usize>()
                })
                .sum();
            println!("{nvisible}");
        }
        Part2 => {
            let view_dist = find_view_dists(trees);
            let max_dist: usize = view_dist
                .into_iter().map(|r| {
                    r.into_iter().max().unwrap()
                })
                .max()
                .unwrap();
            println!("{max_dist}");
        }
    }
}
