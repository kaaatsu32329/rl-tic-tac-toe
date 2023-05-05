use crate::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct GameState(pub(crate) [Option<GridState>; 9]);

impl GameState {
    pub fn new() -> Self {
        Self([None; 9])
    }

    pub fn is_game_finished(&self) -> bool {
        self.0.iter().all(|&gs| gs.is_some())
    }

    pub fn get_winner(&self) -> Option<GridState> {
        // Horizontal
        for col in 0..3 {
            if self.0[3 * col].is_some()
                && self.0[3 * col] == self.0[3 * col + 1]
                && self.0[3 * col] == self.0[3 * col + 2]
            {
                return self.0[3 * col];
            }
        }
        // Vertical
        for row in 0..3 {
            if self.0[row].is_some()
                && self.0[row] == self.0[row + 3]
                && self.0[row] == self.0[row + 6]
            {
                return self.0[row];
            }
        }
        // Diagonal
        if self.0[0].is_some() && self.0[0] == self.0[4] && self.0[0] == self.0[8] {
            return self.0[0];
        } else if self.0[2].is_some() && self.0[2] == self.0[4] && self.0[2] == self.0[6] {
            return self.0[2];
        }

        None
    }

    pub fn check_turn(&self) -> GridState {
        let o_count = self
            .0
            .iter()
            .filter(|&gs| gs.is_some_and(|grid| grid == GridState::O))
            .count();

        let x_count = self
            .0
            .iter()
            .filter(|&gs| gs.is_some_and(|grid| grid == GridState::X))
            .count();

        if o_count <= x_count {
            GridState::O
        } else {
            GridState::X
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::new()
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for (count, grid) in self.0.into_iter().enumerate() {
            match grid {
                Some(gs) => match gs {
                    GridState::O => output = format!("{}{}", output, "o"),
                    GridState::X => output = format!("{}{}", output, "x"),
                },
                None => output = format!("{}{}", output, " "),
            }
            if count == 2 || count == 5 {
                output = format!("{}\n", output);
            }
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game_state() {
        let (gs0, gs1, gs2, gs3, gs4) = get_5_game_state();
        assert_ne!(gs1, gs0);
        assert_eq!(gs1, gs2);
        assert_ne!(gs1, gs3);
        assert_ne!(gs1, gs4);
    }

    #[test]
    fn test_is_finished() {
        let (gs0, _, gs2, _, gs4) = get_5_game_state();

        assert!(!gs0.is_game_finished());
        assert!(!gs2.is_game_finished());
        assert!(gs4.is_game_finished());
    }

    #[test]
    fn test_check_turn() {
        let (gs0, gs1, gs2, gs3, gs4) = get_5_game_state();

        assert_eq!(gs0.check_turn(), GridState::O);
        assert_eq!(gs1.check_turn(), GridState::X);
        assert_eq!(gs2.check_turn(), GridState::X);
        assert_eq!(gs3.check_turn(), GridState::O);
        assert_eq!(gs4.check_turn(), GridState::X);
    }

    #[test]
    fn test_get_winner() {
        let (gs0, _, _, _, gs4) = get_5_game_state();

        assert!(gs0.get_winner().is_none());
        assert_eq!(gs4.get_winner(), Some(GridState::O));

        let gs5 = GameState([
            Some(GridState::X),
            Some(GridState::X),
            Some(GridState::O),
            Some(GridState::O),
            Some(GridState::O),
            Some(GridState::O),
            Some(GridState::X),
            Some(GridState::O),
            Some(GridState::X),
        ]);

        let gs6 = GameState([
            Some(GridState::X),
            Some(GridState::X),
            Some(GridState::O),
            Some(GridState::O),
            Some(GridState::X),
            Some(GridState::X),
            Some(GridState::O),
            Some(GridState::X),
            Some(GridState::O),
        ]);

        let gs7 = GameState([
            Some(GridState::O),
            Some(GridState::X),
            Some(GridState::X),
            Some(GridState::X),
            Some(GridState::O),
            Some(GridState::O),
            Some(GridState::X),
            Some(GridState::O),
            Some(GridState::O),
        ]);

        assert_eq!(gs5.get_winner(), Some(GridState::O));
        assert_eq!(gs6.get_winner(), Some(GridState::X));
        assert_eq!(gs7.get_winner(), Some(GridState::O));
    }

    fn get_5_game_state() -> (GameState, GameState, GameState, GameState, GameState) {
        let game_state0 = GameState::new();

        let mut game_state1 = GameState::new();
        game_state1.0[1] = Some(GridState::O);
        game_state1.0[4] = Some(GridState::X);
        game_state1.0[7] = Some(GridState::O);

        let mut game_state2 = GameState::new();
        game_state2.0[1] = Some(GridState::O);
        game_state2.0[4] = Some(GridState::X);
        game_state2.0[7] = Some(GridState::O);

        let mut game_state3 = GameState::new();
        game_state3.0[1] = Some(GridState::X);
        game_state3.0[4] = Some(GridState::X);
        game_state3.0[7] = Some(GridState::O);

        let game_state4 = GameState([
            Some(GridState::O),
            Some(GridState::X),
            Some(GridState::X),
            Some(GridState::O),
            Some(GridState::O),
            Some(GridState::X),
            Some(GridState::X),
            Some(GridState::O),
            Some(GridState::O),
        ]);

        (
            game_state0,
            game_state1,
            game_state2,
            game_state3,
            game_state4,
        )
    }
}
