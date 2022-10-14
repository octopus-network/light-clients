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
use crate::state_machine::read_proof_check;
use beefy_light_client::{
    commitment::{self, known_payload_ids::MMR_ROOT_ID},
    header, mmr,
};
use codec::{Decode, Encode};
use frame_support::{
    storage::{
        storage_prefix,
        types::{EncodeLikeTuple, KeyGenerator, TupleToEncodedIter},
        Key,
    },
    Blake2_128Concat, StorageHasher,
};
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
use ibc_proto::ics23::commitment_proof::Proof::Exist;
use ibc_proto::protobuf::Protobuf;
use prost::Message;
use serde::{Deserialize, Serialize};
use sp_runtime::traits::BlakeTwo256;
use sp_trie::StorageProof;

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
        Height::new(0, self.latest_height as u64).unwrap()
    }
}

impl Protobuf<RawClientState> for ClientState {}

impl ibc::core::ics02_client::client_state::ClientState for ClientState {
    /// Return the chain identifier which this client is serving (i.e., the client is verifying
    /// consensus states from this chain).
    fn chain_id(&self) -> ChainId {
        self.chain_id.clone()
    }

    /// Type of client associated with this state (eg. Tendermint)
    fn client_type(&self) -> ClientType {
        ClientType::Grandpa
    }

    /// Latest height the client was updated to
    fn latest_height(&self) -> Height {
        // TODO(davirian): need to deal with unwrap
        // this height first variant revision_number how to set.
        Height::new(0, self.latest_height as u64).unwrap()
    }

    /// Frozen height of the client
    fn frozen_height(&self) -> Option<Height> {
        self.frozen_height
    }

