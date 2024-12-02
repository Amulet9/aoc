#[inline]
pub const fn atoi(line: &[u8]) -> (u32, u32) {
    let n1 = ((line[0] - b'0') as u32) * 10000
        + ((line[1] - b'0') as u32) * 1000
        + ((line[2] - b'0') as u32) * 100
        + ((line[3] - b'0') as u32) * 10
        + ((line[4] - b'0') as u32);

    let n2 = ((line[8] - b'0') as u32) * 10000
        + ((line[9] - b'0') as u32) * 1000
        + ((line[10] - b'0') as u32) * 100
        + ((line[11] - b'0') as u32) * 10
        + ((line[12] - b'0') as u32);

    (n1, n2)
}

pub fn part1(input: &'static str) -> u32 {
    let mut lbuf = [0u32; 1000];
    let mut rbuf = [0u32; 1000];
    let mut count = 0usize;

    let bytes = input.as_bytes();
    let mut i = 0;

    while i + 13 <= bytes.len() {
        let line = &bytes[i..i + 13];
        let (l, r) = atoi(line);
        lbuf[count] = l;
        rbuf[count] = r;
        count += 1;
        i += 14;
    }

    radsort::sort(&mut lbuf[..count]);
    radsort::sort(&mut rbuf[..count]);

    let mut total = 0u32;
    for i in 0..count {
        let l = lbuf[i];
        let r = rbuf[i];
        total += l.abs_diff(r);
    }

    total
}
