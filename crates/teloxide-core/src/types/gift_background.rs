use serde::{Deserialize, Serialize};

use crate::types::Rgb;

/// This object describes the background of a gift.
///
/// [The official docs](https://core.telegram.org/bots/api#giftbackground).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct GiftBackground {
    /// Center color of the background in RGB format
    pub center_color: Rgb,

    /// Edge color of the background in RGB format
    pub edge_color: Rgb,

    /// Text color of the background in RGB format
    pub text_color: Rgb,
}
