use crate::*;

const PENALTY: f64 = -0.4;
const WIN_REWARD: f64 = 1.0;
const LOSE_REWARD: f64 = -1.0;

#[derive(Debug, Default, Clone)]
pub struct TicTacToeEnvironment {
    state: GameState,
}

impl TicTacToeEnvironment {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
        }
    }

    pub fn reset(&mut self) {
        self.state.0 = [None; 9];
    }

    /// Input -> action, last_state
    /// Output -> new_state, Reward, next_step(done game or not), winner
    pub fn step(
        &mut self,
        action_id: usize,
        last_state: GameState,
    ) -> (GameState, f64, NextStep, Option<GridState>) {
        let (reward, next_step, winner) = self.transition(action_id, last_state);
        (self.get_state(), reward, next_step, winner)
    }

    /// Input -> action, last_state
    /// Output -> Reward, next_step(done game or not), winner
    fn transition(
        &mut self,
        action_id: usize,
        last_state: GameState,
    ) -> (f64, NextStep, Option<GridState>) {
        let (reward, next_step, winner) = self.get_reward(last_state, self.state);

        if next_step == NextStep::DoneGame {
            return (reward, next_step, winner);
        }

        if self.state.0[action_id].is_none() {
            self.state.0[action_id] = Some(self.state.check_turn());
        }

        let (reward, next_step, winner) = self.get_reward(last_state, self.state);

        (reward, next_step, winner)
    }

    /// Input -> last_state, new_state
    /// Output -> Reward, next_step(done game or not), winner
    fn get_reward(
        &self,
        last_state: GameState,
        new_state: GameState,
    ) -> (f64, NextStep, Option<GridState>) {
        if last_state == new_state && !last_state.is_game_finished() {
            (PENALTY, NextStep::ContinueGame, None)
        } else {
            let next_step;
            let winner;
            let reward;

            match last_state.get_winner() {
                Some(gs) => match gs {
                    GridState::O => {
                        next_step = NextStep::DoneGame;
                        winner = Some(GridState::O);
                        reward = WIN_REWARD;
                    }
                    GridState::X => {
                        next_step = NextStep::DoneGame;
                        winner = Some(GridState::X);
                        reward = LOSE_REWARD;
                    }
                },
                None => {
                    // Draw
                    if self.state.is_game_finished() {
                        next_step = NextStep::DoneGame;
                        winner = None;
                        reward = 0.;
                    } else {
                        next_step = NextStep::ContinueGame;
                        winner = None;
                        reward = 0.
                    }
                }
            }

            (reward, next_step, winner)
        }
    }

    pub fn get_state(&self) -> GameState {
        self.state
    }

    pub fn check_turn(&self) -> GridState {
        self.state.check_turn()
    }
}

impl std::fmt::Display for TicTacToeEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_env() {
        let mut environment = TicTacToeEnvironment::new();
        environment.state.0[3] = Some(GridState::O);

        environment.reset();

        assert!(environment.state.0.iter().all(|gs| gs.is_none()));

        let get_st = environment.get_state();
        assert_eq!(get_st, GameState::default());

        let turn = environment.check_turn();
        assert_eq!(turn, GridState::O);
    }

    #[test]
    fn test_get_reward() {
        let o_win = GameState([
            Some(GridState::X), // x x o
            Some(GridState::X), // o o o
            Some(GridState::O), // x o x
            Some(GridState::O),
            Some(GridState::O),
            Some(GridState::O),
            Some(GridState::X),
            Some(GridState::O),
            Some(GridState::X),
        ]);

        let x_win = GameState([
            Some(GridState::X), // x x o
            Some(GridState::X), // o x x
            Some(GridState::O), // o x o
            Some(GridState::O),
            Some(GridState::X),
            Some(GridState::X),
            Some(GridState::O),
            Some(GridState::X),
            Some(GridState::O),
        ]);

        let draw = GameState([
            Some(GridState::X), // x o x
            Some(GridState::O), // x o o
            Some(GridState::X), // o x o
            Some(GridState::X),
            Some(GridState::O),
            Some(GridState::O),
            Some(GridState::O),
            Some(GridState::X),
            Some(GridState::O),
        ]);

        let init = GameState::default();

        let mut environment = TicTacToeEnvironment::new();

        environment.state = o_win;
        let aaa = environment.get_reward(o_win, environment.state);
        assert_eq!(aaa.0, WIN_REWARD);
        assert_eq!(aaa.1, NextStep::DoneGame);
        assert_eq!(aaa.2, Some(GridState::O));

        environment.state = x_win;
        let bbb = environment.get_reward(x_win, environment.state);
        assert_eq!(bbb.0, LOSE_REWARD);
        assert_eq!(bbb.1, NextStep::DoneGame);
        assert_eq!(bbb.2, Some(GridState::X));

        environment.state = draw;
        let ccc = environment.get_reward(draw, environment.state);
        assert_eq!(ccc.0, 0.);
        assert_eq!(ccc.1, NextStep::DoneGame);
        assert_eq!(ccc.2, None);

        environment.state = init;
        let ddd = environment.get_reward(init, environment.state);
        assert_eq!(ddd.0, PENALTY);
        assert_eq!(ddd.1, NextStep::ContinueGame);
        assert_eq!(ddd.2, None);

        // Transition
        environment.state.0[4] = Some(GridState::O);
        let eee = environment.get_reward(init, environment.state);
        assert_eq!(eee.0, 0.);
        assert_eq!(eee.1, NextStep::ContinueGame);
        assert_eq!(eee.2, None);
    }

    #[test]
    fn test_transition() {
        let mut o_win = GameState([
            Some(GridState::X), // x x o
            Some(GridState::X), //   o o
            Some(GridState::O), //   o x
            None,
            Some(GridState::O),
            Some(GridState::O),
            None,
            Some(GridState::O),
            Some(GridState::X),
        ]);

        let mut environment = TicTacToeEnvironment::new();

        environment.state = o_win;
        let aaa = environment.transition(6, o_win);
        assert_eq!(aaa.0, 0.);
        assert_eq!(aaa.1, NextStep::ContinueGame);
        assert_eq!(aaa.2, None);

        o_win.0[6] = Some(GridState::X);

        environment.state = o_win;
        let bbb = environment.transition(3, o_win);
        assert_eq!(bbb.0, 0.);
        assert_eq!(bbb.1, NextStep::DoneGame);
        assert_eq!(bbb.2, None);

        o_win.0[3] = Some(GridState::O);

        let ccc = environment.transition(0, o_win);
        assert_eq!(ccc.0, WIN_REWARD);
        assert_eq!(ccc.1, NextStep::DoneGame);
        assert_eq!(ccc.2, Some(GridState::O));
    }

    #[test]
    fn test_step() {}
}
