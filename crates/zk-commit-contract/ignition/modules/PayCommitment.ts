import { buildModule } from "@nomicfoundation/hardhat-ignition/modules";
import TokenModule from "./Token";
import VerifierModule from "./Groth16Verifier";



const PayCommitment = buildModule("PayCommitment", (m) => {

    const { token } = m.useModule(TokenModule);
    const { verifier } = m.useModule(VerifierModule);
 
    const pay = m.contract("PayCommitment", [token, verifier], {
    });

    return { pay };
});

export default PayCommitment;