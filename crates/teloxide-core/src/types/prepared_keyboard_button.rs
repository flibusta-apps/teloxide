use serde::{Deserialize, Serialize};

/// Describes a keyboard button prepared for a Mini App.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PreparedKeyboardButton {
    /// Unique identifier of the prepared button.
    pub id: String,
}
