use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::tendermint::v1::ConsensusState as RawConsensusState;
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};
//use tendermint::{hash::Algorithm, time::Time, Hash};
//use tendermint_proto::google::protobuf as tpb;

use crate::error::Error;
use crate::header::Header;
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::error::Error as Ics02Error;
use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use ibc::timestamp::Timestamp;

pub const GRANDPA_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ConsensusState";

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {
    //commitment: Option<Commitment>,used to verify mmr proof
    // pub commitment: Commitment,
    /// The state trie merkle root that used to verify storage proof
    pub state_root: CommitmentRoot,
    // timestamp
    //pub timestamp: Time,
}
