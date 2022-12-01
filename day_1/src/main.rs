use day_1::{get_largest_block, get_n_largest_blocks};

fn main() {
    let input = std::fs::read_to_string("day_1/inputs/input.txt").unwrap();
    println!("Part 1 - {}", get_largest_block(input.clone()));
    println!(
        "Part 2 - {}",
        get_n_largest_blocks(input, 3).into_iter().sum::<u32>()
    );
}
