// Performs element-wise floating point absolute value 

(vabs.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0000001 << 16)",
    )
};

(vabs.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0000001 << 16)",
    )
};

(vabs.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0000001 << 16)",
    )
};

(vabs.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0000001 << 16)",
    )
};

// Performs element-wise floating point addition 

(vadd.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vadd.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vadd.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vadd.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs element-wise floating point asin(rs)⋅2/π operation 

(vasin.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0010111 << 16)",
    )
};

(vasin.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0010111 << 16)",
    )
};

(vasin.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0010111 << 16)",
    )
};

(vasin.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0010111 << 16)",
    )
};

// Calculates the average value of the vector elements 

(vavg.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b1000111 << 16)",
    )
};

(vavg.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b1000111 << 16)",
    )
};

(vavg.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1000111 << 16)",
    )
};

// Performs a `butterfly` operation between the input elements. 

(vbfy1.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b1000010 << 16)",
    )
};

(vbfy1.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1000010 << 16)",
    )
};

// Performs a `butterfly` operation between the input elements. 

(vbfy2.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1000011 << 16)",
    )
};

// Converts the input packed chars into full 32 bit integers in the output register. The input is placed on the most significant bits of the output integer, while the least significant bits are filled with zeros. 

(vc2i.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0111001 << 16)",
    )
};

// Performs element-wise floating point cos(π/2⋅rs) operation 

(vcos.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0010011 << 16)",
    )
};

(vcos.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0010011 << 16)",
    )
};

(vcos.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0010011 << 16)",
    )
};

(vcos.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0010011 << 16)",
    )
};

// Performs a partial cross-product operation 

(vcrs.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
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

// Loads a predefined indexed floating point constant specified by the immediate field 

(vcst.s $rd:ident $([$($rdp:tt)+])?, $imm5:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000011000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::vfpu_const!($imm5), " << 16)",
    )
};

(vcst.p $rd:ident $([$($rdp:tt)+])?, $imm5:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000011000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::vfpu_const!($imm5), " << 16)",
    )
};

(vcst.t $rd:ident $([$($rdp:tt)+])?, $imm5:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000011000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::vfpu_const!($imm5), " << 16)",
    )
};

(vcst.q $rd:ident $([$($rdp:tt)+])?, $imm5:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000011000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::vfpu_const!($imm5), " << 16)",
    )
};

// Performs a 2x2 matrix determinant between two matrix rows 

(vdet.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b01100111000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

// Performs element-wise floating point division 

(vdiv.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100011100000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vdiv.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100011100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vdiv.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100011100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vdiv.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100011100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs vector floating point dot product 

(vdot.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100100100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vdot.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100100100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vdot.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100100100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs element-wise floating point exp2(rs) operation 

(vexp2.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0010100 << 16)",
    )
};

(vexp2.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0010100 << 16)",
    )
};

(vexp2.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0010100 << 16)",
    )
};

(vexp2.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0010100 << 16)",
    )
};

// Converts the float inputs to float16 (half-float) and packs them in pairs in the output register. The conversion process may naturally result in precision loss. 

(vf2h.p $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0110010 << 16)",
    )
};

(vf2h.q $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0110010 << 16)",
    )
};

// Performs element-wise float to integer conversion with optional scaling factor, rounding down (that is, towards the previous, equal or smaller, integer value) 

(vf2id.s $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010011000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2id.p $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010011000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2id.t $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010011000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2id.q $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010011000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

// Performs element-wise float to integer conversion with optional scaling factor, rounding to the nearest integer 

(vf2in.s $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2in.p $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2in.t $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2in.q $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

// Performs element-wise float to integer conversion with optional scaling factor, rounding up (that is, towards the next, equal or greater, integer value) 

(vf2iu.s $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010010000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2iu.p $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010010000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2iu.t $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010010000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2iu.q $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010010000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

// Performs element-wise float to integer conversion with optional scaling factor, truncating the decimal argument (that is, rounding towards zero) 

(vf2iz.s $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010001000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2iz.p $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010001000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2iz.t $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010001000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vf2iz.q $rd:ident $([$($rdp:tt)+])?, $rs:ident, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010010001000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

// Adds all vector elements toghether producing a single result 

(vfad.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b1000110 << 16)",
    )
};

(vfad.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b1000110 << 16)",
    )
};

