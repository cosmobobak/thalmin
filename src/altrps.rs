use std::fmt::Display;

use crate::game::RpsGame;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AltRockPaperScissorsAction {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Display for AltRockPaperScissorsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rock => write!(f, "Rock"),
            Self::Paper => write!(f, "Paper"),
            Self::Scissors => write!(f, "Scissors"),
        }
    }
}

impl AltRockPaperScissorsAction {
    pub const fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            _ => panic!("Invalid action index"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AltRockPaperScissors;

impl RpsGame for AltRockPaperScissors {
    const NUM_ACTIONS: usize = 3;
    type Action = AltRockPaperScissorsAction;

    fn action_utility(our_action: Self::Action, opponent_action: Self::Action) -> f64 {
        match (our_action, opponent_action) {
            (Self::Action::Rock, Self::Action::Paper) => -1.0,
            (Self::Action::Paper, Self::Action::Scissors) => -2.0,
            (Self::Action::Scissors, Self::Action::Rock) => -3.0,
            (Self::Action::Rock, Self::Action::Scissors) => 3.0,
            (Self::Action::Paper, Self::Action::Rock) => 2.0,
            (Self::Action::Scissors, Self::Action::Paper) => 1.0,
            _ => 0.0,
        }
    }

    fn nth_action(n: usize) -> Self::Action {
        Self::Action::from_index(n)
    }
}