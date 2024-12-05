fn parse_usize_bytes(b: &[u8]) -> usize {
    let mut res = 0;
    for c in b {
        res = res * 10 + *c as usize - b'0' as usize;
    }
    res
}

pub fn parse_usize(s: &str) -> usize {
    parse_usize_bytes(s.as_bytes())
}

pub fn parse_isize(s: &str) -> isize {
    let b = s.as_bytes();
    if b[0] == b'-' {
        -(parse_usize_bytes(&b[1..]) as isize)
    } else {
        parse_usize_bytes(b) as isize
    }
}
