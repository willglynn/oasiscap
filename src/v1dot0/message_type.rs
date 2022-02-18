use serde::{Deserialize, Serialize};

/// A classification describing the nature of an alert message.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    /// Initial information requiring attention by targeted recipients
    Alert,
    /// Updates and supersedes the earlier message(s) identified in `references`
    Update,
    /// Cancels the earlier message(s) identified in `references`
    Cancel,
    /// Acknowledges receipt and acceptance of the message(s) identified in `references`
    Ack,
    /// Indicates rejection of the message(s) identified in `references`; explanation SHOULD appear in `note`
    Error,
}
