mod functions;
use functions::open_or_senior::open_or_senior;

fn main() {
    let l = vec![(45, 12), (55,21), (19, -2), (104, 20)];

    println!("{:?}", open_or_senior(l));
}
