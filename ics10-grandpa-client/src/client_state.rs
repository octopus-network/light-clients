use core::convert::{TryFrom, TryInto};
use core::str::FromStr;
use core::time::Duration;
use ibc::core::ics02_client::client_state::UpgradeOptions;
use ibc::core::ics02_client::context::ClientReader;
use ibc::core::ics03_connection::connection::ConnectionEnd;
use ibc::core::ics04_channel::channel::ChannelEnd;
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
use ibc_proto::ibc::lightclients::grandpa::v1::ClientState as RawClientState;

use super::client_state_help::BlockHeader;
use super::client_state_help::Commitment;
use super::client_state_help::ValidatorSet;
use crate::consensus_state::ConsensusState as GpConsensusState;
use crate::error::Error;
use crate::header::Header;
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
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::Height as RawHeight;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as RawMerkleProof;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as RawTmClientState;
use ibc_proto::protobuf::Protobuf;
use prost::Message;
use serde::{Deserialize, Serialize};

pub const GRANDPA_CLIENT_STATE_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.ClientState";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ClientState {
    pub chain_id: ChainId,
    /// block_number is height?
    pub latest_height: u32,
    /// Block height when the client was frozen due to a misbehaviour
    pub frozen_height: Option<Height>,
    pub latest_commitment: Commitment,
    pub validator_set: ValidatorSet,
}

impl ClientState {
    pub fn new(
        chain_id: ChainId,
        latest_height: u32,
        latest_commitment: Commitment,
        validator_set: ValidatorSet,
    ) -> Result<Self, Error> {
        let client_state = ClientState {
            chain_id,
            latest_height,
            latest_commitment,
            validator_set,
            frozen_height: None,
        };

        Ok(client_state)
    }

    pub fn with_header(self, h: Header) -> Self {
        // TODO: Clarify which fields should update.
        ClientState {
            latest_height: h.height().revision_number() as u32,
            ..self
        }
    }

    /// Get the refresh time to ensure the state does not expire
    pub fn refresh_time(&self) -> Option<Duration> {
        //TODO
        Some(Duration::new(3, 0))
    }

    /// Check if the state is expired when `elapsed` time has passed since the latest consensus
    /// state timestamp
    pub fn expired(&self, elapsed: Duration) -> bool {
        //TODO
        false
    }

    pub fn latest_height(&self) -> Height {
        Height::new(8888, self.latest_height as u64).unwrap()
    }
}

impl Protobuf<RawClientState> for ClientState {}

impl ibc::core::ics02_client::client_state::ClientState for ClientState {
    /// Return the chain identifier which this client is serving (i.e., the client is verifying
    /// consensus states from this chain).
    fn chain_id(&self) -> ChainId {
        todo!()
    }

    /// Type of client associated with this state (eg. Tendermint)
    fn client_type(&self) -> ClientType {
        todo!()
    }

    /// Latest height the client was updated to
    fn latest_height(&self) -> Height {
        todo!()
    }

    /// Frozen height of the client
    fn frozen_height(&self) -> Option<Height> {
        todo!()
    }

    /// Check if the state is expired when `elapsed` time has passed since the latest consensus
    /// state timestamp
    fn expired(&self, elapsed: Duration) -> bool {
        todo!()
    }

    /// Helper function to verify the upgrade client procedure.
    /// Resets all fields except the blockchain-specific ones,
    /// and updates the given fields.
    fn upgrade(
        &mut self,
        upgrade_height: Height,
        upgrade_options: &dyn UpgradeOptions,
        chain_id: ChainId,
    ) {
        todo!()
    }

    fn initialise(&self, consensus_state: Any) -> Result<Box<dyn ConsensusState>, Ics02Error> {
        todo!()
    }

    fn check_header_and_update_state(
        &self,
        ctx: &dyn ClientReader,
        client_id: ClientId,
        header: Any,
    ) -> Result<UpdatedState, Ics02Error> {
        todo!()
    }

    fn verify_upgrade_and_update_state(
        &self,
        consensus_state: Any,
        proof_upgrade_client: RawMerkleProof,
        proof_upgrade_consensus_state: RawMerkleProof,
    ) -> Result<UpdatedState, Ics02Error> {
        todo!()
    }

    /// Verification functions as specified in:
    /// <https://github.com/cosmos/ibc/tree/master/spec/core/ics-002-client-semantics>
    ///
    /// Verify a `proof` that the consensus state of a given client (at height `consensus_height`)
    /// matches the input `consensus_state`. The parameter `counterparty_height` represent the
    /// height of the counterparty chain that this proof assumes (i.e., the height at which this
    /// proof was computed).
    #[allow(clippy::too_many_arguments)]
    fn verify_client_consensus_state(
        &self,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        client_id: &ClientId,
        consensus_height: Height,
        expected_consensus_state: &dyn ConsensusState,
    ) -> Result<(), Ics02Error> {
        todo!()
    }

