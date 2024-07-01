use alloy::{
    primitives::{Address, FixedBytes, U256},
    sol,
    sol_types::SolCall,
};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TestERC20,
    "static/abis/TestERC20.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    PayCommitment,
    "static/abis/PayCommitment.json"
);

use crate::groth16::Groth16ProofWithPublicData;

use std::str::FromStr;

const PUB_LENGTH: usize = 13;

pub struct Blockchain {}

impl Blockchain {
    // get token approval contract call data; to be signed by mobile wallet
    pub fn approve_token_call_data(spender: &str, amount: u64) -> Vec<u8> {
        let spender = Address::from_str(spender).unwrap();
        let input = TestERC20::approveCall { spender, amount: U256::from(amount) };
        let data = input.abi_encode();
        data
    }

    pub fn get_deposit_token_call_data(amount: u64, commitment: FixedBytes<32>) -> Vec<u8> {
        let call = PayCommitment::depositERC20Call {
            _amount: U256::from(amount),
            _commitment: commitment,
        };
        let data = call.abi_encode();
        data
    }

    pub fn get_claim_token_call_data(pwi: &Groth16ProofWithPublicData) -> Vec<u8> {
        let pub_signals = pwi
            .public_data
            .iter()
            .map(|x| U256::from_str_radix(x, 10).unwrap())
            .collect::<Vec<U256>>();

        let mut pub_signals_array = [U256::default(); PUB_LENGTH];
        for (i, item) in pub_signals.iter().take(PUB_LENGTH).enumerate() {
            pub_signals_array[i] = *item;
        }

        let input = PayCommitment::withdrawCall {
            _pA: [
                U256::from_str_radix(&pwi.proof.pi_a[0], 10).unwrap(),
                U256::from_str_radix(&pwi.proof.pi_a[1], 10).unwrap(),
            ],
            _pB: [
                [
                    U256::from_str_radix(&pwi.proof.pi_b[0][1], 10).unwrap(),
                    U256::from_str_radix(&pwi.proof.pi_b[0][0], 10).unwrap(),
                ],
                [
                    U256::from_str_radix(&pwi.proof.pi_b[1][1], 10).unwrap(),
                    U256::from_str_radix(&pwi.proof.pi_b[1][0], 10).unwrap(),
                ],
            ],
            _pC: [
                U256::from_str_radix(&pwi.proof.pi_c[0], 10).unwrap(),
                U256::from_str_radix(&pwi.proof.pi_c[1], 10).unwrap(),
            ],
            _pubSignals: pub_signals_array,
        };
        let data = input.abi_encode();
        data
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_blockchain() {
        let call_data =
            Blockchain::approve_token_call_data("0x1234567890123456789012345678901234567890", 100);
        let out = hex::encode(call_data);
        assert_eq!(out, "095ea7b300000000000000000000000012345678901234567890123456789012345678900000000000000000000000000000000000000000000000000000000000000064");
    }
}
