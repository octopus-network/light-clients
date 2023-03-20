use serde::Serialize;

pub const GRANDPA_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ConsensusState";
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {}
