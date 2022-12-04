use day_3::*;

fn main() {
    let input = std::fs::read_to_string("day_3/inputs/input.txt").unwrap();
    println!("Part 1 - {}", part_1(input.clone()));
    println!("Part 2 - {}", part_2(input));
}
