use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object represents an update to a managed bot.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ManagedBotUpdated {
    /// The user that manages the bot.
    pub user: User,
    /// The managed bot.
    pub bot: User,
}
