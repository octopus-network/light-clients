use ibc::core::ics02_client::context::ClientReader;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics04_channel::commitment::{AcknowledgementCommitment, PacketCommitment};
use ibc::core::ics04_channel::packet::Sequence;
use ibc::core::ics23_commitment::commitment::{
    CommitmentPrefix, CommitmentProofBytes, CommitmentRoot,
};
use ibc::core::ics23_commitment::merkle::{apply_prefix, MerkleProof};
use ibc::core::ics24_host::path::{
    AcksPath, ChannelEndsPath, ClientConsensusStatePath, ClientStatePath, CommitmentsPath,
    ConnectionsPath, ReceiptsPath, SeqRecvsPath,
};
use ibc::core::ics24_host::Path;
//use crate::prelude::*;

use core::convert::{TryFrom, TryInto};
use core::time::Duration;

use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::Height as RawHeight;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as RawTmClientState;
use ibc_proto::protobuf::Protobuf;
use prost::Message;
use serde::{Deserialize, Serialize};
//use tendermint_light_client_verifier::options::Options;
//use tendermint_light_client_verifier::types::{TrustedBlockState, UntrustedBlockState};
//use tendermint_light_client_verifier::{ProdVerifier, Verdict, Verifier};

use crate::consensus_state::ConsensusState as GpConsensusState;
use crate::error::Error;
use crate::header::Header as GpHeader;
use ibc::core::ics02_client::client_state::{
    ClientState as Ics2ClientState, UpdatedState, UpgradeOptions as CoreUpgradeOptions,
};
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::consensus_state::ConsensusState;
use ibc::core::ics02_client::error::{Error as Ics02Error, ErrorDetail as Ics02ErrorDetail};
use ibc::core::ics02_client::trust_threshold::TrustThreshold;
use ibc::core::ics04_channel::context::ChannelReader;
use ibc::core::ics23_commitment::specs::ProofSpecs;
use ibc::core::ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId};
use ibc::timestamp::{Timestamp, ZERO_DURATION};
use ibc::Height;

pub const GRANDPA_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ClientState";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ClientState {
    pub chain_id: ChainId,
    /// block_number is height?
    pub latest_height: u32,
    /// Block height when the client was frozen due to a misbehaviour
    pub frozen_height: Option<Height>,
    //    pub latest_commitment: Commitment,
    //    pub validator_set: ValidatorSet,
}
