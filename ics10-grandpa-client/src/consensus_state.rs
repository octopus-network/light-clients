use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use serde::Serialize;

pub const GRANDPA_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ConsensusState";
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {
    /// The state trie merkle root that used to verify storage proof
    pub state_root: CommitmentRoot,
}
