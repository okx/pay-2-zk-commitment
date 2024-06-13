pragma circom 2.1.0;

include "../../src/finalproof.circom";

template FinalProofTest(DEPTH, LEAF_ARRAY_LEN) {
  signal input siblings[DEPTH][4];
  signal input index;
  signal input leaf[LEAF_ARRAY_LEN];
  signal input root[4];

  component fpc = FinalProofChecker(DEPTH, LEAF_ARRAY_LEN);
  fpc.siblings <== siblings;
  fpc.index <== index;
  fpc.leaf <== leaf;

  root === fpc.root;
}

component main = FinalProofTest(3, 8);
