use std::{collections::HashMap, ops::AddAssign, slice::SliceIndex};
use rustc_hash::{FxBuildHasher, FxHashMap};

#[inline]
fn swar(chunk: u64) -> u64 {
    let lower = chunk & 0x000f000f000f000f;
    let upper = (chunk & 0x0f000f000f000f00) >> 8;
    let chunk = 10 * upper + lower;

    let lower = chunk & 0x000000ff000000ff;
    let upper = (chunk & 0x00ff000000ff0000) >> 16;
    let chunk = 100 * upper + lower;

    let lower = chunk & 0x000000000000ffff;
    let upper = (chunk & 0x0000ffff00000000) >> 32;
    10000 * upper + lower
}

pub fn part2(input: &str) -> u64 {
    let mut left = Vec::with_capacity(1000);
    let mut right =
        HashMap::<_, _, FxBuildHasher>::with_capacity_and_hasher(1_000, FxBuildHasher::default());

    for chunk in input.as_bytes().chunks_exact(14) {
        let n = u64::from_be_bytes(chunk[0..8].try_into().unwrap()) >> 24;
        left.push(n);

        let n = u64::from_be_bytes(chunk[5..13].try_into().unwrap()) & 0xffffffffff;
        *right.entry(n).or_insert(0) += 1;
    }

    left.iter()
        .filter_map(|n| right.get(n).map(|f| swar(*n) * f))
        .sum()
}
