use plonky2::field::types::Field;

use zk_commit_core::{types::F, utils::AmountSecretPairing as CoreAmountSecretPairing};

#[derive(uniffi::Record)]
pub struct AmountSecretPairing {
    pub amount: u64,
    pub secret: u64,
}

impl AmountSecretPairing {
    pub fn from_core(amount_secret_pairing: &CoreAmountSecretPairing) -> Self {
        Self { amount: amount_secret_pairing.amount.0, secret: amount_secret_pairing.secret.0 }
    }

    pub fn to_core(&self) -> CoreAmountSecretPairing {
        CoreAmountSecretPairing {
            amount: F::from_canonical_u64(self.amount),
            secret: F::from_canonical_u64(self.secret),
        }
    }
}