(vfad.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1000110 << 16)",
    )
};

// Loads a float16 immediate value in a register 

(vfim.s $rd:ident $([$($rdp:tt)+])?, $imm16:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11011111000000000000000000000000",
        "| 0b0000000010000000",
        "| ((", stringify!($imm16), " & 0xFFFF) << 0)",
        "| (", $crate::register_single!($rd), " << 16)",
    )
};

// Waits until the write buffer has been flushed 

(vflush) => {
    ".word 0b11111111111111110000010000001101"
};

// Converts the input packed float16 into full 32 bit floating point numbers. 

(vh2f.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0110011 << 16)",
    )
};

(vh2f.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0110011 << 16)",
    )
};

// Performs vector floating point homegeneous dot product 

(vhdp.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100110000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vhdp.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100110000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vhdp.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
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
        "| (", $crate::register_pair!($rd), " << 0)",
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
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_mquad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Converts the four integer inputs to char and packs them as a single element word. The conversion process takes the 8 most significant bits of each integer. 

(vi2c.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0111101 << 16)",
    )
};

// Performs element-wise integer to float conversion with optional scaling factor. The integer is divided by 2^scale after the conversion. 

(vi2f.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010010100000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vi2f.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010010100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vi2f.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010010100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

(vi2f.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $scale:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010010100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", stringify!($scale), " << 16)",
    )
};

// Converts the integer inputs to short and packs them in pairs in the output register. The conversion process takes the 16 most significant bits of each integer. 

(vi2s.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0111111 << 16)",
    )
};

(vi2s.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0111111 << 16)",
    )
};

// Converts the four integer inputs to char and packs them as a single element word. The conversion process takes the 8 most significant bits of each integer and clamps any negative input values to zero. 

(vi2uc.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0111100 << 16)",
    )
};

// Converts the integer inputs to short and packs them in pairs in the output register. The conversion process takes the 16 most significant bits of each integer and clamps any negative input values to zero. 

(vi2us.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0111110 << 16)",
    )
};

(vi2us.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0111110 << 16)",
    )
};

// Initializes destination register as an identity matrix row (all zeros but one). The behaviour depends on the destination register number. 

(vidt.p $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (0b0000011 << 16)",
    )
};

(vidt.q $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (0b0000011 << 16)",
    )
};

// Loads a signed 16 bit immediate value (converted to floating point) in a register 

(viim.s $rd:ident $([$($rdp:tt)+])?, $imm16:expr) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11011111000000000000000000000000",
        "| ((", stringify!($imm16), " & 0xFFFF) << 0)",
        "| (", $crate::register_single!($rd), " << 16)",
    )
};

// Performs element-wise logB() calculation 

(vlgb.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0110111 << 16)",
    )
};

// Performs element-wise floating point log2(rs) operation 

(vlog2.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0010101 << 16)",
    )
};

(vlog2.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0010101 << 16)",
    )
};

(vlog2.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0010101 << 16)",
    )
};

(vlog2.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0010101 << 16)",
    )
};

// Performs element-wise floating point max(rs, rt) operation 

(vmax.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101101100000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vmax.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101101100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vmax.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101101100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vmax.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
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
        "| (0b0000011 << 16)",
    )
};

(vmidt.t $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_mtriple!($rd), " << 0)",
        "| (0b0000011 << 16)",
    )
};

(vmidt.q $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_mquad!($rd), " << 0)",
        "| (0b0000011 << 16)",
    )
};

// Performs element-wise floating point min(rs, rt) operation 

(vmin.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101101000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vmin.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101101000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vmin.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101101000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vmin.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
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
        "| ((", $crate::register_mpair!($rs), " ^ 0b0100000) << 8)",
        "| (", $crate::register_mpair!($rt), " << 16)",
    )
};

(vmmul.t $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_mtriple!($rd), " << 0)",
        "| ((", $crate::register_mtriple!($rs), " ^ 0b0100000) << 8)",
        "| (", $crate::register_mtriple!($rt), " << 16)",
    )
};

