use day_4::{count_full_dupes, count_partial_dupes, get_ranges_from_input};

fn main() {
    let input = std::fs::read_to_string("day_4/inputs/input.txt").unwrap();
    let ranges = get_ranges_from_input(input);
    println!("Part 1 - {}", count_full_dupes(&ranges));
    println!("Part 2 - {}", count_partial_dupes(&ranges));
}
