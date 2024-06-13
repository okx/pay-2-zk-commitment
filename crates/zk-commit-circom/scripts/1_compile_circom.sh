#!/bin/bash
set -e
source .env

echo "****COMPILING CIRCUIT****"
start=$(date +%s)
circom ${CIRCUIT_PATH} --r1cs --sym --c
end=$(date +%s)
echo "DONE ($((end - start))s)"

echo "****COMPILING WITNESS GENERATOR****"
start=$(date +%s)
cd ${CIRCUIT_NAME}_cpp && make -j && cd ..
end=$(date +%s)
echo "DONE ($((end - start))s)"
