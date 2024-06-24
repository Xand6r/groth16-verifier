use ::easy_hasher::easy_hasher::raw_sha256;
use eth_encode_packed::{
    ethabi::{ethereum_types::U256, Address},
    SolidityDataType, TakeLastXBytes,
};

use easy_hasher::easy_hasher;

#[derive(Debug)]
pub enum SystemExitCode {
    Halted,
    Paused,
    SystemSplit,
}


#[derive(Debug)]
pub struct Output{
    pub journal_digest: Vec<u8>,
    pub assumptions_digest: Vec<u8>,
}

impl Output{
    fn digest(self) -> Vec<u8>{
        let tag_digest = easy_hasher::sha256(&"risc0.Output".to_string()).to_vec();

        let input = vec![
            SolidityDataType::Bytes(&tag_digest),
            SolidityDataType::Bytes(&self.journal_digest),
            SolidityDataType::Bytes(&self.assumptions_digest),
            SolidityDataType::NumberWithShift(U256::from(2)<<8, TakeLastXBytes(16))
        ];
        let (bytes,_) = eth_encode_packed::abi::encode_packed(&input);
        let out = raw_sha256(bytes).to_vec();

        out
    }
}

#[derive(Debug)]
pub struct Receipt{
    pub seal: Vec<u8>,
    pub claim: ReceiptClaim,
}

#[derive(Debug)]
pub struct ExitCode {
    pub system:SystemExitCode,
    pub user: u8,
}

#[derive(Debug)]
pub struct ReceiptClaim {
    pub pre_state_digest: Vec<u8>,
    pub post_state_digest: Vec<u8>,
    pub exit_code: ExitCode,
    pub input: Vec<u8>,
    pub output: Vec<u8>,
}

impl ReceiptClaim{
    fn digest(self) -> Vec<u8> {
        let tag_digest = easy_hasher::sha256(&"risc0.ReceiptClaim".to_string()).to_vec();

        let input = vec![
            SolidityDataType::Bytes(&tag_digest),
            SolidityDataType::Bytes(&self.input),
            SolidityDataType::Bytes(&self.pre_state_digest),
            SolidityDataType::Bytes(&self.post_state_digest),
            SolidityDataType::Bytes(&self.output),
            SolidityDataType::NumberWithShift(U256::from(self.exit_code.system as u16)<<24, TakeLastXBytes(32)),
            SolidityDataType::NumberWithShift(U256::from(self.exit_code.user)<<24, TakeLastXBytes(32)),
            SolidityDataType::NumberWithShift(U256::from(4)<<8, TakeLastXBytes(16)),
        ];
        let (bytes, _) = eth_encode_packed::abi::encode_packed(&input);
        let out = raw_sha256(bytes).to_vec();
        
        out
    }
}


#[cfg(test)]
mod tests {
    use self::easy_hasher::raw_sha256;
    use eth_encode_packed::ethabi::{encode, Token};

    use super::*;

    // #[test]
    // fn test_output_digest(){
    //     let journal_output = hex::decode("4f8ad5ce1d0fc577d04d618880b8c77c9aced63740ca43b708f32425a95b11b7").unwrap();

    //     let journal_digest = raw_sha256(encode(&[
    //         Token::Bytes(journal_output)
    //     ])).to_vec();

    //     let output = Output {
    //         journal_digest: journal_digest.clone(),
    //         assumptions_digest: hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap()
    //     };
    //     let output_digest = output.digest();
    //     let expected_output_digest = "7e76559b3f58036f3e6d302930e1eccda6d667425fe3c2d9d69f913204e04ec2";
        
    //     assert_eq!(
    //         hex::encode(output_digest),
    //         expected_output_digest
    //     );
    // }

    #[test]
    fn test_receipt_digest(){
        let image_id = hex::decode("fecdec02d41f87fa668ffd9b363ea5c9ac973bf9537153ecca58fe76af2ab77f").unwrap();

        let seal = hex::decode("028ddfd058a2e818bf25fb9d710359042fb5e7200c28105944b2b9c8c3e2af4d1cb77b9730fcb22b0c182c791678d6895ac436cab7fb99e15e773d2a97e4e1d526ee8e31b492c6ebfc66418ffa8c0e5ed0b84ca66d5a35174640bc01c8f779a218d3be19a74b952697f7fe73520e2d12795309c67e9e255d5a890bb7aa415bbe277d73cc765337c91c2f65c8f4617144bc4bc76bf95f01d49328e27de3820a6910a2bdc481bb93ddaf6d688223abad6a53245be7676e505e702ff0b4cb5d5d6a22c30923e5db696f5c94dc9a3ad736d909eec7da10ce00476b31f5d678ae05dc1e293717c3dafe22a942f693ebfb4c75eb5c08f75dd1e45b74d9f455860088b2").unwrap();
        let post_state_digest = hex::decode("861ffac3fc1be3ec222139c10070c668284c85b2e8906cc2546b889c6ca8fe2d").unwrap();
        let journal_output = hex::decode("4f8ad5ce1d0fc577d04d618880b8c77c9aced63740ca43b708f32425a95b11b7").unwrap();

        let journal_digest = raw_sha256(encode(&[
            Token::Bytes(journal_output)
        ])).to_vec();


        let receipt = Receipt{
            seal,
            claim: ReceiptClaim{
                pre_state_digest: image_id,
                post_state_digest: post_state_digest,
                exit_code: ExitCode { system: SystemExitCode::Halted, user: 0 },
                input: hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
                output: Output {
                    journal_digest: journal_digest.clone(),
                    assumptions_digest: hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap()
                }.digest()
            }
        };

        let digest = receipt.claim.digest();
        let expected_digest = "77be7b053e70c2d8456d676ff1c0070b4e9dcd6522556ca6dcf4220bd5f6b2ab";
        assert_eq!(
            hex::encode(digest),
            expected_digest
        );

        
    }
}