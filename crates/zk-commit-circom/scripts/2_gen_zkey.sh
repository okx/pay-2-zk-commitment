#!/bin/bash
set -e
source .env

echo "****GENERATING ZKEY 0****"
# If failed: https://hackmd.io/@yisun/BkT0RS87q
start=$(date +%s)
# ${NODE_PATH} ${NODE_PARAMS} ${SNARKJS_PATH} groth16 setup $CIRCUIT_NAME.r1cs ${POT_PATH} "$CIRCUIT_NAME"_0.zkey
nohup ${NODE_PATH} ${NODE_PARAMS} ${SNARKJS_PATH} zkey new $CIRCUIT_NAME.r1cs ${POT_PATH} ${ZKEY_OUT_PATH}/${CIRCUIT_NAME}_0.zkey -v > zkey0.out 2>&1 &
end=$(date +%s)
echo "DONE ($((end - start))s)"

echo "****CONTRIBUTE TO PHASE 2 CEREMONY****"
start=$(date +%s)
${NODE_PATH} ${NODE_PARAMS} ${SNARKJS_PATH} zkey contribute -verbose ${ZKEY_OUT_PATH}/${CIRCUIT_NAME}_0.zkey ${ZKEY_OUT_PATH}/${CIRCUIT_NAME}.zkey -n="First phase2 contribution" -e="some random text" > contribute.out
end=$(date +%s)
echo "DONE ($((end - start))s)"

echo "****VERIFYING FINAL ZKEY (SKIP FOR TESTING)****"
start=$(date +%s)
${NODE_PATH} ${NODE_PARAMS} ${SNARKJS_PATH} zkey verify -verbose "$CIRCUIT_NAME".r1cs ${POT_PATH} ${ZKEY_OUT_PATH}/${CIRCUIT_NAME}.zkey > verify.out
end=$(date +%s)
echo "DONE ($((end - start))s)"