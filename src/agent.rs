#![allow(clippy::cast_precision_loss)]

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::game::RpsGame;

#[derive(Debug, Clone)]
pub struct Agent<G: RpsGame + Clone + Send + Sync> {
    regret_sum: Vec<f64>,
    strategy_sum: Vec<f64>,
    _marker: std::marker::PhantomData<G>,
}

impl<G: RpsGame + Clone + Send + Sync> Agent<G> {
    pub fn new() -> Self {
        Self {
            regret_sum: vec![0.0; G::NUM_ACTIONS],
            strategy_sum: vec![0.0; G::NUM_ACTIONS],
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get_strategy(&mut self) -> Vec<f64> {
        let mut normalizing_sum = 0.0;
        let mut strategy = vec![0.0; G::NUM_ACTIONS];

        for (s, rs) in strategy.iter_mut().zip(self.regret_sum.iter()) {
            *s = rs.max(0.0);
            normalizing_sum += *s;
        }

        for (s, ss) in strategy.iter_mut().zip(self.strategy_sum.iter_mut()) {
            if normalizing_sum > 0.0 {
                *s /= normalizing_sum;
            } else {
                *s = 1.0 / G::NUM_ACTIONS as f64;
            }
            *ss += *s;
        }

        strategy
    }

    pub fn get_action(strategy: &[f64]) -> usize {
        let r = rand::random::<f64>();
        let mut cumulative_probability = 0.0;
        for (i, s) in strategy.iter().enumerate() {
            cumulative_probability += *s;
            if r < cumulative_probability {
                return i;
            }
        }
        G::NUM_ACTIONS - 1
    }

    pub fn _train_against_fixed(&mut self, iterations: usize, opponent_strategy: &[f64]) {
        let mut action_utility = vec![0.0; G::NUM_ACTIONS];
        for _ in 0..iterations {
            // Get regret-matched mixed-strategy actions:
            let strategy = self.get_strategy();
            let action = Self::get_action(&strategy);
            let opponent_action = Self::get_action(opponent_strategy);
            // Compute action utilities:
            let opponent_action = G::nth_action(opponent_action);
            for (i, au) in action_utility.iter_mut().enumerate() {
                *au = G::action_utility(G::nth_action(i), opponent_action);
            }
            // Accumulate action regrets:
            let utility_of_taken_action = action_utility[action];
            for (rs, au) in self.regret_sum.iter_mut().zip(action_utility.iter()) {
                *rs += *au - utility_of_taken_action;
            }
        }
    }

    pub fn train(&mut self, opponent: &mut Self, iterations: usize) {
        let mut action_utility = vec![0.0; G::NUM_ACTIONS];
        let mut opp_action_utility = vec![0.0; G::NUM_ACTIONS];
        for _ in 0..iterations {
            // Get regret-matched mixed-strategy actions:
            let strategy = self.get_strategy();
            let opponent_strategy = opponent.get_strategy();
            let action = Self::get_action(&strategy);
            let opponent_action = Self::get_action(&opponent_strategy);
            // Compute action utilities:
            let opponent_action_obj = G::nth_action(opponent_action);
            for (i, au) in action_utility.iter_mut().enumerate() {
                *au = G::action_utility(G::nth_action(i), opponent_action_obj);
            }
            let action_obj = G::nth_action(action);
            for (i, au) in opp_action_utility.iter_mut().enumerate() {
                *au = G::action_utility(G::nth_action(i), action_obj);
            }
            // Accumulate action regrets:
            let utility_of_taken_action = action_utility[action];
            for (rs, au) in self.regret_sum.iter_mut().zip(action_utility.iter()) {
                *rs += *au - utility_of_taken_action;
            }
            let utility_of_taken_action = opp_action_utility[opponent_action];
            for (rs, au) in opponent.regret_sum.iter_mut().zip(opp_action_utility.iter()) {
                *rs += *au - utility_of_taken_action;
            }
        }
    }

    pub fn get_average_strategy(&self) -> Vec<f64> {
        let mut avg_strategy = vec![0.0; G::NUM_ACTIONS];
        let normalizing_sum = self.strategy_sum.iter().sum::<f64>();
        for (ss, av_s) in self.strategy_sum.iter().zip(avg_strategy.iter_mut()) {
            if normalizing_sum > 0.0 {
                *av_s = *ss / normalizing_sum;
            } else {
                *av_s = 1.0 / G::NUM_ACTIONS as f64;
            }
        }
        avg_strategy
    }

    pub fn avg_utility(our_policy: &[f64], opp_policy: &[f64]) -> f64 {
        let mut utility = 0.0;
        for (i, &our_p) in our_policy.iter().enumerate() {
            for (j, &opp_p) in opp_policy.iter().enumerate() {
                utility += our_p * opp_p * G::action_utility(G::nth_action(i), G::nth_action(j));
            }
        }
        utility
    }
}