use serde::{Deserialize, Serialize};

/// The intended distribution scope of an alert message
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Scope {
    /// For general dissemination to unrestricted audiences
    Public,
    /// For dissemination only to users with a known operational requirement (see `restriction`)
    Restricted,
    /// For dissemination only to specified addresses (see `addresses`)
    Private,
}
