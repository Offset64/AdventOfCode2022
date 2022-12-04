use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Sack {
    pub left: Compartment,
    pub right: Compartment,
}

impl Sack {
    pub fn combined(&self) -> HashSet<String> {
        self.left
            .contents
            .union(&self.right.contents)
            .cloned()
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Compartment {
    pub contents: HashSet<String>,
}

impl Compartment {
    // Find all common items between compartments
    pub fn find_common_items(&self, other: &Compartment) -> Vec<String> {
        self.contents
            .intersection(&other.contents)
            .map(|diff| diff.to_owned())
            .collect()
    }
}

impl From<&str> for Compartment {
    fn from(s: &str) -> Self {
        let mut set = HashSet::new();
        for c in s.chars() {
            set.insert(c.to_string());
        }
        Self { contents: set }
    }
}

impl From<&str> for Sack {
    fn from(s: &str) -> Self {
        let (left, right) = s.split_at(s.len() / 2);
        Sack {
            left: Compartment::from(left),
            right: Compartment::from(right),
        }
    }
}

pub fn score_item(item: &str) -> u32 {
    let item_value = item.as_bytes().first().unwrap();
    let modifier = match item.chars().next().unwrap() {
        'a'..='z' => 96,
        'A'..='Z' => 38,
        _ => panic!("Unexpected character"),
    };

    *item_value as u32 - modifier
}

pub fn find_common_items_in_group(mut group: Vec<Sack>) -> Vec<String> {
    let mut common_set = group.pop().unwrap().combined();
    for sack in group {
        common_set = common_set.intersection(&sack.combined()).cloned().collect();
    }

    common_set.into_iter().collect_vec()
}

pub fn part_1(input: String) -> u32 {
    let sacks: Vec<Sack> = input.split("\n").map(|line| Sack::from(line)).collect();
    let scores = sacks
        .into_iter()
        .map(|s| s.left.find_common_items(&s.right))
        .map(|x| x.into_iter().fold(0, |acc, x| acc + score_item(&x)))
        .collect_vec();
    let total_score: u32 = scores.into_iter().sum();
    total_score
}
pub fn part_2(input: String) -> u32 {
    let sacks: Vec<Sack> = input.split("\n").map(|line| Sack::from(line)).collect();
    let score = sacks
        .chunks(3)
        .map(|group| find_common_items_in_group(group.to_vec()))
        .map(|x| x.into_iter().fold(0, |acc, x| acc + score_item(&x)))
        .sum::<u32>();
    score
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn can_parse_input() {
        let input = std::fs::read_to_string("inputs/example_input.txt").unwrap();
        let sacks: Vec<Sack> = input.split("\n").map(|line| Sack::from(line)).collect();
        assert_eq!(sacks.len(), 6);
    }

    #[test]
    fn can_find_common_items() {
        let input = std::fs::read_to_string("inputs/example_input.txt").unwrap();
        let sack: Sack = input
            .split("\n")
            .map(|line| Sack::from(line))
            .collect::<Vec<Sack>>()
            .into_iter()
            .nth(0)
            .unwrap();
        let diff = sack.left.find_common_items(&sack.right);
        assert_eq!(diff, vec!["p"]);
    }

    #[test]
    fn can_solve_part_1() {
        let input = std::fs::read_to_string("inputs/example_input.txt").unwrap();
        let total_score = part_1(input);
        assert_eq!(total_score, 157);
    }

    #[test]
    fn can_solve_part_2() {
        let input = std::fs::read_to_string("inputs/example_input.txt").unwrap();

        let score = part_2(input);

        assert_eq!(score, 70);
    }
}
