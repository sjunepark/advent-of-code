use day_01::part1::parse;

fn main() {
    let file = include_str!("./input1.txt");
    let result = parse(file);
    println!("{:?}", result);
}
