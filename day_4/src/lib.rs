use itertools::Itertools;
use std::ops::RangeInclusive;

type RangePair = (RangeInclusive<u32>, RangeInclusive<u32>);

// Given an input text, get the pairs of ranges it represents
pub fn get_ranges_from_input(input: String) -> Vec<RangePair> {
    input
        .split("\n")
        .map(|line| {
            line.split(',')
                .map(|r| {
                    let (start, end) = r
                        .split("-")
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap();
                    start..=end
                })
                .collect_tuple::<RangePair>()
                .unwrap()
        })
        .collect_vec()
}

pub fn count_full_dupes(pairs: &Vec<RangePair>) -> u32 {
    pairs
        .iter()
        .fold(0, |acc, pair| match does_pair_contain_dupe(pair) {
            true => acc + 1,
            false => acc,
        })
}

pub fn does_pair_contain_dupe(pair: &RangePair) -> bool {
    let (left, right) = pair;
    (left.contains(&right.start()) && left.contains(&right.end()))
        || (right.contains(&left.start()) && right.contains(&left.end()))
}

pub fn does_pair_overlap(pair: &RangePair) -> bool {
    let (left, right) = pair;
    left.contains(&right.start())
        || left.contains(&right.end())
        || right.contains(&left.start())
        || right.contains(&left.end())
}

pub fn count_partial_dupes(pairs: &Vec<RangePair>) -> u32 {
    pairs
        .iter()
        .fold(0, |acc, pair| match does_pair_overlap(pair) {
            true => acc + 1,
            false => acc,
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part_1() {
        let input = std::fs::read_to_string("inputs/example_input.txt").unwrap();
        let ranges = get_ranges_from_input(input);
        let dupes = count_full_dupes(&ranges);
        assert_eq!(2, dupes);
    }

    #[test]
    fn can_solve_part_2() {
        let input = std::fs::read_to_string("inputs/example_input.txt").unwrap();
        let ranges = get_ranges_from_input(input);
        let total_dupes = count_partial_dupes(&ranges);
        assert_eq!(4, total_dupes);
    }
}
