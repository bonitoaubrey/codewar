pub fn xo(string: &'static str) -> bool {
    let mut x = 0;
    let mut o = 0;

    for i in string.to_lowercase().as_bytes() {
        if *i == b'x' {x += 1}
        if *i == b'o' {o += 1}
    }
    x == o
}
