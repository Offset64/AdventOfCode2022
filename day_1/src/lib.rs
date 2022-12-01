use itertools::Itertools;

/*
    The input is a list of numbers, occasionally separated by blank line
    Group each block of numbers and sum the blocks.
    Find the highest valued (summed) block.
*/
pub fn get_largest_block(input: String) -> u32 {
    convert_to_blocks(input).into_iter().max().unwrap()
}

//Similar to the above, except that we want an arbitrary number of blocks
pub fn get_n_largest_blocks(input: String, block_count: usize) -> Vec<u32> {
    convert_to_blocks(input)
        .into_iter()
        .sorted()
        .rev()
        .take(block_count)
        .collect::<Vec<u32>>()
}

fn convert_to_blocks(input: String) -> Vec<u32> {
    input
        .split("\n\n") // Split the input into blocks
        .map(
            //Convert each block to a sum of the ints that block contains
            |block| {
                block
                    .split("\n")
                    //Take each entry in the block and convert it to an int
                    .map(|item| item.parse::<u32>().unwrap())
                    .sum::<u32>()
            },
        )
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn can_solve_part_1() {
        let input = fs::read_to_string("inputs/example_input.txt").unwrap();
        let expected_result = 24000u32;
        assert_eq!(get_largest_block(input), expected_result);
    }

    #[test]
    fn can_solve_part_2() {
        let input = fs::read_to_string("inputs/example_input.txt").unwrap();
        let expected_result = 45000u32;
        assert_eq!(
            get_n_largest_blocks(input, 3).iter().sum::<u32>(),
            expected_result
        );
    }
}
