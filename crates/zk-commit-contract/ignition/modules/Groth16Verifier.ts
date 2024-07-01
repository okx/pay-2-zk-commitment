import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";

const VerifierModule = buildModule("VerifierModule", (m) => {

  const verifier = m.contract("Groth16Verifier", [], {
  });

  return { verifier };
});

export default VerifierModule;