mod part2;

// beat 47 micro with binary search
fn main() {
    let input = include_str!("../input2.txt");

    part2::process(input).unwrap();
}
