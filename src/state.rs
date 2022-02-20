mod loading_state;
mod main_state;
mod state;

use loading_state::LoadingState;
use main_state::MainState;
use state::StateTrait;

pub use main_state::MainStateViewState;
pub use state::State;
