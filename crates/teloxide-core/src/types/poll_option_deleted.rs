use serde::{Deserialize, Serialize};

use crate::types::{MaybeInaccessibleMessage, MessageEntity};

/// This object represents a poll option that was deleted.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct PollOptionDeleted {
    /// The poll message.
    pub poll_message: Option<MaybeInaccessibleMessage>,
    /// Persistent identifier of the option.
    pub option_persistent_id: String,
    /// Text of the option.
    pub option_text: String,
    /// Special entities that appear in the option text.
    pub option_text_entities: Option<Vec<MessageEntity>>,
}
