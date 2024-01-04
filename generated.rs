// Performs element-wise floating point absolute value 

(vabs.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vabs.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vabs.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vabs.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs element-wise floating point addition 

(vadd.s $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vadd.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vadd.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vadd.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs element-wise floating point asin(rs)⋅2/π operation 

(vasin.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vasin.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vasin.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vasin.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Calculates the average value of the vector elements 

(vavg.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vavg.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vavg.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs a `butterfly` operation between the input elements. 

(vbfy1.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vbfy1.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs a `butterfly` operation between the input elements. 

(vbfy2.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Converts the input packed chars into full 32 bit integers in the output register. The input is placed on the most significant bits of the output integer, while the least significant bits are filled with zeros. 

(vc2i.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

// Performs element-wise floating point cos(π/2⋅rs) operation 

(vcos.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vcos.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vcos.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vcos.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs a partial cross-product operation 

(vcrs.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100110100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

// Performs a full cross-product operation 

(vcrsp.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110010100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

// Performs a 2x2 matrix determinant between two matrix rows 

(vdet.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100111000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

// Performs element-wise floating point division 

(vdiv.s $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100011100000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vdiv.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100011100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vdiv.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100011100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vdiv.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100011100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs vector floating point dot product 

(vdot.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100100100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vdot.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100100100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vdot.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100100100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs element-wise floating point exp2(rs) operation 

(vexp2.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vexp2.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vexp2.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vexp2.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Converts the float inputs to float16 (half-float) and packs them in pairs in the output register. The conversion process may naturally result in precision loss. 

(vf2h.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vf2h.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Adds all vector elements toghether producing a single result 

(vfad.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vfad.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vfad.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Converts the input packed float16 into full 32 bit floating point numbers. 

(vh2f.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vh2f.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

// Performs vector floating point homegeneous dot product 

(vhdp.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100110000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vhdp.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100110000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vhdp.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100110000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs a vector-matrix homogeneous transform (matrix-vector product), with a vector result 

(vhtfm2.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110000100000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_mpair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

// Performs a vector-matrix homogeneous transform (matrix-vector product), with a vector result 

(vhtfm3.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110001000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_mtriple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

// Performs a vector-matrix homogeneous transform (matrix-vector product), with a vector result 

(vhtfm4.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110001100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_mquad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Converts the four integer inputs to char and packs them as a single element word. The conversion process takes the 8 most significant bits of each integer. 

(vi2c.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Converts the integer inputs to short and packs them in pairs in the output register. The conversion process takes the 16 most significant bits of each integer. 

(vi2s.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vi2s.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Converts the four integer inputs to char and packs them as a single element word. The conversion process takes the 8 most significant bits of each integer and clamps any negative input values to zero. 

(vi2uc.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Converts the integer inputs to short and packs them in pairs in the output register. The conversion process takes the 16 most significant bits of each integer and clamps any negative input values to zero. 

(vi2us.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vi2us.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Initializes destination register as an identity matrix row (all zeros but one). The behaviour depends on the destination register number. 

(vidt.p $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
    )
};

(vidt.q $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
    )
};

// Performs element-wise logB() calculation 

(vlgb.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

// Performs element-wise floating point log2(rs) operation 

(vlog2.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vlog2.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vlog2.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vlog2.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs element-wise floating point max(rs, rt) operation 

(vmax.s $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101101100000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vmax.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101101100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vmax.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101101100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vmax.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101101100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Writes the identity matrix into the destination register 

(vmidt.p $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_mpair!($rd), " << 0)",
    )
};

(vmidt.t $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_mtriple!($rd), " << 0)",
    )
};

(vmidt.q $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_mquad!($rd), " << 0)",
    )
};

// Performs element-wise floating point min(rs, rt) operation 

(vmin.s $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101101000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vmin.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101101000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vmin.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101101000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vmin.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101101000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Element-wise data copy 

(vmmov.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_mpair!($rd), " << 0)",
        "| (", $crate::register_mpair!($rs), " << 8)",
    )
};

(vmmov.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_mtriple!($rd), " << 0)",
        "| (", $crate::register_mtriple!($rs), " << 8)",
    )
};

(vmmov.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_mquad!($rd), " << 0)",
        "| (", $crate::register_mquad!($rs), " << 8)",
    )
};

// Performs a matrix multiplication 

(vmmul.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_mpair!($rd), " << 0)",
        "| (", $crate::register_mpair!($rs), " << 8)",
        "| (", $crate::register_mpair!($rt), " << 16)",
    )
};

(vmmul.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_mtriple!($rd), " << 0)",
        "| (", $crate::register_mtriple!($rs), " << 8)",
        "| (", $crate::register_mtriple!($rt), " << 16)",
    )
};

(vmmul.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_mquad!($rd), " << 0)",
        "| (", $crate::register_mquad!($rs), " << 8)",
        "| (", $crate::register_mquad!($rt), " << 16)",
    )
};

// Overwrites all elements in a matrix with ones (1.0f) 

(vmone.p $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_mpair!($rd), " << 0)",
    )
};

(vmone.t $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_mtriple!($rd), " << 0)",
    )
};

(vmone.q $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_mquad!($rd), " << 0)",
    )
};

// Element-wise data copy 

(vmov.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vmov.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vmov.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vmov.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs a matrix scaling by a single factor 

(vmscl.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110010000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_mpair!($rd), " << 0)",
        "| (", $crate::register_mpair!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vmscl.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110010000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_mtriple!($rd), " << 0)",
        "| (", $crate::register_mtriple!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vmscl.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110010000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_mquad!($rd), " << 0)",
        "| (", $crate::register_mquad!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

// Performs element-wise floating point multiplication 

(vmul.s $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100100000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vmul.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100100000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vmul.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100100000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vmul.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100100000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Writes a zero matrix into the destination register 

(vmzero.p $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_mpair!($rd), " << 0)",
    )
};

(vmzero.t $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_mtriple!($rd), " << 0)",
    )
};

(vmzero.q $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_mquad!($rd), " << 0)",
    )
};

// Performs element-wise floating point negation 

(vneg.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vneg.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vneg.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vneg.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs element-wise floating point negated reciprocal 

(vnrcp.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vnrcp.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vnrcp.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vnrcp.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs element-wise floating point -sin(π/2⋅rs) operation 

(vnsin.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vnsin.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vnsin.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vnsin.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs element-wise one's complement (1.0f - x) 

(vocp.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vocp.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vocp.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vocp.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Writes ones (1.0f) into the destination register 

(vone.s $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
    )
};

(vone.p $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
    )
};

(vone.t $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
    )
};

(vone.q $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
    )
};

// Performs a vector-matrix homogeneous transform (matrix-vector product), with a vector result 

(vqmul.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110010100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs element-wise floating point reciprocal 

(vrcp.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vrcp.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vrcp.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vrcp.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs element-wise floating point 1/exp2(rs) operation (equivalent to exp2(-rs)) 

(vrexp2.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vrexp2.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vrexp2.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vrexp2.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Writes pseudorandom numbers to the destination elements so that each element (x) can assert 1.0f <= x < 2.0f 

(vrndf1.s $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
    )
};

(vrndf1.p $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
    )
};

(vrndf1.t $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
    )
};

(vrndf1.q $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
    )
};

// Writes pseudorandom numbers to the destination elements so that each element (x) can assert 2.0f <= x < 4.0f 

(vrndf2.s $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
    )
};

(vrndf2.p $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
    )
};

(vrndf2.t $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
    )
};

(vrndf2.q $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
    )
};

// Writes pseudorandom 32 bit numbers to the destination elements (full 32bit range) 

(vrndi.s $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
    )
};

(vrndi.p $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
    )
};

(vrndi.t $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
    )
};

(vrndi.q $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
    )
};

// Uses the integer value as a seed for the pseudorandom number generator. 

(vrnds.s $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

// Performs element-wise floating pointreciprocal square root 

(vrsq.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vrsq.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vrsq.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vrsq.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Converts the input packed shorts into full 32 bit integers in the output register. The input is placed on the most significant bits of the output integer, while the least significant bits are filled with zeros. 

(vs2i.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vs2i.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

// Saturates inputs to the [0.0f ... 1.0f] range 

(vsat0.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vsat0.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vsat0.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vsat0.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Saturates inputs to the [-1.0f ... 1.0f] range 

(vsat1.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vsat1.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vsat1.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vsat1.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Rescales rs operand to have rt as exponent. This would be equivalent to ldexp(frexp(rs, NULL), rt + 128). If we express the number in its IEEE754 terms, that is, if rs can be expressed as ±m * 2^e, the instruction will replace "e" with the value of rt + 127 mod 256. 

(vsbn.s $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100001000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

// Rescales rs operand to have zero as exponent, so that it is reduced to the [1.0, 2.0) interval. This is essentially equivalent to the vsbn instruction with rt=0. 

(vsbz.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

// Scales a vector (element-wise) by an scalar factor 

(vscl.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100101000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vscl.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100101000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vscl.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100101000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

// Performs element-wise floating point comparison. The result is -1.0f, 0.0f or 1.0f depending on whether the input vs is less that vt, equal, or greater, respectively. 

(vscmp.s $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101110100000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vscmp.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101110100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vscmp.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101110100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vscmp.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101110100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs element-wise floating point bigger-or-equal comparison. The result will be 1.0 if vs is bigger or equal to vt, otherwise will be zero. 

(vsge.s $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101111000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vsge.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101111000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vsge.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101111000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vsge.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101111000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs element-wise floating point sign(rs) operation. This function returns -1, 0 or 1 depending on whether the input is negative zero or positive respectively. 

(vsgn.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vsgn.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vsgn.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vsgn.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs element-wise floating point sin(π/2⋅rs) operation 

(vsin.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vsin.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vsin.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vsin.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs element-wise floating point less-than comparison. The result will be 1.0 if vs less than vt, otherwise will be zero. 

(vslt.s $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101111100000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vslt.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101111100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vslt.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101111100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vslt.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01101111100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs element-wise one's complement (1.0f - x) with saturation to [0.0f ... 1.0f] 

(vsocp.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vsocp.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

// Performs element-wise floating point aproximate square root 

(vsqrt.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vsqrt.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vsqrt.t $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vsqrt.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs a min() sorting step between elements pairs 0-1 and 2-3, shuffling them depending on their values. 

(vsrt1.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs a min() sorting step between elements pairs 3-0 and 1-2, shuffling them depending on their values. 

(vsrt2.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs a max() sorting step between elements pairs 0-1 and 2-3, shuffling them depending on their values. 

(vsrt3.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs a max() sorting step between elements pairs 3-0 and 1-2, shuffling them depending on their values. 

(vsrt4.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs element-wise floating point subtraction 

(vsub.s $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100000100000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vsub.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100000100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vsub.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100000100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vsub.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b01100000100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Converts four ABGR8888 color points to ABGR4444. The output 16 bit values are packed into a vector register pair. 

(vt4444.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Converts four ABGR8888 color points to ABGR1555. The output 16 bit values are packed into a vector register pair. 

(vt5551.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Converts four ABGR8888 color points to BGR565. The output 16 bit values are packed into a vector register pair. 

(vt5650.q $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
    )
};

// Performs a vector-matrix transform (matrix-vector product), with a vector result 

(vtfm2.p $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110000100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_mpair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

// Performs a vector-matrix transform (matrix-vector product), with a vector result 

(vtfm3.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110001000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_mtriple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

// Performs a vector-matrix transform (matrix-vector product), with a vector result 

(vtfm4.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110001100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_mquad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Converts the input packed chars into full 32 bit integers in the output register. The input is placed on the most significant bits of the output integer, while the least significant bits are filled with zeros  XXXXXs. 

(vuc2ifs.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

// Converts the input packed shorts into full 32 bit integers in the output register. The input is placed on the most significant bits of the output integer, while the least significant bits are filled with zeros. 

(vus2i.s $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vus2i.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

// Writes zeros (0.0f) into the destination register 

(vzero.s $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
    )
};

(vzero.p $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
    )
};

(vzero.t $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
    )
};

(vzero.q $rd:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
    )
};