#![allow(clippy::module_name_repetitions)]

use std::fmt::Display;

use crate::game::RpsGame;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FingerCount {
    One = 0,
    Two = 1,
    Three = 2,
}

impl FingerCount {
    pub const fn from_index(index: usize) -> Self {
        match index {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Three,
            _ => panic!("Invalid action index"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MorraAction {
    num_fingers: FingerCount,
    prediction: FingerCount,
}

impl Display for MorraAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} fingers, predict {}", self.num_fingers as i8 + 1, self.prediction as i8 + 1)
    }
}

impl MorraAction {
    pub const fn from_index(index: usize) -> Self {
        assert!(index < 9);
        let num_fingers = FingerCount::from_index(index / 3);
        let prediction = FingerCount::from_index(index % 3);
        Self {
            num_fingers,
            prediction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Morra;

impl RpsGame for Morra {
    const NUM_ACTIONS: usize = 9;
    type Action = MorraAction;

    fn action_utility(our_action: Self::Action, opponent_action: Self::Action) -> f64 {
        let MorraAction {
            num_fingers,
            prediction,
        } = our_action;
        let MorraAction {
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
