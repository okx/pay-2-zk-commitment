import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.21",
  networks: {
    anvil: {
      url: "http://127.0.0.1:8545",
      initialDate: Date().toString(),
    },
    hardhat: {
      initialDate: Date().toString(),
    },
    mainnet: {
      url: process.env.NODE_MAINNET_URL || "",
      timeout: 20000,
    },

  }

};

export default config;
