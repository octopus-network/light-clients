use alloc::string::ToString;
use core::cmp::Ordering;
use core::fmt::{Display, Error as FmtError, Formatter};

use bytes::Buf;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::lightclients::tendermint::v1::Header as RawHeader;
use ibc_proto::protobuf::Protobuf;
use prost::Message;
use serde_derive::{Deserialize, Serialize};
//use tendermint::block::signed_header::SignedHeader;
//use tendermint::validator::Set as ValidatorSet;

use crate::error::Error;
use ibc::core::ics02_client::client_type::ClientType;
use ibc::core::ics02_client::error::Error as Ics02Error;
use ibc::core::ics24_host::identifier::ChainId;
use ibc::timestamp::Timestamp;
use ibc::utils::pretty::{PrettySignedHeader, PrettyValidatorSet};
use ibc::Height;

pub const GRANDPA_HEADER_TYPE_URL: &str = "/ibc.lightclients.grandpa.v1.Header";

/// block header
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Header {
    //    pub block_header: BlockHeader,
    //    pub mmr_root: MmrRoot,
    // timestamp
    //    pub timestamp: Time,
}
