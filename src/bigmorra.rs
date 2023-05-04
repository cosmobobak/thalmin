#![allow(clippy::module_name_repetitions)]

use std::fmt::Display;

use crate::game::RpsGame;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BigFingerCount {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
}

impl BigFingerCount {
    pub const fn from_index(index: usize) -> Self {
        match index {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Three,
            3 => Self::Four,
            4 => Self::Five,
            _ => panic!("Invalid action index"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BigMorraAction {
    num_fingers: BigFingerCount,
    prediction: BigFingerCount,
}

impl Display for BigMorraAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} fingers, predict {}", self.num_fingers as i8 + 1, self.prediction as i8 + 1)
    }
}

impl BigMorraAction {
    pub const fn from_index(index: usize) -> Self {
        assert!(index < 25);
        let num_fingers = BigFingerCount::from_index(index / 5);
        let prediction = BigFingerCount::from_index(index % 5);
        Self {
            num_fingers,
            prediction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BigMorra;

impl RpsGame for BigMorra {
    const NUM_ACTIONS: usize = 25;
    type Action = BigMorraAction;

    fn action_utility(our_action: Self::Action, opponent_action: Self::Action) -> f64 {
        let BigMorraAction {
            num_fingers,
            prediction,
        } = our_action;
        let BigMorraAction {
            num_fingers: opponent_num_fingers,
            prediction: opponent_prediction,
        } = opponent_action;

        let us_correct = prediction == opponent_num_fingers;
        let them_correct = opponent_prediction == num_fingers;
        if us_correct && them_correct {
            return 0.0;
        }
        if !us_correct && !them_correct {
            return 0.0;
        }
        let finger_sum = f64::from(num_fingers as i8 + 1) + f64::from(opponent_num_fingers as i8 + 1);
        if us_correct {
            finger_sum
        } else {
            -finger_sum
        }
    }

    fn nth_action(n: usize) -> Self::Action {
        Self::Action::from_index(n)
    }
}
