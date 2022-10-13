use ibc_proto::ibc::lightclients::tendermint::v1::Misbehaviour as RawMisbehaviour;
use ibc_proto::protobuf::Protobuf;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::header::Header;
use ibc::core::ics24_host::identifier::ClientId;
use ibc::Height;

#[derive(Clone, Debug, PartialEq)]
pub struct Misbehaviour {
    pub client_id: ClientId,
    pub header1: Header,
    pub header2: Header,
}
