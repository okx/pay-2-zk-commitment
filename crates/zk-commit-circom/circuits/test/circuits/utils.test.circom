pragma circom 2.0.9;
include "../../src/utils.circom";

template UtilsTest() {
    signal input in;
    signal output out;

    component sgl = Num128ToGl();
    sgl.in <==    161360025679164738177229890718278319974;

    sgl.out[0] === 8747344519683359610;
    sgl.out[1] === 15084727568693690214;

    component re1 = ReverseEndian();
    re1.in <== 8747344519683359610;
    re1.out === 8846107364446659705;

    component re2 = ReverseEndian();
    re2.in <== 15084727568693690214;
    re2.out === 7385770527525656529;

    // Dummy input/output
    in === 1;
    out <== 1;
}

component main = UtilsTest();
