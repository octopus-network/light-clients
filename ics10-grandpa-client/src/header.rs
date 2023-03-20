use serde_derive::{Deserialize, Serialize};
pub const GRANDPA_HEADER_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.Header";

/// block header
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Header {}