    /// Check if the state is expired when `elapsed` time has passed since the latest consensus
    /// state timestamp
    fn expired(&self, elapsed: Duration) -> bool {
        // todo(davirain)
        false
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

    // TODO(davirian): need first todo
    fn initialise(&self, consensus_state: Any) -> Result<Box<dyn ConsensusState>, Ics02Error> {
        todo!()
    }

    // TODO(davirian): need first todo
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
        //TODO(daivirain) in tendermint is todo
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
        // todo(davirian)
        // client_state.verify_height(height)?;

        // TODO(davirian)
        // 2022-07-14 18:21:24.062 TRACE tokio-runtime-worker runtime::pallet-ibc: deliver error  : ICS03 connection error
        //
        // Caused by:
        // 0: the consensus proof verification failed (height: 0-17)
        // 1: verify membership failed!
        //
        // Location:
        // /Users/davirain/.cargo/registry/src/github.com-1ecc6299db9ec823/flex-error-0.4.4/src/tracer_impl/eyre.rs:10:9
        // 2022-07-14 18:21:24.063  INFO tokio-runtime-worker pallet_ibc::pallet: result: []
        // let path = ClientConsensusStatePath {
        //     client_id: client_id.clone(),
        //     epoch: consensus_height.revision_number,
        //     height: consensus_height.revision_height,
        // };
        // let value = expected_consensus_state
        //     .encode_vec()
        //     .map_err(Ics02Error::invalid_any_consensus_state)?;
        //
        // verify_membership(
        //     prefix,
        //     proof,
        //     root,
        //     Path::ClientConsensusState(path),
        //     value,
        // )

        Ok(())
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
        // todo(davirian)
        // client_state.verify_height(height)?;

        let path = ConnectionsPath(connection_id.clone());
        let value = expected_connection_end
            .encode_vec()
            .map_err(Ics02Error::invalid_connection_end)?;

        verify_membership(prefix, proof, root, Path::Connections(path), value)
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
        // todo(daviiran)
        // client_state.verify_height(height)?;

        let path = ChannelEndsPath(port_id.clone(), channel_id.clone());
        let value = expected_channel_end
            .encode_vec()
            .map_err(Ics02Error::invalid_channel_end)?;
        verify_membership(prefix, proof, root, Path::ChannelEnds(path), value)
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
        // todo(davirian)
        // client_state.verify_height(height)?;
        //         , 64, 76, 195, 203, 235, 162, 185, 174, 5, 104, 95, 8, 242, 131, 232, 130, 12, 246, 179, 105, 215, 10, 187, 160, 190, 136, 229, 32, 9, 0, 0, 0, 0, 0, 0, 0, 104, 95, 13, 175, 218, 65, 33, 225, 150, 51, 237, 160, 123, 37, 248, 10, 100, 93, 32, 1, 0, 0, 0, 0, 0, 0, 0, 128, 228, 240, 196, 227, 209, 249, 82, 83, 199, 213, 188, 111, 8, 93, 101, 175, 2, 8, 238, 176, 175, 150, 244, 255, 72, 51, 187, 41, 185, 202, 96, 126, 128, 27, 42, 223, 222, 178, 181, 216, 105, 60, 141, 13, 16, 87, 54, 243, 71, 131, 150, 141, 242, 196, 131, 155, 56, 7, 183, 90, 114, 187, 62, 80, 114, 128, 157, 230, 178, 49, 94, 186, 219, 185, 144, 22, 221, 142, 192, 190, 189, 76, 15, 250, 93, 182, 207, 240, 51, 189, 211, 119, 211, 234, 255, 5, 240, 15]] }
        // 2022-08-22 13:22:48.092 TRACE tokio-runtime-worker ibc-rs: in client_def -- get_storage_via_proof, _storage_keys = [101, 204, 242, 3, 105, 192, 221, 218, 216, 45, 16, 3, 82, 58, 196, 142, 83, 132, 145, 119, 250, 48, 85, 212, 116, 56, 227, 154, 243, 42, 226, 177, 252, 130, 244, 54, 210, 6, 77, 152, 118, 243, 115, 152, 48, 167, 27, 139, 128, 99, 108, 105, 101, 110, 116, 115, 47, 49, 48, 45, 103, 114, 97, 110, 100, 112, 97, 45, 48, 47, 99, 108, 105, 101, 110, 116, 83, 116, 97, 116, 101]
        // 2022-08-22 13:22:48.092 TRACE tokio-runtime-worker ibc-rs: in client_def -- get_storage_via_proof, storage_result = [10, 40, 47, 105, 98, 99, 46, 108, 105, 103, 104, 116, 99, 108, 105, 101, 110, 116, 115, 46, 103, 114, 97, 110, 100, 112, 97, 46, 118, 49, 46, 67, 108, 105, 101, 110, 116, 83, 116, 97, 116, 101, 18, 51, 10, 5, 105, 98, 99, 45, 49, 16, 2, 26, 0, 34, 0, 42, 36, 16, 1, 26, 32, 174, 180, 122, 38, 147, 147, 41, 127, 75, 10, 60, 156, 156, 253, 0, 199, 164, 25, 82, 85, 39, 76, 243, 157, 131, 218, 188, 47, 204, 159, 243, 215], storage_name="ClientStates"
        // 2022-08-22 13:22:48.092 TRACE tokio-runtime-worker runtime::pallet-ibc: deliver error  : ICS03 connection error

        // Caused by:
        //    0: the client state proof verification failed for client id 10-grandpa-0
        //    1: verify membership failed!

        // Location:
        //     /Users/davirain/.cargo/registry/src/github.com-1ecc6299db9ec823/flex-error-0.4.4/src/tracer_impl/eyre.rs:10:9
        // 2022-08-22 13

        // todo(davirian)
        // let path = ClientStatePath(client_id.clone());
        // let value = expected_client_state
        // .encode_vec()
        // .map_err(Ics02Error::invalid_any_client_state)?;

        // verify_membership(prefix, proof, root, Path::ClientState(path), value)
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
        let commitment_path = CommitmentsPath {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            sequence,
        };

        verify_membership(
            connection_end.counterparty().prefix(),
            proof,
            root,
            Path::Commitments(commitment_path),
            commitment.into_vec(),
        )
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
        // TODO(davirian)
        // client_state.verify_height(height)?;
        // verify_delay_passed(ctx, height, connection_end)?;

        let ack_path = AcksPath {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            sequence,
        };
        verify_membership(
            connection_end.counterparty().prefix(),
            proof,
            root,
            Path::Acks(ack_path),
            ack.into_vec(),
        )
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
        // todo(davirian)
        // client_state.verify_height(height)?;
        // verify_delay_passed(ctx, height, connection_end)?;

        let mut seq_bytes = Vec::new();
        Message::encode(&u64::from(sequence), &mut seq_bytes).expect("buffer size too small");

        let seq_path = SeqRecvsPath(port_id.clone(), channel_id.clone());
        verify_membership(
            connection_end.counterparty().prefix(),
            proof,
            root,
            Path::SeqRecvs(seq_path),
            seq_bytes,
        )
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
        // todo(davirain) we need add
        // client_state.verify_height(height)?;
        // verify_delay_passed(ctx, height, connection_end)?;

        // TODO(davirian)
        // let receipt_path = ReceiptsPath {
        //     port_id: port_id.clone(),
        //     channel_id: *channel_id,
        //     sequence,
        // };
        //
        // verify_non_membership(
        //     connection_end.counterparty().prefix(),
        //     proof,
        //     root,
        //     receipt_path.into(),
        // )

        Ok(())
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

fn verify_membership(
    prefix: &CommitmentPrefix,
    proof: &CommitmentProofBytes,
    root: &CommitmentRoot,
    path: Path,
    value: Vec<u8>,
) -> Result<(), Ics02Error> {
    // TODO(we need prefix)??
    // let merkle_path = apply_prefix(prefix, vec![path.into().to_string()]);

    let (key, storage_name) = match Path::from(path) {
        Path::ClientType(_) => unimplemented!(),
        Path::ClientState(value) => (value.to_string().as_bytes().to_vec(), "ClientStates"),
        Path::ClientConsensusState(value) => {
            (value.to_string().as_bytes().to_vec(), "ConsensusStates")
        }
        Path::ClientConnections(_) => unimplemented!(),
        Path::Connections(value) => (value.to_string().as_bytes().to_vec(), "Connections"),
        Path::Ports(_) => unimplemented!(),
        Path::ChannelEnds(value) => (value.to_string().as_bytes().to_vec(), "Channels"),
        Path::SeqSends(_) => unimplemented!(),
        Path::SeqRecvs(value) => (value.to_string().as_bytes().to_vec(), "NextSequenceRecv"),
        Path::SeqAcks(_) => unimplemented!(),
        Path::Commitments(value) => (value.to_string().as_bytes().to_vec(), "PacketCommitment"),
        Path::Acks(value) => (value.to_string().as_bytes().to_vec(), "Acknowledgements"),
        Path::Receipts(value) => (value.to_string().as_bytes().to_vec(), "PacketReceipt"),
        Path::Upgrade(_) => unimplemented!(),
    };

    let storage_result = get_storage_via_proof(root, proof, key, storage_name)?;

    if storage_result != value {
        Err(Ics02Error::client_specific(
            "verify_membership_error".to_string(),
        ))
    } else {
        Ok(())
    }
}

fn verify_non_membership(
    prefix: &CommitmentPrefix,
    proof: &CommitmentProofBytes,
    root: &CommitmentRoot,
    path: Path,
) -> Result<(), Ics02Error> {
    // TODO(we need prefix)??
    // let merkle_path = apply_prefix(prefix, vec![path.into().to_string()]);

    let (key, storage_name) = match Path::from(path) {
        Path::ClientType(_) => unimplemented!(),
        Path::ClientState(value) => (value.to_string().as_bytes().to_vec(), "ClientStates"),
        Path::ClientConsensusState(value) => {
            (value.to_string().as_bytes().to_vec(), "ConsensusStates")
        }
        Path::ClientConnections(_) => unimplemented!(),
        Path::Connections(value) => (value.to_string().as_bytes().to_vec(), "Connections"),
        Path::Ports(_) => unimplemented!(),
        Path::ChannelEnds(value) => (value.to_string().as_bytes().to_vec(), "Channels"),
        Path::SeqSends(_) => unimplemented!(),
        Path::SeqRecvs(value) => (value.to_string().as_bytes().to_vec(), "NextSequenceRecv"),
        Path::SeqAcks(_) => unimplemented!(),
        Path::Commitments(value) => (value.to_string().as_bytes().to_vec(), "PacketCommitment"),
        Path::Acks(value) => (value.to_string().as_bytes().to_vec(), "Acknowledgements"),
        Path::Receipts(value) => (value.to_string().as_bytes().to_vec(), "PacketReceipt"),
        Path::Upgrade(_) => unimplemented!(),
    };

    let storage_result = get_storage_via_proof(root, proof, key, storage_name);

    // TODO(is or not correct)
    if storage_result.is_err() {
        Ok(())
    } else {
        Err(Ics02Error::client_specific(
            "verify_no_membership_error".to_string(),
        ))
    }
}

/// Reconstruct on-chain storage value by proof, key(path), and state root
fn get_storage_via_proof(
    root: &CommitmentRoot,
    proof: &CommitmentProofBytes,
    keys: Vec<u8>,
    storage_name: &str,
) -> Result<Vec<u8>, Ics02Error> {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ReadProofU8 {
        pub at: String,
        pub proof: Vec<Vec<u8>>,
    }

    let merkel_proof = RawMerkleProof::try_from(proof.clone())
        .map_err(|e| Ics02Error::client_specific(e.to_string()))?;
    let merkel_proof = merkel_proof.proofs[0]
        .proof
        .clone()
        .ok_or(Ics02Error::client_specific("empty_proof".to_string()))?;
    let storage_proof = match merkel_proof {
        Exist(exist_proof) => {
            let proof_str = String::from_utf8(exist_proof.value)
                .map_err(|e| Ics02Error::client_specific(e.to_string()))?;
            let storage_proof: ReadProofU8 = serde_json::from_str(&proof_str)
                .map_err(|e| Ics02Error::client_specific(e.to_string()))?;
            storage_proof
        }
        _ => unimplemented!(),
    };

    let storage_keys = storage_map_final_key(keys, storage_name);
    let state_root = root.clone().into_vec();
    let state_root = vector_to_array::<u8, 32>(state_root);

    let storage_result = read_proof_check::<BlakeTwo256>(
        sp_core::H256::from(state_root),
        StorageProof::new(storage_proof.proof),
        &storage_keys,
    )
    .map_err(|e| Ics02Error::client_specific(e.to_string()))?
    .ok_or(Ics02Error::client_specific("empty_proof".to_string()))?;

    let storage_result = <Vec<u8> as Decode>::decode(&mut &storage_result[..])
        .map_err(|e| Ics02Error::client_specific(e.to_string()))?;

    Ok(storage_result)
}

/// Calculate the storage's final key
fn storage_map_final_key(key: Vec<u8>, storage_name: &str) -> Vec<u8> {
    // Migrate from: https://github.com/paritytech/substrate/blob/32b71896df8a832e7c139a842e46710e4d3f70cd/frame/support/src/storage/generator/map.rs?_pjax=%23js-repo-pjax-container%2C%20div%5Bitemtype%3D%22http%3A%2F%2Fschema.org%2FSoftwareSourceCode%22%5D%20main%2C%20%5Bdata-pjax-container%5D#L66
    let key_hashed: &[u8] = &Blake2_128Concat::hash(&Encode::encode(&key));
    let storage_prefix = storage_prefix("Ibc".as_bytes(), storage_name.as_bytes());
    let mut final_key = Vec::with_capacity(storage_prefix.len() + key_hashed.as_ref().len());
    final_key.extend_from_slice(&storage_prefix);
    final_key.extend_from_slice(key_hashed.as_ref());

    final_key
}

/// A hashing function for packet commitments
fn hash(value: String) -> String {
    let r = sp_io::hashing::sha2_256(value.as_bytes());

    let mut tmp = String::new();
    for item in r.iter() {
        tmp.push_str(&format!("{:02x}", item));
    }

    tmp
}

fn verify_header(
    block_header: BlockHeader,
    mmr_leaf: Vec<u8>,
    mmr_leaf_proof: Vec<u8>,
) -> Result<(), Ics02Error> {
    let block_number = block_header.block_number as u64;
    let mmr_leaf: Vec<u8> = Decode::decode(&mut &mmr_leaf[..])
        .map_err(|e| Ics02Error::client_specific(e.to_string()))?;

    let mmr_leaf: mmr::MmrLeaf =
        Decode::decode(&mut &*mmr_leaf).map_err(|e| Ics02Error::client_specific(e.to_string()))?;

    // check mmr leaf
    if mmr_leaf.parent_number_and_hash.1.is_empty() {
        return Err(Ics02Error::client_specific(
            "empty mmr leaf parent hash mmr root".to_string(),
        ));
    }

    // decode mmr leaf proof
    let mmr_leaf_proof = beefy_light_client::mmr::MmrLeafProof::decode(&mut &mmr_leaf_proof[..])
        .map_err(|e| Ics02Error::client_specific(e.to_string()))?;

    if block_number > mmr_leaf_proof.leaf_count {
        return Err(Ics02Error::client_specific(
            "invalid mmr leaf proof".to_string(),
        ));
    }

    // verfiy block header
    if block_header.parent_hash != mmr_leaf.parent_number_and_hash.1.to_vec() {
        return Err(Ics02Error::client_specific(
            "header hash not match".to_string(),
        ));
    }

    Ok(())
}

fn vector_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}