(vmmul.q $rd:ident, $rs:ident, $rt:ident) => {
    concat!(
        ".word 0b11110000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_mquad!($rd), " << 0)",
        "| ((", $crate::register_mquad!($rs), " ^ 0b0100000) << 8)",
        "| (", $crate::register_mquad!($rt), " << 16)",
    )
};

// Overwrites all elements in a matrix with ones (1.0f) 

(vmone.p $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_mpair!($rd), " << 0)",
        "| (0b0000111 << 16)",
    )
};

(vmone.t $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_mtriple!($rd), " << 0)",
        "| (0b0000111 << 16)",
    )
};

(vmone.q $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_mquad!($rd), " << 0)",
        "| (0b0000111 << 16)",
    )
};

// Element-wise data copy 

(vmov.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
    )
};

(vmov.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
    )
};

(vmov.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
    )
};

(vmov.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
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

(vmul.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100100000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vmul.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100100000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vmul.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100100000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vmul.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
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
        "| (0b0000110 << 16)",
    )
};

(vmzero.t $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_mtriple!($rd), " << 0)",
        "| (0b0000110 << 16)",
    )
};

(vmzero.q $rd:ident) => {
    concat!(
        ".word 0b11110011100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_mquad!($rd), " << 0)",
        "| (0b0000110 << 16)",
    )
};

// Performs element-wise floating point negation 

(vneg.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0000010 << 16)",
    )
};

(vneg.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0000010 << 16)",
    )
};

(vneg.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0000010 << 16)",
    )
};

(vneg.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0000010 << 16)",
    )
};

// Does nothing and wastes one VFPU cycle. Used to avoid pipeline hazards. This instruction does consume prefixes. 

(vnop) => {
    ".word 0b11111111111111110000000000000000"
};

// Performs element-wise floating point negated reciprocal 

(vnrcp.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0011000 << 16)",
    )
};

(vnrcp.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0011000 << 16)",
    )
};

(vnrcp.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0011000 << 16)",
    )
};

(vnrcp.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0011000 << 16)",
    )
};

// Performs element-wise floating point -sin(π/2⋅rs) operation 

(vnsin.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0011010 << 16)",
    )
};

(vnsin.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0011010 << 16)",
    )
};

(vnsin.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0011010 << 16)",
    )
};

(vnsin.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0011010 << 16)",
    )
};

// Performs element-wise one's complement (1.0f - x) 

(vocp.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b1000100 << 16)",
    )
};

(vocp.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b1000100 << 16)",
    )
};

(vocp.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b1000100 << 16)",
    )
};

(vocp.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1000100 << 16)",
    )
};

// Writes ones (1.0f) into the destination register 

(vone.s $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (0b0000111 << 16)",
    )
};

(vone.p $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (0b0000111 << 16)",
    )
};

(vone.t $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (0b0000111 << 16)",
    )
};

(vone.q $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (0b0000111 << 16)",
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

(vrcp.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0010000 << 16)",
    )
};

(vrcp.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0010000 << 16)",
    )
};

(vrcp.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0010000 << 16)",
    )
};

(vrcp.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0010000 << 16)",
    )
};

// Performs element-wise floating point 1/exp2(rs) operation (equivalent to exp2(-rs)) 

(vrexp2.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0011100 << 16)",
    )
};

(vrexp2.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0011100 << 16)",
    )
};

(vrexp2.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0011100 << 16)",
    )
};

(vrexp2.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0011100 << 16)",
    )
};

// Writes pseudorandom numbers to the destination elements so that each element (x) can assert 1.0f <= x < 2.0f 

(vrndf1.s $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (0b0100010 << 16)",
    )
};

(vrndf1.p $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (0b0100010 << 16)",
    )
};

(vrndf1.t $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (0b0100010 << 16)",
    )
};

(vrndf1.q $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (0b0100010 << 16)",
    )
};

// Writes pseudorandom numbers to the destination elements so that each element (x) can assert 2.0f <= x < 4.0f 

(vrndf2.s $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (0b0100011 << 16)",
    )
};

(vrndf2.p $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (0b0100011 << 16)",
    )
};

(vrndf2.t $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (0b0100011 << 16)",
    )
};

(vrndf2.q $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (0b0100011 << 16)",
    )
};

