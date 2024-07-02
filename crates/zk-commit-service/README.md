# zk-commitment-service
service accepting mobile genrated plonky2 proof

## run
```
cargo run -p zk-commit-service
export GATE_WAY_URL=http://127.0.0.1:8080
curl -X GET ${GATE_WAY_URL}
curl -X POST \
    -F "file=@path/to/your/file.bin" \
    --max-time 600 \
    ${GATE_WAY_URL}/api/v1/wrap/groth16
```