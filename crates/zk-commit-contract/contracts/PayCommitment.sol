// SPDX-License-Identifier: Apache-2.0.
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "./libraries/Address.sol";
import "./interfaces/IGroth16Verifier.sol";

contract PayCommitment {
    using Addresses for address;

    address public tokenAddress;
    address public groth16VerifierAddress;

    mapping(bytes32 => uint256) commitmentAmounts;
    mapping(bytes32 => bool) public nullifierHashes;

    uint256 internal constant PUB_IN_AMOUNT_OFFSET = 0;
    uint256 internal constant PUB_IN_COMMITMENT_OFFSET = 1;
    uint256 internal constant PUB_IN_NULLIFIER_HASH_OFFSET = 2;
    uint256 internal constant PUB_IN_RECEIPIENT_OFFSET = 3;

    constructor(address _tokenAddress, address _groth16VerifierAddress) {
        tokenAddress = _tokenAddress;
        groth16VerifierAddress = _groth16VerifierAddress;
    }

    event LogDeposit(address depositor, uint256 amount, bytes32 commitment);
    event LogClaim(
        address receipient,
        uint256 amount,
        bytes32 commitment,
        bytes32 nullifierHash
    );
    /**
    @dev Deposit funds into the contract. 
    @param _amount the amount to deposit
    @param _commitment the tree roort
  */
    function depositERC20(
        uint256 _amount,
        bytes32 _commitment
    ) external payable {
        require(
            msg.value == 0,
            "ETH value is supposed to be 0 for ERC20 deposit"
        );
        if (_amount == 0) return;

        require(commitmentAmounts[_commitment] == 0, "commitment exists");
        IERC20 token = IERC20(tokenAddress);
        uint256 balanceBefore = token.balanceOf(address(this));
        bytes memory callData = abi.encodeWithSelector(
            token.transferFrom.selector,
            msg.sender,
            address(this),
            _amount
        );
        tokenAddress.safeTokenContractCall(callData);
        uint256 balanceAfter = token.balanceOf(address(this));
        require(balanceAfter >= balanceBefore, "OVERFLOW");
        // NOLINTNEXTLINE(incorrect-equality): strict equality needed.
        require(
            balanceAfter == balanceBefore + _amount,
            "INCORRECT_AMOUNT_TRANSFERRED"
        );

        commitmentAmounts[_commitment] = _amount;

        emit LogDeposit(msg.sender, _amount, _commitment);
    }

    /**
    @dev Withdraw a deposit from the contract. `proof` is a zkSNARK proof data, and input is an array of circuit public inputs
    `input` array consists of:
      - amount amount ot token to claim
      - commitment the corresponding commitment root
      - nullifier_hash nullifier hash
      - receipient claimer recipient address
  */
    function withdraw(
        uint[2] calldata _pA,
        uint[2][2] calldata _pB,
        uint[2] calldata _pC,
        uint[4] calldata _pubSignals // amount,commitment, nullifier_hash, receipient
    ) external {
        require(_pubSignals.length >= 4, "incorrect publicInput length");
        uint256 _amount = _pubSignals[PUB_IN_AMOUNT_OFFSET];
        bytes32 _commitment = bytes32(_pubSignals[PUB_IN_COMMITMENT_OFFSET]);
        bytes32 _nullifierHash = bytes32(
            _pubSignals[PUB_IN_NULLIFIER_HASH_OFFSET]
        );
        address receipient = address(
            uint160(_pubSignals[PUB_IN_RECEIPIENT_OFFSET])
        );

        require(
            !nullifierHashes[_nullifierHash],
            "The note has been already spent"
        );
        require(
            commitmentAmounts[_commitment] >= _amount,
            "no enough remaining amount to claim"
        );
        require(
            IGroth16Verifier(groth16VerifierAddress).verifyProof(
                _pA,
                _pB,
                _pC,
                _pubSignals
            ),
            "groth16 verification fail"
        );

        nullifierHashes[_nullifierHash] = true;
        commitmentAmounts[_commitment] =
            commitmentAmounts[_commitment] -
            _amount;
        // perform withdrawal
        IERC20 token = IERC20(tokenAddress);
        bytes memory callData = abi.encodeWithSelector(
            token.transfer.selector,
            receipient,
            _amount
        );
        tokenAddress.safeTokenContractCall(callData);

        emit LogClaim(receipient, _amount, _commitment, _nullifierHash);
    }
}
