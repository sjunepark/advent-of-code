use day_01::part2::parse;

fn main() {
    let file = include_str!("./input2.txt");
    let result = parse(file);
    println!("{:?}", result);
}
