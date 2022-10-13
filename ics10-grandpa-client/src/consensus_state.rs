use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::tendermint::v1::ConsensusState as RawConsensusState;
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::header::Header;
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::error::Error as Ics02Error;
use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use ibc::timestamp::Timestamp;
use super::client_state_help::Commitment;

pub const GRANDPA_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ConsensusState";
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {
    ///commitment: Option<Commitment>,used to verify mmr proof
    pub commitment: Commitment,
    /// The state trie merkle root that used to verify storage proof
    pub state_root: CommitmentRoot,
}

impl ConsensusState {
    pub fn new(commitment: Commitment, state_root: CommitmentRoot) -> Self {
        Self {
            commitment,
            state_root,
        }
    }
}

impl Default for ConsensusState {
    fn default() -> Self {
        Self {
            commitment: Commitment::default(),
            state_root: CommitmentRoot::from(vec![0]),
        }
    }
}
impl Protobuf<RawConsensusState> for ConsensusState {}

impl ibc::core::ics02_client::consensus_state::ConsensusState for ConsensusState {
    /// Type of client associated with this consensus state (eg. Tendermint)
    fn client_type(&self) -> ClientType {
        todo!()
    }

    /// Commitment root of the consensus state, which is used for key-value pair verification.
    fn root(&self) -> &CommitmentRoot {
        todo!()
    }

    /// The timestamp of the consensus state
    fn timestamp(&self) -> Timestamp {
        todo!()
    }
}

impl TryFrom<RawConsensusState> for ConsensusState {
    type Error = Error;

    fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<ConsensusState> for RawConsensusState {
    fn from(value: ConsensusState) -> Self {
        todo!()
    }
}

impl From<Header> for ConsensusState {
    fn from(header: Header) -> Self {
        todo!()
    }
}

impl Protobuf<Any> for ConsensusState {}

impl TryFrom<Any> for ConsensusState {
    type Error = Ics02Error;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use core::ops::Deref;
        use prost::Message;

        fn decode_consensus_state<B: Buf>(buf: B) -> Result<ConsensusState, Error> {
            RawConsensusState::decode(buf)
            .map_err(Error::decode)?
            .try_into()
        }

        match raw.type_url.as_str() {
            GRANDPA_CONSENSUS_STATE_TYPE_URL => {
                // todo
                Ok(decode_consensus_state(raw.value.deref()).unwrap())
            }
            _ => Err(Ics02Error::unknown_consensus_state_type(raw.type_url)),
        }
    }
}

impl From<ConsensusState> for Any {
    fn from(consensus_state: ConsensusState) -> Self {
        Any {
            type_url: GRANDPA_CONSENSUS_STATE_TYPE_URL.to_string(),
            value: Protobuf::<RawConsensusState>::encode_vec(&consensus_state)
            .expect("encoding to `Any` from `TmConsensusState`"),
        }
    }
}