// Writes pseudorandom 32 bit numbers to the destination elements (full 32bit range) 

(vrndi.s $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (0b0100001 << 16)",
    )
};

(vrndi.p $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (0b0100001 << 16)",
    )
};

(vrndi.t $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (0b0100001 << 16)",
    )
};

(vrndi.q $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (0b0100001 << 16)",
    )
};

// Uses the integer value as a seed for the pseudorandom number generator. 

(vrnds.s $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0100000 << 16)",
    )
};

// Calculates a rotation matrix row, given an angle argument 

(vrot.p $rd:ident, $rs:ident, [$($imm5:tt)*]) => {
    concat!(
        ".word 0b11110011101000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::vrot_immediate_pair!($($imm5)*), " << 16)",
    )
};

(vrot.t $rd:ident, $rs:ident, [$($imm5:tt)*]) => {
    concat!(
        ".word 0b11110011101000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::vrot_immediate_triple!($($imm5)*), " << 16)",
    )
};

(vrot.q $rd:ident, $rs:ident, [$($imm5:tt)*]) => {
    concat!(
        ".word 0b11110011101000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::vrot_immediate_quad!($($imm5)*), " << 16)",
    )
};

// Performs element-wise floating pointreciprocal square root 

(vrsq.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0010001 << 16)",
    )
};

(vrsq.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0010001 << 16)",
    )
};

(vrsq.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0010001 << 16)",
    )
};

(vrsq.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0010001 << 16)",
    )
};

// Converts the input packed shorts into full 32 bit integers in the output register. The input is placed on the most significant bits of the output integer, while the least significant bits are filled with zeros. 

(vs2i.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0111011 << 16)",
    )
};

(vs2i.p $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0111011 << 16)",
    )
};

// Saturates inputs to the [0.0f ... 1.0f] range 

(vsat0.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0000100 << 16)",
    )
};

(vsat0.p $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0000100 << 16)",
    )
};

(vsat0.t $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0000100 << 16)",
    )
};

(vsat0.q $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0000100 << 16)",
    )
};

// Saturates inputs to the [-1.0f ... 1.0f] range 

(vsat1.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0000101 << 16)",
    )
};

(vsat1.p $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0000101 << 16)",
    )
};

(vsat1.t $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0000101 << 16)",
    )
};

(vsat1.q $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0000101 << 16)",
    )
};

// Rescales rs operand to have rt as exponent. This would be equivalent to ldexp(frexp(rs, NULL), rt + 128). If we express the number in its IEEE754 terms, that is, if rs can be expressed as ±m * 2^e, the instruction will replace "e" with the value of rt + 127 mod 256. 

(vsbn.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100001000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

// Rescales rs operand to have zero as exponent, so that it is reduced to the [1.0, 2.0) interval. This is essentially equivalent to the vsbn instruction with rt=0. 

(vsbz.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0110110 << 16)",
    )
};

// Scales a vector (element-wise) by an scalar factor 

(vscl.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b01100101000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vscl.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b01100101000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vscl.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b01100101000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

// Performs element-wise floating point comparison. The result is -1.0f, 0.0f or 1.0f depending on whether the input vs is less that vt, equal, or greater, respectively. 

(vscmp.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101110100000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vscmp.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101110100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vscmp.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101110100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vscmp.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101110100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs element-wise floating point bigger-or-equal comparison. The result will be 1.0 if vs is bigger or equal to vt, otherwise will be zero. 

(vsge.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101111000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vsge.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101111000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vsge.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101111000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vsge.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101111000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Performs element-wise floating point sign(rs) operation. This function returns -1, 0 or 1 depending on whether the input is negative zero or positive respectively. 

(vsgn.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b1001010 << 16)",
    )
};

(vsgn.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b1001010 << 16)",
    )
};

(vsgn.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b1001010 << 16)",
    )
};

(vsgn.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1001010 << 16)",
    )
};

// Performs element-wise floating point sin(π/2⋅rs) operation 

(vsin.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0010010 << 16)",
    )
};

(vsin.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0010010 << 16)",
    )
};

(vsin.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0010010 << 16)",
    )
};

(vsin.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0010010 << 16)",
    )
};

