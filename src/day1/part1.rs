use std::fmt::Display;

fn split_vec(vec: Vec<(u32, u32)>) -> (Vec<u32>, Vec<u32>) {
    let first: Vec<u32> = vec.iter().map(|&(a, _)| a).collect();
    let second: Vec<u32> = vec.iter().map(|&(_, b)| b).collect();
    (first, second)
}

pub fn part1(input: &'static str) -> impl Display {
    let (mut lbuf, mut rbuf) = split_vec(input.lines().map(|line| {
        let mut chars = line.as_bytes();
        (atoi(&chars[..5]), atoi(&chars[8..13]))
    }).collect());

    lbuf.sort_unstable();
    rbuf.sort_unstable();

    lbuf.into_iter().zip(rbuf.into_iter()).map(|(l, r)| l.abs_diff(r)).sum::<u32>()
}

pub fn atoi(chars: &[u8]) -> u32 {
    
    chars.into_iter().fold(0u32, |mut c, m| {
        c *= 10;
        c += *m as u32;
        c
    })
    
}