use day_2::{NewStrat, OldStrat};
fn main() {
    let input = std::fs::read_to_string("day_2/inputs/input.txt").unwrap();
    println!(
        "Part 1 - {}",
        OldStrat::new(input.clone()).calculate_score()
    );
    println!("Part 2 - {}", NewStrat::new(input).calculate_score());
}
