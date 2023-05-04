#![allow(clippy::module_name_repetitions, clippy::cast_precision_loss)]

pub trait RpsGame {
    const NUM_ACTIONS: usize;
    type Action: Copy + PartialEq + Eq + std::fmt::Debug + std::fmt::Display;
    fn action_utility(our_action: Self::Action, opponent_action: Self::Action) -> f64;
    fn nth_action(n: usize) -> Self::Action;
    fn format_action_distribution(distribution: &[f64]) -> String {
        assert_eq!(distribution.len(), Self::NUM_ACTIONS, "Invalid action distribution length, expected {}, got {}", Self::NUM_ACTIONS, distribution.len());
        let mut s = String::new();
        let mut indexed_probabilities = distribution.iter().enumerate().collect::<Vec<_>>();
        indexed_probabilities.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
        for (action, probability) in indexed_probabilities {
            s.push_str(&format!("{:15}: {:.2}%,\n", Self::nth_action(action), probability * 100.0));
        }
        s.pop();
        s.pop();
        s
    }
    fn uniform_policy() -> Vec<f64> {
        vec![1.0 / Self::NUM_ACTIONS as f64; Self::NUM_ACTIONS]
    }
}