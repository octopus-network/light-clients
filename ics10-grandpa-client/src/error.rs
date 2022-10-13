use flex_error::DisplayOnly;
use flex_error::{define_error, TraceError};
use ibc::core::ics02_client::error::Error as Ics02Error;
use ibc::core::ics24_host::error::ValidationError;
use ibc::core::ics24_host::identifier::ClientId;
use ibc::timestamp::{Timestamp, TimestampOverflowError};
use ibc::Height;

define_error! {
    #[derive(Debug, PartialEq, Eq)]
    Error{
        Dummy
            |_| { format_args!("dummy error") },

        Decode
        [ TraceError<prost::DecodeError> ]
            | _ | { "decode error" },

        MissingLatestHeight
            | _ | { "missing latest height" },

        MissingHeight
            | _ | { "missing height" },

        InvalidChainIdentifier
        [ ValidationError ]
            | _ | { "invalid chain identifier" },

        MissingFrozenHeight
            | _ | { "missing frozen height" },

        InvalidRawConsensusState
        { reason: String }
            | _ | { "invalid raw client consensus state" },

        InvalidRawMisbehaviour
        { reason: String }
            | _ | { "invalid raw misbehaviour" },

        InvalidRawHeader
        { reason: String }
            | _ | { "invalid raw header" },

        Encode
        [ TraceError<prost::EncodeError> ]
            | _ | { "encode error" },

        EmptyCommitment
            | _ | { "empty commitment"},

        InvalidSignedCommitment
            | _ | { "invalid Signed Commitment" },

        InvalidValidatorMerkleProof
            | _ | { "invalid Validator Merkle Proof" },

        InvalidMmrLeaf
            | _ | { "invalid Mmr Leaf" },

        InvalidMmrLeafProof
            | _ | { "invalid Mmr Lead Proof" },

        InvalidCommitment
            | _ | { "Invalid commitment"},

        InvalidStorageProof
            | _ | { "invalid storage Proof" },

        GetStorageByProofErr
        {
            e: String,
        }
            | e | {
                format_args!("failed to get storage by proof: {0}", e)
            },

        InvalidChainId
            | _ | { "invalid chain id" },

        EmptyBlockHeader
            | _ | { "empty block header" },

        EmptyLatestCommitment
            | _ | { "empty latest commitment" },

        EmptyValidatorSet
            | _ | { "empty validator set" },

        EmptyMmrLeaf
            | _ | { "empty mmr leaf" },

        EmptyMmrLeafProof
            | _ | { "empty mmr leaf proof" },

        EmptyMmrRoot
            | _ | { "empty mmr root" },

        EmptyTimestamp
            | _ | { "empty timestamp" },

        EmptySignedCommitment
            | _ | { "empty signed commitment" },

        InvalidConvertHash
            | _ | { "invalid convert hash" },

        InvalidConvertSignature
            | _ | { "invalid convert signature" },

        EmptyParentNumberAndHash
            | _ | { "empty parent and hash" },

        EmptyBeefyNextAuthoritySet
            | _ | { "empty next authority set" },

        InvalidCodecDecode
        [ DisplayOnly<codec::Error> ]
            |_| { "invalid codec decode" },

        InvalidMmrRoot
        { reason: String }
            |e| { format_args!("invalid mmr root, failed basic validation: {}", e.reason) },
    }
}
