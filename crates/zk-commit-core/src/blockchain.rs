use crate::{
    bindings::{paycommitment::PayCommitment, testerc20::TestERC20},
    groth16::Groth16ProofWithPublicData,
};
use ethers::{
    prelude::*,
    providers::{Http, Provider},
};

use std::{str::FromStr, sync::Arc};

const PUB_LENGTH: usize = 4;

pub struct Blockchain {
    pub token: TestERC20<Provider<Http>>,
    pub pay_commitment: PayCommitment<Provider<Http>>,
}

impl Blockchain {
    pub fn new(rpc_url: &str, token_addr: &str, pay_commitment_addr: &str) -> Self {
        let provider = Arc::new(Provider::<Http>::try_from(rpc_url).unwrap());

        let pay_commitment =
            PayCommitment::new(Address::from_str(pay_commitment_addr).unwrap(), provider.clone());
        let token = TestERC20::new(Address::from_str(token_addr).unwrap(), provider.clone());
        Self { pay_commitment, token }
    }

    // get token approval contract call data; to be signed by mobile wallet
    pub fn approve_token_call_data(&self, spender: &str, amount: u64) -> Bytes {
        let spender = Address::from_str(spender).unwrap();
        let calldata = self.token.approve(spender, U256::from(amount)).calldata().unwrap();
        calldata
    }

    pub fn get_deposit_token_call_data(&self, amount: u64, commitment: [u8; 32]) -> Bytes {
        let calldata =
            self.pay_commitment.deposit_erc20(U256::from(amount), commitment).calldata().unwrap();
        calldata
    }

    pub fn get_claim_token_call_data(&self, pwi: &Groth16ProofWithPublicData) -> Bytes {
        let pub_signals = pwi
            .public_data
            .iter()
            .map(|x| U256::from_str_radix(x, 10).unwrap())
            .collect::<Vec<U256>>();

        let mut pub_signals_array = [U256::default(); PUB_LENGTH];
        for (i, item) in pub_signals.iter().take(PUB_LENGTH).enumerate() {
            pub_signals_array[i] = *item;
        }
        let calldata = self
            .pay_commitment
            .withdraw(
                [
                    U256::from_str_radix(&pwi.proof.pi_a[0], 10).unwrap(),
                    U256::from_str_radix(&pwi.proof.pi_a[1], 10).unwrap(),
                ],
                [
                    [
                        U256::from_str_radix(&pwi.proof.pi_b[0][1], 10).unwrap(),
                        U256::from_str_radix(&pwi.proof.pi_b[0][0], 10).unwrap(),
                    ],
                    [
                        U256::from_str_radix(&pwi.proof.pi_b[1][1], 10).unwrap(),
                        U256::from_str_radix(&pwi.proof.pi_b[1][0], 10).unwrap(),
                    ],
                ],
                [
                    U256::from_str_radix(&pwi.proof.pi_c[0], 10).unwrap(),
                    U256::from_str_radix(&pwi.proof.pi_c[1], 10).unwrap(),
                ],
                pub_signals_array,
            )
            .calldata()
            .unwrap();
        calldata
    }
}
