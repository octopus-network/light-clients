use super::client_state_help::Commitment;
use crate::error::Error;
use crate::header::Header;
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::error::Error as Ics02Error;
use ibc::core::ics23_commitment::commitment::CommitmentRoot;
use ibc::timestamp::Timestamp;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::grandpa::v1::ConsensusState as RawConsensusState;
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};
use tendermint::time::Time;
use tendermint_proto::google::protobuf as tpb;

pub const GRANDPA_CONSENSUS_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ConsensusState";
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConsensusState {
    ///commitment: Option<Commitment>,used to verify mmr proof
    pub commitment: Commitment,
    /// The state trie merkle root that used to verify storage proof
    pub state_root: CommitmentRoot,
    /// timestamp
    pub timestamp: Time,
}

impl ConsensusState {
    pub fn new(commitment: Commitment, state_root: CommitmentRoot, timestamp: Time) -> Self {
        Self {
            commitment,
            state_root,
            timestamp,
        }
    }
}

impl Default for ConsensusState {
    fn default() -> Self {
        Self {
            commitment: Commitment::default(),
            state_root: CommitmentRoot::from(vec![0]),
            // todo(davirian) unwrap()
            timestamp: Time::from_unix_timestamp(0, 0).unwrap(),
        }
    }
}
impl Protobuf<RawConsensusState> for ConsensusState {}

impl ibc::core::ics02_client::consensus_state::ConsensusState for ConsensusState {
    /// Type of client associated with this consensus state (eg. Tendermint)
    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    /// Commitment root of the consensus state, which is used for key-value pair verification.
    fn root(&self) -> &CommitmentRoot {
        &self.state_root
    }

    /// The timestamp of the consensus state
    fn timestamp(&self) -> Timestamp {
        self.timestamp.clone().into()
    }
}

impl TryFrom<RawConsensusState> for ConsensusState {
    type Error = Error;

    fn try_from(raw: RawConsensusState) -> Result<Self, Self::Error> {
        let ibc_proto::google::protobuf::Timestamp { seconds, nanos } = raw
            .timestamp
            .ok_or_else(|| Error::invalid_raw_consensus_state("missing timestamp".into()))?;

        let proto_timestamp = tpb::Timestamp { seconds, nanos };

        let timestamp = proto_timestamp
            .try_into()
            .map_err(|e| Error::invalid_raw_consensus_state(format!("invalid timestamp: {}", e)))?;

        Ok(Self {
            commitment: raw
                .commitment
                .ok_or_else(Error::empty_latest_commitment)?
                .into(),
            state_root: raw
                .state_root
                .ok_or_else(|| {
                    Error::invalid_raw_consensus_state("missing commitment root".into())
                })?
                .hash
                .into(),
            timestamp,
        })
    }
}

impl From<ConsensusState> for RawConsensusState {
    fn from(value: ConsensusState) -> Self {
        // FIXME: shunts like this are necessary due to
        // https://github.com/informalsystems/tendermint-rs/issues/1053
        let tpb::Timestamp { seconds, nanos } = value.timestamp.into();
        let timestamp = ibc_proto::google::protobuf::Timestamp { seconds, nanos };

        Self {
            commitment: Some(value.commitment.into()),
            state_root: Some(ibc_proto::ibc::core::commitment::v1::MerkleRoot {
                hash: value.state_root.into_vec(),
            }),
            timestamp: Some(timestamp),
        }
    }
}

impl From<Header> for ConsensusState {
    fn from(header: Header) -> Self {
        Self {
            commitment: header.mmr_root.signed_commitment.commitment.unwrap(),
            state_root: CommitmentRoot::from_bytes(&header.block_header.state_root),
            timestamp: header.timestamp,
        }
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
