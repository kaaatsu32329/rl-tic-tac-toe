use rl_tic_tac_toe::*;

const EPSILON: f64 = 0.1;
const ALPHA: f64 = 0.01;

const STEP: usize = 1000000;

fn main() {
    let mut environment = TicTacToeEnvironment::new();
    let mut agent = Agent::new(EPSILON, ALPHA);

    let mut winning_count = 0;

    for i in 0..=STEP {
        let v = agent.play(&mut environment);

        if let Some(gs) = v.last().unwrap().0 {
            if gs == GridState::O {
                winning_count += 1;
            }
        }

        if i % 50000 == 0 && i != 0 {
            println!("--------------------------------------");
            println!("Step: {}", i);
            println!("Win rate: {}", winning_count as f64 / 500.);
            println!("Q table:{:?}\n", agent.get_q_table().get(&GameState::new()));

            winning_count = 0;
        }
    }
}
