mod state_transition;
pub use state_transition::State;
pub use state_transition::Msg;

mod prelude {
    pub use crate::model::prelude::*;
    pub use std::sync::Arc;
}
