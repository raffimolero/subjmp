use subjmp::prelude::*;

fn main() {
    #[rustfmt::skip]
    let mem = [
    // LEGEND:
    //    0x## => instruction
    //    num  => used memory location
    //      0  => unused
    // ------------------------------------------------------
    //    _0    _1    _2    _3    _4    _5    _6    _7
    //    _8    _9    _a    _b    _c    _d    _e    _f
        0xfe, 0xfe, 0xfe, 0xfd, 0xff, 0xfd, 0xfd, 0xfc, // 8_
        0xfd, 0xfb, 0xfb, 0xfc, 0x74,    0,    0,    0,

           0,    0,    0,    0,    0,    0,    0,    0, // 9_
           0,    0,    0,    0,    0,    0,    0,    0,

           0,    0,    0,    0,    0,    0,    0,    0, // a_
           0,    0,    0,    0,    0,    0,    0,    0,

           0,    0,    0,    0,    0,    0,    0,    0, // b_
           0,    0,    0,    0,    0,    0,    0,    0,

           0,    0,    0,    0,    0,    0,    0,    0, // c_
           0,    0,    0,    0,    0,    0,    0,    0,

           0,    0,    0,    0,    0,    0,    0,    0, // d_
           0,    0,    0,    0,    0,    0,    0,    0,

           0,    0,    0,    0,    0,    0,    0,    0, // e_
           0,    0,    0,    0,    0,    0,    0,    0,

           0,    0,    0,    0,    0,    0,    0,    0, // f_
           0,    0,    0,    0, 0xf0, 0x10,
        0b_0001_0000, // &input  = 0xfe
        0b_0000_0000, // &output = 0xff
    ];

    // Program::new(0b_0000, 0x00, 0, mem).run().unwrap();
    println!("{}", Program::new(0b_0000, 0x00, 0, mem).rle());
}
