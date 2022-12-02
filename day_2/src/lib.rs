use std::vec;

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug)]
pub struct OldStrat {
    rounds: Vec<Round>,
}
impl OldStrat {
    pub fn new(input: String) -> Self {
        Self {
            rounds: input
                .split("\n")
                .map(|l| Round {
                    opponents_choice: Move::new(&l.chars().nth(0).unwrap()),
                    player_choice: Move::new(&l.chars().nth(2).unwrap()),
                })
                .collect_vec(),
        }
    }
    pub fn calculate_score(&self) -> u32 {
        self.rounds
            .iter()
            .fold(0u32, |acc, r| acc + r.calculate_score())
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct NewStrat {
    rounds: Vec<Round>,
}
impl NewStrat {
    pub fn new(input: String) -> Self {
        Self {
            rounds: input
                .split("\n")
                .map(|l| {
                    let op_move = Move::new(&l.chars().nth(0).unwrap());
                    let desired = Outcome::new(&l.chars().nth(2).unwrap());
                    Round {
                        player_choice: Move::cheat(desired, &op_move),
                        opponents_choice: op_move,
                    }
                })
                .collect_vec(),
        }
    }

    pub fn calculate_score(&self) -> u32 {
        self.rounds
            .iter()
            .fold(0u32, |acc, r| acc + r.calculate_score())
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Round {
    opponents_choice: Move,
    player_choice: Move,
}

impl Round {
    pub fn calculate_score(&self) -> u32 {
        self.player_choice.score()
            + match self.player_choice.versus(&self.opponents_choice) {
                Outcome::Win => 6,
                Outcome::Lose => 0,
                Outcome::Draw => 3,
            }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    pub fn versus(&self, other: &Move) -> Outcome {
        match (self, other) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Lose,
            (Move::Rock, Move::Scissor) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissor) => Outcome::Lose,
            (Move::Scissor, Move::Rock) => Outcome::Lose,
            (Move::Scissor, Move::Paper) => Outcome::Win,
            (Move::Scissor, Move::Scissor) => Outcome::Draw,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }

    pub fn new(from: &char) -> Self {
        match from {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissor,
            _ => {
                todo!()
            }
        }
    }

    pub fn cheat(target: Outcome, other: &Move) -> Self {
        let moves = vec![Move::Rock, Move::Paper, Move::Scissor];
        for m in moves {
            if target == m.versus(other) {
                return m;
            }
        }
        todo!()
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}
impl Outcome {
    pub fn new(from: &char) -> Self {
        match from {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_input() {
        let input = std::fs::read_to_string("inputs/example_input.txt").unwrap();
        assert_eq!(
            OldStrat::new(input),
            OldStrat {
                rounds: vec![
                    Round {
                        opponents_choice: Move::Rock,
                        player_choice: Move::Paper
                    },
                    Round {
                        opponents_choice: Move::Paper,
                        player_choice: Move::Rock
                    },
                    Round {
                        opponents_choice: Move::Scissor,
                        player_choice: Move::Scissor
                    },
                ]
            }
        )
    }

    #[test]
    fn can_solve_part_1() {
        let input = std::fs::read_to_string("inputs/example_input.txt").unwrap();
        let strategy = OldStrat::new(input);
        println!("{:#?}", strategy);
        assert_eq!(strategy.calculate_score(), 15)
    }

    #[test]
    fn can_solve_part_2() {
        let input = std::fs::read_to_string("inputs/example_input.txt").unwrap();
        let strategy = NewStrat::new(input);
        println!("{:#?}", strategy);
        assert_eq!(strategy.calculate_score(), 12)
    }
}
