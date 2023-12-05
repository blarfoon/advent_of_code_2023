pub mod part1;
pub mod part2;

fn main() {
    let input = include_str!("../input2.txt");

    println!("{}", part2::process(input).unwrap());
}
