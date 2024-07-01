# zk-commit-contract
the smart contract that do
- proof verification
- asset deposit and transfer

## setup
```
npx hardhat compile # compile contracts
curl -L https://foundry.paradigm.xyz | bash # install foundry
foundryup # install several clis, includes anvil
anvil # starts local blockchain
npx hardhat ignition deploy ./ignition/modules/PayCommitment.ts --network anvil
```

there are three contracts created by order
1. token: 0x5fbdb2315678afecb367f032d93f642f64180aa3
2. verifier: 0xe7f1725e7734ce288f8367e1bb143e90bb3f0512
3. paycommitment: 0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0