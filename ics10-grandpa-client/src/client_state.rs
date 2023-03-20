use ibc::core::ics24_host::identifier::ChainId;
use ibc::Height;
use serde::{Deserialize, Serialize};

pub const GRANDPA_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ClientState";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ClientState {
    pub chain_id: ChainId,
    /// block_number is height?
    pub latest_height: u32,
    /// Block height when the client was frozen due to a misbehaviour
    pub frozen_height: Option<Height>,
}
