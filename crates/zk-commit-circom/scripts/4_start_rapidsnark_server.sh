#!/bin/bash
set -e
source .env

# rapidsnark server needs below to generate witness
cp ${CIRCUIT_NAME}_cpp/${CIRCUIT_NAME}.dat ${RS_PATH}/build/${CIRCUIT_NAME}.dat
cp ${CIRCUIT_NAME}_cpp/${CIRCUIT_NAME} ${RS_PATH}/build/${CIRCUIT_NAME}

export LD_LIBRARY_PATH=depends/pistache/build/src
cd ${RS_PATH}
./build/proverServer 3000 ${ZKEY_OUT_PATH}/${CIRCUIT_NAME}.zkey