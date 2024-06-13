import * as ethers from "ethers";

import { Groth16Verifier__factory } from "../typechain-types"

const verify = async () => {
    let provier = new ethers.providers.JsonRpcProvider("http://127.0.0.1:8545");
    let wallet = new ethers.Wallet("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", provier);
    let verifier = Groth16Verifier__factory.connect("0xe7f1725e7734ce288f8367e1bb143e90bb3f0512", wallet
    );


    const fs = require("fs");
    let text = fs.readFileSync("./test/data/l1_verifier_input.txt").toString();
    text = text.replace(/\s+/g, '');
    text = text.replace(/\[+/g, '');
    text = text.replace(/]+/g, '');
    text = text.replace(/"+/g, '');
    const p = text.split(",");
    let public_inputs = [];
    for (let i = 0; i < p.length - 8; i++) {
        public_inputs.push(p[8 + i]);
    }
    let ret = await verifier.verifyProof(
        [p[0], p[1]],
        [[p[2], p[3]], [p[4], p[5]]],
        [p[6], p[7]],
        public_inputs
    );
    console.log(ret);
}

verify();