use serde::{Deserialize, Serialize};

use crate::types::RequestId;

/// This object defines the criteria used to request a managed bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct KeyboardButtonRequestManagedBot {
    /// Identifier of the request. Must be unique within the message.
    pub request_id: RequestId,

    /// The suggested name for the bot.
    pub suggested_name: Option<String>,

    /// The suggested username for the bot.
    pub suggested_username: Option<String>,
}

impl KeyboardButtonRequestManagedBot {
    /// Creates a new [`KeyboardButtonRequestManagedBot`].
    pub fn new(request_id: RequestId) -> Self {
        Self { request_id, suggested_name: None, suggested_username: None }
    }

    /// Setter for the `suggested_name` field.
    #[must_use]
    pub fn suggested_name(mut self, v: impl Into<String>) -> Self {
        self.suggested_name = Some(v.into());
        self
    }

    /// Setter for the `suggested_username` field.
    #[must_use]
    pub fn suggested_username(mut self, v: impl Into<String>) -> Self {
        self.suggested_username = Some(v.into());
        self
    }
}
