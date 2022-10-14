use alloc::string::ToString;
use core::cmp::Ordering;
use core::fmt::{Display, Error as FmtError, Formatter};

use crate::error::Error;
use beefy_merkle_tree::Hash;
use bytes::Buf;
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::error::Error as Ics02Error;
use ibc::core::ics24_host::identifier::ChainId;
use ibc::timestamp::Timestamp;
use ibc::utils::pretty::{PrettySignedHeader, PrettyValidatorSet};
use ibc::Height;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::grandpa::v1::Header as RawHeader;
use ibc_proto::protobuf::Protobuf;
use prost::Message;
use serde_derive::{Deserialize, Serialize};

use super::client_state_help::BlockHeader;
use super::client_state_help::MmrLeaf;
use super::client_state_help::MmrLeafProof;
use super::client_state_help::MmrRoot;
use super::client_state_help::SignedCommitment;
use super::client_state_help::ValidatorMerkleProof;
use tendermint::time::Time;
use tendermint_proto::google::protobuf as tpb;

pub const GRANDPA_HEADER_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.Header";

/// block header
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Header {
    pub block_header: BlockHeader,
    pub mmr_root: MmrRoot,
    //// timestamp
    pub timestamp: Time,
}
impl Default for Header {
    fn default() -> Self {
        Self {
            block_header: BlockHeader::default(),
            mmr_root: MmrRoot::default(),
            // todo(davirian): detail with unwrap
            timestamp: Time::from_unix_timestamp(0, 0).unwrap(),
        }
    }
}
impl Header {
    pub fn new(block_header: BlockHeader, mmr_root: MmrRoot, timestamp: Time) -> Self {
        Self {
            block_header,
            mmr_root,
            timestamp,
        }
    }

    pub fn hash(&self) -> Result<Hash, Error> {
        self.block_header.hash()
    }

    pub fn height(&self) -> Height {
        Height::new(8888, self.block_header.block_number as u64).unwrap()
    }
}

impl ibc::core::ics02_client::header::Header for Header {
     /// The type of client (eg. Tendermint)
    fn client_type(&self) -> ClientType {
         todo!()
     }

    /// The height of the consensus state
    fn height(&self) -> Height {
        // todo(davirian): unwrap
        Height::new(0, self.block_header.block_number as u64).unwrap()
     }

    /// The timestamp of the consensus state
    fn timestamp(&self) -> Timestamp {
        self.timestamp.clone().into()
    }
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        let ibc_proto::google::protobuf::Timestamp { seconds, nanos } = raw
        .timestamp
        .ok_or_else(|| Error::invalid_raw_header("missing timestamp".into()))?;

    let proto_timestamp = tpb::Timestamp { seconds, nanos };
    let timestamp = proto_timestamp
        .try_into()
        .map_err(|e| Error::invalid_raw_header(format!("invalid timestamp: {}", e)))?;

    Ok(Self {
        block_header: raw
            .block_header
            .ok_or_else(Error::empty_block_header)?
            .into(),
        mmr_root: raw.mmr_root.ok_or_else(Error::empty_mmr_root)?.try_into()?,
        timestamp: timestamp,
    })
    }
}

pub fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
    RawHeader::decode(buf).map_err(Error::decode)?.try_into()
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        let tpb::Timestamp { seconds, nanos } = value.timestamp.into();
        let timestamp = ibc_proto::google::protobuf::Timestamp { seconds, nanos };
        let mmr_root = value.mmr_root.try_into().unwrap();
        RawHeader {
            block_header: Some(value.block_header.into()),
            mmr_root: Some(mmr_root),
            timestamp: Some(timestamp),
        }
    }
}

impl Protobuf<Any> for Header {}

impl TryFrom<Any> for Header {
    type Error = Ics02Error;

    fn try_from(raw: Any) -> Result<Self, Ics02Error> {
        use core::ops::Deref;

        fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
            RawHeader::decode(buf).map_err(Error::decode)?.try_into()
        }

        match raw.type_url.as_str() {
            GRANDPA_HEADER_TYPE_URL => Ok(decode_header(raw.value.deref()).unwrap()),
            _ => Err(Ics02Error::unknown_header_type(raw.type_url)),
        }
    }
}

impl From<Header> for Any {
    fn from(header: Header) -> Self {
        Any {
            type_url: GRANDPA_HEADER_TYPE_URL.to_string(),
            value: Protobuf::<RawHeader>::encode_vec(&header)
            .expect("encoding to `Any` from `GpHeader`"),
        }
    }
}
