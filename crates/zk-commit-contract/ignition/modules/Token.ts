import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const TokenModule = buildModule("TokenModule", (m) => {
    const token = m.contract("TestERC20", [1000000000000], {});
    return { token };
});

export default TokenModule;