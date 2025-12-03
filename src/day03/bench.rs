extern crate test;

use test::Bencher;

use super::{part1, part2};

#[bench]
fn bench_part1(b: &mut Bencher) {
    let input = include_str!("../../inputs/day03.txt");

    b.iter(|| part1::run(input));
}

#[bench]
fn bench_part2(b: &mut Bencher) {
    let input = include_str!("../../inputs/day03.txt");

    b.iter(|| part2::run(input));
}
