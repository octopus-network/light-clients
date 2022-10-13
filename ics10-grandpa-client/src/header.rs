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
use ibc_proto::ibc::lightclients::tendermint::v1::Header as RawHeader;
use ibc_proto::protobuf::Protobuf;
use prost::Message;
use serde_derive::{Deserialize, Serialize};

use super::client_state_help::BlockHeader;
use super::client_state_help::MmrLeaf;
use super::client_state_help::MmrLeafProof;
use super::client_state_help::MmrRoot;
use super::client_state_help::SignedCommitment;
use super::client_state_help::ValidatorMerkleProof;

pub const GRANDPA_HEADER_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.Header";

/// block header
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Header {
    pub block_header: BlockHeader,
    pub mmr_root: MmrRoot,
    //// timestamp
    //    pub timestamp: Timestamp,
}
impl Default for Header {
    fn default() -> Self {
        Self {
            block_header: BlockHeader::default(),
            mmr_root: MmrRoot::default(),
            //            timestamp: Timestamp::from_unix_timestamp(0, 0).unwrap(),
        }
    }
}
impl Header {
    pub fn new(block_header: BlockHeader, mmr_root: MmrRoot) -> Self {
        Self {
            block_header,
            mmr_root,
            //            timestamp,
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
         todo!()
     }

    /// The timestamp of the consensus state
    fn timestamp(&self) -> Timestamp {
         todo!()
     }
}

impl Protobuf<RawHeader> for Header {}

impl TryFrom<RawHeader> for Header {
    type Error = Error;

    fn try_from(raw: RawHeader) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub fn decode_header<B: Buf>(buf: B) -> Result<Header, Error> {
    RawHeader::decode(buf).map_err(Error::decode)?.try_into()
}

impl From<Header> for RawHeader {
    fn from(value: Header) -> Self {
        todo!()
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
