use near_groth16_verifier::{self, G1Point, G2Point, Verifier};
use verifier::get_verifier;
use hex;

pub mod verifier;
pub mod risczero;
pub mod utils;

fn main() {
    let verifier = get_verifier();
    
    // need to get the recipet digest and the seal
    let seal = hex::decode("028ddfd058a2e818bf25fb9d710359042fb5e7200c28105944b2b9c8c3e2af4d1cb77b9730fcb22b0c182c791678d6895ac436cab7fb99e15e773d2a97e4e1d526ee8e31b492c6ebfc66418ffa8c0e5ed0b84ca66d5a35174640bc01c8f779a218d3be19a74b952697f7fe73520e2d12795309c67e9e255d5a890bb7aa415bbe277d73cc765337c91c2f65c8f4617144bc4bc76bf95f01d49328e27de3820a6910a2bdc481bb93ddaf6d688223abad6a53245be7676e505e702ff0b4cb5d5d6a22c30923e5db696f5c94dc9a3ad736d909eec7da10ce00476b31f5d678ae05dc1e293717c3dafe22a942f693ebfb4c75eb5c08f75dd1e45b74d9f455860088b2").unwrap();
    let post_state_digest = hex::decode("861ffac3fc1be3ec222139c10070c668284c85b2e8906cc2546b889c6ca8fe2d").unwrap();
    let journal_output = hex::decode("4f8ad5ce1d0fc577d04d618880b8c77c9aced63740ca43b708f32425a95b11b7").unwrap();

    let image_id = hex::decode("fecdec02d41f87fa668ffd9b363ea5c9ac973bf9537153ecca58fe76af2ab77f").unwrap();
    // let input = ;
    // let proof = ;
    // verifier.verify(input, proof);
    let zero_bytes_32 = hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    println!("{:?}", zero_bytes_32);
}
