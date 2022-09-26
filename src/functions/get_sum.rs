pub fn get_sum(a: i64, b: i64) -> i64 {
    if a == b {
        return a;
    }

    let mut sum = 0;
    let mut c = a;

    while c != b {
        if c < b {
            sum += c;
            c += 1;
        } else {
            sum += c;
            c -= 1;
        }
    }

    sum + b
}
