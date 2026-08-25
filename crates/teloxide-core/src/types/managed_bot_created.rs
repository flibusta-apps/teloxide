use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object represents a managed bot that was created.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ManagedBotCreated {
    /// The created bot.
    pub bot: User,
}
