pragma circom 2.0.6;
include "../../node_modules/circomlib/circuits/comparators.circom";
include "./goldilocks_ext.circom";

template Reduce(N) {
  signal input in[N][2];
  signal input alpha[2];
  signal input old_eval[2];
  signal output out[2];

  component add[N];
  component mul[N];

  for (var i = N; i > 0; i--) {
    add[i - 1] = GlExtAdd();
    mul[i - 1] = GlExtMul();
    if (i == N) {
      mul[i - 1].a[0] <== old_eval[0];
      mul[i - 1].a[1] <== old_eval[1];
    } else {
      mul[i - 1].a[0] <== add[i].out[0];
      mul[i - 1].a[1] <== add[i].out[1];
    }
    mul[i - 1].b[0] <== alpha[0];
    mul[i - 1].b[1] <== alpha[1];

    add[i - 1].a[0] <== in[i - 1][0];
    add[i - 1].a[1] <== in[i - 1][1];
    add[i - 1].b[0] <== mul[i - 1].out[0];
    add[i - 1].b[1] <== mul[i - 1].out[1];
  }

  out[0] <== add[0].out[0];
  out[1] <== add[0].out[1];
}

// A working but slow implementation
template RandomAccess(N) {
  signal input a[N];
  signal input idx;
  signal output out;

  component cIsEqual[N];
  signal sum[N + 1];
  sum[0] <== 0;
  for (var i = 0; i < N; i++) {
    cIsEqual[i] = IsEqual();
    cIsEqual[i].in[0] <== idx;
    cIsEqual[i].in[1] <== i;
    sum[i + 1] <== cIsEqual[i].out * a[i] + sum[i];
  }

  out <== sum[N];
}

template RandomAccess2(N, M) {
  signal input a[N][M];
  signal input idx;
  signal output out[M];

  component ra[M];
  for (var i = 0; i < M; i++) {
    ra[i] = RandomAccess(N);
    ra[i].idx <== idx;
    for (var j = 0; j < N; j++) {
      ra[i].a[j] <== a[j][i];
    }
    out[i] <== ra[i].out;
  }
}

// convert a 128 bits number to 2 gl number
// out[0] holds the most significant number
// out[1] holds the least significant number
template Num128ToGl() {
    signal input in;
    signal output out[2];

     var acc=0;

    out[1] <-- (in) & ((1<<64) -1);
    acc += out[1];

    out[0] <-- (in >> 64) & ((1<<64) -1);
    acc += (out[0] * (1<<64));

  acc === in;
}

// TODO: need to add constraint to ensure it is indeed endian reversed
template ReverseEndian() {
  signal input in;
  signal output out;

  var mask = ((1<<8) -1);
  var tmp = (in ) & mask;
  tmp = tmp * (1<<8) + ((in >> (8*1) ) & mask);
  tmp = tmp * (1<<8) + ((in >> (8*2) ) & mask);
  tmp = tmp * (1<<8) + ((in >> (8*3) ) & mask);
  tmp = tmp * (1<<8) + ((in >> (8*4) ) & mask);
  tmp = tmp * (1<<8) + ((in >> (8*5) ) & mask);
  tmp = tmp * (1<<8) + ((in >> (8*6) ) & mask);
  tmp = tmp * (1<<8) + ((in >> (8*7) ) & mask);

  out <--tmp;


}


template Slot2Gls(n) {
  signal input in[n];
  signal output out[n*2];
 component spliters[n];
 component reverse_endians[n*2];

     for (var i = 0; i < n; i++) {
        spliters[i] = Num128ToGl();
        reverse_endians[i*2] =ReverseEndian();
        reverse_endians[i*2+1] =ReverseEndian();
    }

  for(var i=0;i<n;i++){

    spliters[i].in <== in[i];


    reverse_endians[i*2].in <== spliters[i].out[0];
    out[i*2] <== reverse_endians[i*2].out;


    reverse_endians[i*2+1].in <== spliters[i].out[1];
    out[i*2+1] <== reverse_endians[i*2+1].out;

  }
}