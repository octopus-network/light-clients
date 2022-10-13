use flex_error::{define_error, TraceError};

use ibc::core::ics02_client::error::Error as Ics02Error;
use ibc::core::ics24_host::error::ValidationError;
use ibc::core::ics24_host::identifier::ClientId;
use ibc::timestamp::{Timestamp, TimestampOverflowError};

use ibc::Height;
//use tendermint::account::Id;
//use tendermint::hash::Hash;
//use tendermint::Error as TendermintError;
//use tendermint_light_client_verifier::errors::VerificationErrorDetail as LightClientErrorDetail;

define_error! {
    #[derive(Debug, PartialEq, Eq)]
    Error{
        Dummy
            |_| { format_args!("dummy error") },
    }
}
