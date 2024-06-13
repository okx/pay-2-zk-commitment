// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;


library Addresses {
    function isContract(address account) internal view returns (bool) {
        uint256 size;
        assembly {
            size := extcodesize(account)
        }
        return size > 0;
    }

    function performEthTransfer(address recipient, uint256 amount) internal {
        (bool success, ) = recipient.call{value: amount}(''); // NOLINT: low-level-calls.
        require(success, 'ETH_TRANSFER_FAILED');
    }

    /*
      Safe wrapper around ERC20/ERC721 calls.
      This is required because many deployed ERC20 contracts don't return a value.
      See https://github.com/ethereum/solidity/issues/4116.
    */
    function safeTokenContractCall(
        address tokenAddress,
        bytes memory callData
    ) internal {
        require(isContract(tokenAddress), 'BAD_TOKEN_ADDRESS');
        // NOLINTNEXTLINE: low-level-calls.
        (bool success, bytes memory returndata) = tokenAddress.call(callData);
        require(success, string(returndata));

        if (returndata.length > 0) {
            require(abi.decode(returndata, (bool)), 'TOKEN_OPERATION_FAILED');
        }
    }

    /*
      Validates that the passed contract address is of a real contract,
      and that its id hash (as infered fromn identify()) matched the expected one.
    */
    function validateContractId(
        address contractAddress,
        bytes32 expectedIdHash
    ) internal {
        require(isContract(contractAddress), 'ADDRESS_NOT_CONTRACT');
        (bool success, bytes memory returndata) = contractAddress.call( // NOLINT: low-level-calls.
            abi.encodeWithSignature('identify()')
        );
        require(success, 'FAILED_TO_IDENTIFY_CONTRACT');
        string memory realContractId = abi.decode(returndata, (string));
        require(
            keccak256(abi.encodePacked(realContractId)) == expectedIdHash,
            'UNEXPECTED_CONTRACT_IDENTIFIER'
        );
    }
}