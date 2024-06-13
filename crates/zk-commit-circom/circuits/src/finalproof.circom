pragma circom 2.1.0;

include "./poseidon.circom";
include "../../node_modules/circomlib/circuits/bitify.circom";

// if s == 0 returns [in[0], in[1]]
// if s == 1 returns [in[1], in[0]]
template DualMux(N) {
  signal input in[2][N];
  signal input s;
  signal output out[2][N];

  s * (1 - s) === 0;

  for (var i = 0; i < N; i++) {
    out[0][i] <== (in[1][i] - in[0][i])*s + in[0][i];
    out[1][i] <== (in[0][i] - in[1][i])*s + in[1][i];
  }
}

template ZeroCapacity(N) {
  signal output capacity[N];

  for (var i = 0; i < N; i++){
    capacity[i] <== 0;
  }
}

template FinalProofChecker(DEPTH, LEAF_ARRAY_LEN) {
  signal input siblings[DEPTH][4];
  signal input index;
  signal input leaf[LEAF_ARRAY_LEN];

  signal output root[4];

  component sbits = Num2Bits(DEPTH);
  sbits.in <== index;

  component lh = HashNoPad_GL(LEAF_ARRAY_LEN, 4);
  lh.capacity <== ZeroCapacity(4)();
  lh.in <== leaf;

  component hasher[DEPTH];
  component mux[DEPTH];
  signal hashes[DEPTH][4];

  for (var i = 0; i < DEPTH; i++) {
    mux[i] = DualMux(4);
    mux[i].in[0] <== i == 0 ? lh.out : hasher[i - 1].out;
    mux[i].in[1] <== siblings[i];
    mux[i].s <== sbits.out[i];

    hasher[i] = Poseidon_GL(4);
    hasher[i].capacity <== ZeroCapacity(4)();
    for (var k = 0; k < 4; k++) {
      hasher[i].in[k] <== mux[i].out[0][k];
      hasher[i].in[k + 4] <== mux[i].out[1][k];
    }
  }

  root <== hasher[DEPTH - 1].out;
}