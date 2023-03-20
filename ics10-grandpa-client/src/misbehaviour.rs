use serde::{Deserialize, Serialize};

use crate::header::Header;
use ibc::core::ics24_host::identifier::ClientId;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Misbehaviour {
    pub client_id: ClientId,
    pub header1: Header,
    pub header2: Header,
}
