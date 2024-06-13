import {ethers} from "hardhat";
import "@nomiclabs/hardhat-etherscan";
import {assert, expect} from "chai";

describe("Groth16", function () {
    it("Should return true when proof is correct", async function () {
        const verifierFactory = await ethers.getContractFactory("Groth16Verifier");
        const verifier = await verifierFactory.deploy();
        await verifier.deployed();

        const fs = require("fs");
        let text = fs.readFileSync("./test/data/l1_verifier_input.txt").toString();
        text = text.replace(/\s+/g, '');
        text = text.replace(/\[+/g, '');
        text = text.replace(/]+/g, '');
        text = text.replace(/"+/g, '');
        const p = text.split(",");
        console.log('p length', p.length);
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
        expect(ret).to.equal(true);
    });
});