    /// Verify a `proof` that a connection state matches that of the input `connection_end`.
    #[allow(clippy::too_many_arguments)]
    fn verify_connection_state(
        &self,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        connection_id: &ConnectionId,
        expected_connection_end: &ConnectionEnd,
    ) -> Result<(), Ics02Error> {
        todo!()
    }

    /// Verify a `proof` that a channel state matches that of the input `channel_end`.
    #[allow(clippy::too_many_arguments)]
    fn verify_channel_state(
        &self,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        port_id: &PortId,
        channel_id: &ChannelId,
        expected_channel_end: &ChannelEnd,
    ) -> Result<(), Ics02Error> {
        todo!()
    }

    /// Verify the client state for this chain that it is stored on the counterparty chain.
    #[allow(clippy::too_many_arguments)]
    fn verify_client_full_state(
        &self,
        height: Height,
        prefix: &CommitmentPrefix,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        client_id: &ClientId,
        expected_client_state: Any,
    ) -> Result<(), Ics02Error> {
        todo!()
    }

    /// Verify a `proof` that a packet has been commited.
    #[allow(clippy::too_many_arguments)]
    fn verify_packet_data(
        &self,
        ctx: &dyn ChannelReader,
        height: Height,
        connection_end: &ConnectionEnd,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
        commitment: PacketCommitment,
    ) -> Result<(), Ics02Error> {
        todo!()
    }

    /// Verify a `proof` that a packet has been commited.
    #[allow(clippy::too_many_arguments)]
    fn verify_packet_acknowledgement(
        &self,
        ctx: &dyn ChannelReader,
        height: Height,
        connection_end: &ConnectionEnd,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
        ack: AcknowledgementCommitment,
    ) -> Result<(), Ics02Error> {
        todo!()
    }

    /// Verify a `proof` that of the next_seq_received.
    #[allow(clippy::too_many_arguments)]
    fn verify_next_sequence_recv(
        &self,
        ctx: &dyn ChannelReader,
        height: Height,
        connection_end: &ConnectionEnd,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<(), Ics02Error> {
        todo!()
    }

    /// Verify a `proof` that a packet has not been received.
    #[allow(clippy::too_many_arguments)]
    fn verify_packet_receipt_absence(
        &self,
        ctx: &dyn ChannelReader,
        height: Height,
        connection_end: &ConnectionEnd,
        proof: &CommitmentProofBytes,
        root: &CommitmentRoot,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<(), Ics02Error> {
        todo!()
    }
}

impl TryFrom<RawClientState> for ClientState {
    type Error = Error;

    fn try_from(raw: RawClientState) -> Result<Self, Self::Error> {
        let frozen_height = raw
            .frozen_height
            .and_then(|raw_height| raw_height.try_into().ok());

        Ok(Self {
            chain_id: ChainId::from_str(raw.chain_id.as_str())
                .map_err(|_| Error::invalid_chain_id())?,
            latest_height: raw.latest_height,
            frozen_height,
            latest_commitment: raw
                .latest_commitment
                .ok_or_else(Error::empty_latest_commitment)?
                .into(),
            validator_set: raw
                .validator_set
                .ok_or_else(Error::empty_validator_set)?
                .into(),
        })
    }
}

impl From<ClientState> for RawClientState {
    fn from(value: ClientState) -> Self {
        Self {
            chain_id: value.chain_id.to_string(),
            latest_height: value.latest_height,
            frozen_height: Some(value.frozen_height.map(|height| height.into()).unwrap_or(
                RawHeight {
                    revision_number: 0,
                    revision_height: 0,
                },
            )),
            latest_commitment: Some(value.latest_commitment.into()),
            validator_set: Some(value.validator_set.into()),
        }
    }
}

impl Protobuf<Any> for ClientState {}

impl TryFrom<Any> for ClientState {
    type Error = Ics02Error;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use core::ops::Deref;

        fn decode_client_state<B: Buf>(buf: B) -> Result<ClientState, Error> {
            RawClientState::decode(buf)
                .map_err(Error::decode)?
                .try_into()
        }

        match raw.type_url.as_str() {
            GRANDPA_CLIENT_STATE_TYPE_URL => {
                //todo
                Ok(decode_client_state(raw.value.deref()).unwrap())
            }
            _ => Err(Ics02Error::unknown_client_state_type(raw.type_url)),
        }
    }
}

impl From<ClientState> for Any {
    fn from(client_state: ClientState) -> Self {
        Any {
            type_url: GRANDPA_CLIENT_STATE_TYPE_URL.to_string(),
            value: Protobuf::<RawClientState>::encode_vec(&client_state)
                .expect("encoding to `Any` from `GpClientState`"),
        }
    }
}
