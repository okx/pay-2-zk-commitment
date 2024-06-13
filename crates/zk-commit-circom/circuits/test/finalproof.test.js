const path = require("path");
const proof = require("./data/final_proof.json");

const wasm_tester = require("circom_tester").wasm;

describe("Final Proof Verification Test", function () {
  let circuit;

  this.timeout(10000000);

  before(async () => {
    circuit = await wasm_tester(path.join(__dirname, "circuits", "finalproof.test.circom"), {
        // verbose: true,
    });
  });

  it("Should pass", async () => {
    const input = proof;

    const w = await circuit.calculateWitness(input, true);

    await circuit.checkConstraints(w);
  });
});
