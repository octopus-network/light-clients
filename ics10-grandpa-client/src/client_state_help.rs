use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use beefy_light_client::commitment::known_payload_ids::MMR_ROOT_ID;
use beefy_light_client::commitment::Commitment as BeefyCommitment;
use beefy_light_client::commitment::Payload;
use beefy_light_client::mmr::MmrLeafVersion;
use beefy_light_client::Hash;
use beefy_light_client::ValidatorMerkleProof as BeefyValidatorMerkleProof;
use beefy_light_client::{commitment, header, mmr, validator_set};
use codec::{Decode, Encode};
use core::convert::TryFrom;
use flex_error::{define_error, DisplayOnly, TraceError};
use ibc::Height;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct Commitment {}
