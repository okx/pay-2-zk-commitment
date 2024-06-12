## To Pay

The payer generates N tuple of $$(a_i,null_i, x_i)$$ where $$a_i$$ is the amount to pay, $$x_i$$is the secret to spend the amount, $$null_i$$is the nullifier to avoid double spend. The total amount to spend would be $$A = \sum_{i=0}^{n-1}a_i$$. The payer has to deposit the amount of $$A$$usdc in the contract. The payer computes the Poseidon hash of each pair $$H_i = Hash(a_i,null_i, x_i), i \in [0,n-1]$$; and uses those values as leaves to build a merkle tree. Payer sends each tuple to a receiver by means of a red pocket; when receiver opens the red pocket, it will receive $$(a_i, null_i, x_i)$$. Or it could be sent by means of QR code scan (we start by QR code)
Unclaimed amount could be transferred to the depositor after some fixed duration of time

## To receive
Upon receiving $$(a_i, null_i, x_i)$$, the receiver uses $$x_i$$as private input, $$a_i, null_i$$as public input and generates a plonky2 ZK proof for a circuit with below constraints
- The prover knows a merkle path to the certain leaf and preimage to this leaf
- User supplies address to withdraw (as a public input), to avoid replay attack
The receiver passes the plonky2 proof to OKX service, OKX service uses this proof to generate groth16 proof (as a way to reduce gas cost); and submits to the onchain contract to spend the amount.

## On Smart Contract
- Check Groth16 proof is valid
- Check that nullifier is not spent
- Save nullifier
- Release funds
