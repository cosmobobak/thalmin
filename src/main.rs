#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(unused_imports)]

mod rps;
mod agent;
mod game;
mod altrps;
mod morra;
mod bigmorra;
mod morra2;

use bigmorra::BigMorra;
use morra::Morra;
use morra2::Morra2;
use rps::RockPaperScissors;
use altrps::AltRockPaperScissors;

use crate::game::RpsGame;
type G = RockPaperScissors;

fn main() {
    println!("Computing Nash equilibrium...");
    println!("Training agent against itself... (this may take a while)");
    let mut agent_1 = agent::Agent::<G>::new();
    let mut agent_2 = agent::Agent::<G>::new();
    for _ in 0..10 {
        agent_1.train(&mut agent_2, 1_000_000);
        println!("strategy: {:?}", agent_1.get_average_strategy().iter().map(|x| (x * 100.0).round() / 100.0).collect::<Vec<_>>());
    }
    let final_strategy = agent_1.get_average_strategy();
    println!("Final strategy:\n{}", G::format_action_distribution(&final_strategy));

    println!(
        "Utility gained against uniform policy: {}",
        agent::Agent::<G>::avg_utility(&final_strategy, &G::uniform_policy())
    );
    // let mut user_input = String::new();
    // loop {
    //     println!("Pick an action from 0 to {}: ", G::NUM_ACTIONS - 1);
    //     std::io::stdin().read_line(&mut user_input).unwrap();
    //     let user_action = user_input.trim().parse::<usize>().unwrap();
    //     if user_action >= G::NUM_ACTIONS {
    //         println!("Invalid action!");
    //         continue;
    //     }
    //     let agent_action = agent::Agent::<G>::get_action(&final_strategy);
    //     let user_action = G::nth_action(user_action);
    //     let agent_action = G::nth_action(agent_action);
    //     println!("You picked: {user_action:?}");
    //     println!("Agent picked: {agent_action:?}");
    //     println!("Your utility: {}", G::action_utility(user_action, agent_action));
    //     println!("Agent utility: {}", G::action_utility(agent_action, user_action));
    //     println!();
    //     user_input.clear();
    // }
}
