#!/bin/bash
set -e
source .env

######################### GEN PROOF USING LOCAL RAPIDSNARK #########################################################################################
## take plonky2 final proof(proof.json) as input, generate witness file witness.wtns
echo "****WITNESS GENERATION****"
start=$(date +%s)
./${CIRCUIT_NAME}_cpp/${CIRCUIT_NAME} ${INPUT_PATH} ./witness.wtns
end=$(date +%s)
echo "DONE ($((end - start))s)"

## run rapidsnark in standalone mode
## take witness.wtns as input, generate proof.json and public.json
echo "****GENERATING PROOF****"
start=$(date +%s)
${RAPIDSNARK_PATH} ${ZKEY_OUT_PATH}/${CIRCUIT_NAME}.zkey ./witness.wtns proof.json public.json
# ${NODE_PATH} ${SNARKJS_PATH} groth16 prove "$CIRCUIT_NAME".zkey ./witness.wtns proof.json public.json
end=$(date +%s)
echo "DONE ($((end - start))s)"

######################### VERIFY USING SNARKJS #########################################################################################
## take zkey as input, export verification_key.json
echo "****EXPORTING VKEY****"
start=$(date +%s)
${NODE_PATH} ${SNARKJS_PATH} zkey export verificationkey ${ZKEY_OUT_PATH}/${CIRCUIT_NAME}.zkey verification_key.json -v
end=$(date +%s)
echo "DONE ($((end - start))s)"

## verify final proof
## input: public.json, proof.json & verification_key
echo "****VERIFYING PROOF****"
start=$(date +%s)
${NODE_PATH} ${SNARKJS_PATH} groth16 verify verification_key.json public.json proof.json -v
end=$(date +%s)
echo "DONE ($((end - start))s)"

######################### VERIFY USING HARDHAT Contract #########################################################################################
${NODE_PATH} ${SNARKJS_PATH} zkey export solidityverifier ${ZKEY_OUT_PATH}/${CIRCUIT_NAME}.zkey Groth16Verifier.sol
mv Groth16Verifier.sol ../contracts/Groth16Verifier.sol
${NODE_PATH} ${SNARKJS_PATH} zkesc public.json proof.json > ../test/data/l1_verifier_input.txt

cd .. && npx hardhat test