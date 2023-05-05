use rand::{self, Rng};
use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone)]
pub struct Agent {
    epsilon: f64,
    alpha: f64,
    n: HashMap<GameState, [u64; 9]>,
    q: HashMap<GameState, [f64; 9]>,
    turn: GridState,
    last_state: GameState,
}

impl Agent {
    pub fn new(epsilon: f64, alpha: f64) -> Self {
        Self {
            epsilon,
            alpha,
            n: HashMap::new(),
            q: HashMap::new(),
            turn: GridState::O,
            last_state: GameState::new(),
        }
    }

    pub fn policy(&self, env: &TicTacToeEnvironment) -> usize {
        let mut rng = rand::thread_rng();

        if rng.gen_bool(self.epsilon) {
            rng.gen_range(0..9)
        } else {
            let state = env.get_state();

            match self.q.get_key_value(&state) {
                Some((_key, &q_table)) => {
                    let (action_id, _q) = q_table.iter().enumerate().fold(
                        (q_table.len(), f64::MIN),
                        |(i_max, q_max), (i_val, &q_val)| {
                            if q_max > q_val {
                                (i_max, q_max)
                            } else {
                                (i_val, q_val)
                            }
                        },
                    );
                    action_id
                }
                None => rng.gen_range(0..9),
            }
        }
    }

    pub fn play(&mut self, env: &mut TicTacToeEnvironment) -> Vec<(Option<GridState>, f64)> {
        let mut rng = rand::thread_rng();

        env.reset();
        self.turn = random_grid_state();

        let mut rewards = vec![];
        let mut experience = vec![];

        'learn: loop {
            if env.check_turn() == self.turn {
                let first_state = env.get_state();
                let action_id = self.policy(env);
                let (game_state, reward, next_step, winner) = env.step(action_id, self.last_state);

                // Update to transitioned state.
                self.last_state = game_state;

                experience.push((first_state, action_id, reward));
                rewards.push((winner, reward));

                if next_step == NextStep::DoneGame {
                    break 'learn;
                }
            } else {
                let action_id = rng.gen_range(0..9);
                let (game_state, _, _, _) = env.step(action_id, self.last_state);
                // Update to transitioned state.
                self.last_state = game_state;
            }
        }

        for (i, (game_state, action_id, _)) in experience.iter().enumerate() {
            let mut g = 0.;

            for (count, (_, _, reward)) in experience.iter().skip(i).enumerate() {
                g += 0.9f64.powi(count as i32) * reward;
            }

            let n_default = [0u64; 9];
            let mut n_count = *self.n.get(game_state).unwrap_or(&n_default);
            n_count[*action_id] += 1;
            self.n.entry(*game_state).or_insert(n_count);

            let alpha = 1. / n_count[*action_id] as f64;
            let alpha = alpha.max(self.alpha);

            let q_defalut = [0f64; 9];
            let mut q_table = *self.q.get(game_state).unwrap_or(&q_defalut);
            q_table[*action_id] += alpha * (g - q_table[*action_id]);
            self.q.insert(*game_state, q_table);
        }

        rewards
    }

    pub fn get_q_table(&self) -> HashMap<GameState, [f64; 9]> {
        self.q.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EPSILON: f64 = 0.1;
    const ALPHA: f64 = 0.01;

    #[test]
    fn test_agent_policy() {
        let mut agent = Agent::new(EPSILON, ALPHA);
        let env = TicTacToeEnvironment::new();

        let gs = GameState::new();
        let q_table = [0.; 9];

        agent.q.insert(gs, q_table);
        let action_id = agent.policy(&env);

        assert!((0..9).contains(&action_id));
    }

    #[test]
    fn test_agent_play() {
        let mut agent = Agent::new(EPSILON, ALPHA);
        let mut env = TicTacToeEnvironment::new();

        let log_vec = agent.play(&mut env);

        println!("N:\n{:?}\n", agent.n);
        println!("Q:\n{:?}", agent.q);

        println!("{log_vec:?}");
    }
}
