use constants::{IMAGE_ID_STRING, JOURNAL_OUTPUT_STRING, SEAL_STRING};
use easy_hasher::easy_hasher::raw_sha256;
use eth_encode_packed::ethabi::{encode, ethereum_types::U256, Token};
use near::verify_integrity;
use near_groth16_verifier::{self, G1Point, G2Point, Verifier};
use risczero::{Receipt, ReceiptClaim};
use verifier::get_verifier;
use hex;

pub mod verifier;
pub mod risczero;
pub mod utils;
pub mod constants;
pub mod near;

fn main() {
    let image_id = hex::decode(IMAGE_ID_STRING).unwrap();
    let seal = hex::decode(SEAL_STRING).unwrap();
    let journal_output = hex::decode(JOURNAL_OUTPUT_STRING).unwrap();

    let journal_digest = raw_sha256(encode(&[Token::Bytes(journal_output)])).to_vec();

    let receipt = Receipt {
        seal: seal.clone(),
        claim: ReceiptClaim::ok(image_id, journal_digest),
    };
    let claim_digest = receipt.claim.digest();

    verify_integrity(seal, claim_digest)
}