// Performs element-wise floating point less-than comparison. The result will be 1.0 if vs less than vt, otherwise will be zero. 

(vslt.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101111100000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vslt.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101111100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vslt.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01101111100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vslt.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
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
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b1000101 << 16)",
    )
};

(vsocp.p $rd:ident, $rs:ident) => {
    concat!(
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b1000101 << 16)",
    )
};

// Performs element-wise floating point aproximate square root 

(vsqrt.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0010110 << 16)",
    )
};

(vsqrt.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0010110 << 16)",
    )
};

(vsqrt.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (0b0010110 << 16)",
    )
};

(vsqrt.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b0010110 << 16)",
    )
};

// Performs a min() sorting step between elements pairs 0-1 and 2-3, shuffling them depending on their values. 

(vsrt1.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1000000 << 16)",
    )
};

// Performs a min() sorting step between elements pairs 3-0 and 1-2, shuffling them depending on their values. 

(vsrt2.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1000001 << 16)",
    )
};

// Performs a max() sorting step between elements pairs 0-1 and 2-3, shuffling them depending on their values. 

(vsrt3.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1001000 << 16)",
    )
};

// Performs a max() sorting step between elements pairs 3-0 and 1-2, shuffling them depending on their values. 

(vsrt4.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1001001 << 16)",
    )
};

// Performs element-wise floating point subtraction 

(vsub.s $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100000100000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (", $crate::register_single!($rt), " << 16)",
    )
};

(vsub.p $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100000100000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (", $crate::register_pair!($rt), " << 16)",
    )
};

(vsub.t $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100000100000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (", $crate::register_triple!($rs), " << 8)",
        "| (", $crate::register_triple!($rt), " << 16)",
    )
};

(vsub.q $rd:ident $([$($rdp:tt)+])?, $rs:ident $([$($rsp:tt)+])?, $rt:ident $([$($rtp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        $($crate::instruction!(vpfxt $($rtp)*), "\n",)?
        ".word 0b01100000100000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (", $crate::register_quad!($rt), " << 16)",
    )
};

// Waits until all operations in the VFPU pipeline have completed 

(vsync) => {
    ".word 0b11111111111111110000001100100000"
};

// Converts four ABGR8888 color points to ABGR4444. The output 16 bit values are packed into a vector register pair. 

(vt4444.q $rd:ident, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1011001 << 16)",
    )
};

// Converts four ABGR8888 color points to ABGR1555. The output 16 bit values are packed into a vector register pair. 

(vt5551.q $rd:ident, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1011010 << 16)",
    )
};

// Converts four ABGR8888 color points to BGR565. The output 16 bit values are packed into a vector register pair. 

(vt5650.q $rd:ident, $rs:ident $([$($rsp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxs $($rsp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_quad!($rs), " << 8)",
        "| (0b1011011 << 16)",
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

(vuc2ifs.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0111000 << 16)",
    )
};

// Converts the input packed shorts into full 32 bit integers in the output register. The input is placed on the most significant bits of the output integer, while the least significant bits are filled with zeros. 

(vus2i.s $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (", $crate::register_single!($rs), " << 8)",
        "| (0b0111010 << 16)",
    )
};

(vus2i.p $rd:ident $([$($rdp:tt)+])?, $rs:ident) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (", $crate::register_pair!($rs), " << 8)",
        "| (0b0111010 << 16)",
    )
};

// Writes zeros (0.0f) into the destination register 

(vzero.s $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| (", $crate::register_single!($rd), " << 0)",
        "| (0b0000110 << 16)",
    )
};

(vzero.p $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b0000000010000000",
        "| (", $crate::register_pair!($rd), " << 0)",
        "| (0b0000110 << 16)",
    )
};

(vzero.t $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000000000000",
        "| (", $crate::register_triple!($rd), " << 0)",
        "| (0b0000110 << 16)",
    )
};

(vzero.q $rd:ident $([$($rdp:tt)+])?) => {
    concat!(
        $($crate::instruction!(vpfxd $($rdp)*), "\n",)?
        ".word 0b11010000000000000000000000000000",
        "| 0b1000000010000000",
        "| (", $crate::register_quad!($rd), " << 0)",
        "| (0b0000110 << 16)",
    )
};