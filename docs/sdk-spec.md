# sdk spec
1. setup
```rust
struct Distribution {
    amount: u64,
    secret: [u64; 4]
}

struct CommitTree {

}
fn set_up(distrubutions: Vec<Distribution>, n: u32) -> CommitTree; //for payer to setup; return commitment; provided by rust sdk
fn deposit_usdc(commitment: [u8;32], amount: u64, private_key: [u8;32]) // deposit to contract; rust sdk will compose tx and return to mobile end; mobile end will do rpc
fn gen_plonky2_proof(secret: [u64; 4], amount: u64, sibling: Vec<u64>, index: u32, commitment: [u8;32] ) -> String; // return proof in json string; provided by rust sdk
fn request_groth16_proof(plonky2_proof: String) -> String// return groth16 proof in json string; Http client implemented by mobile; server provided by ZKP team
fn claim(groth16_proof: String, private_key: [u8;32]);// for receiver to claim, rust sdk will compose tx and return to mobile end; mobile end will do rpc