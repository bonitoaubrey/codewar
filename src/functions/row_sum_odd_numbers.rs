pub fn row_sum_odd_numbers(n: i64) -> i64 {
    let mut number = 1;
    let mut i;
    let mut j = n;
    let mut sum = 0;

    while j > 1 {
        i = j;
        while i > 1 {
            i -= 1;
            number += 2;
        }
        j -= 1;
    }

    i = n;
    while i > 0 {
        sum += number;
        number += 2;
        i -= 1;
    }

    sum
}
