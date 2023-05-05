use rand::{self, Rng};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GridState {
    O,
    X,
}
pub fn random_grid_state() -> GridState {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        GridState::O
    } else {
        GridState::X
    }
}
