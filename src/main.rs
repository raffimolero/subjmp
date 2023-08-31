use std::{fmt::Display, io::stdin, num::Wrapping as W};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Program {
    mem: [W<i8>; 128],
    acc: W<i8>,
    ip: W<i8>,
    keys: i8,
}

impl Program {
    const INPUT: usize = 126;
    const OUTPUT: usize = 127;

    fn new(keys: i8, ip: i8, acc: i8, mem: [i16; 128]) -> Self {
        Self {
            mem: mem.map(|x: i16| W(x as i8)),
            acc: W(acc),
            ip: W(ip),
            keys,
        }
    }

    fn input(&mut self, line: &str) {
        for c in line.chars() {
            match c {
                '0' => self.keys |= 1 << 0,
                '1' => self.keys |= 1 << 1,
                'a' => self.keys |= 1 << 2,
                'c' => self.keys |= 1 << 3,
                _ => {}
            }
        }
    }

    fn step(&mut self) {
        const HI: i8 = -0b_1000_0000;
        let cmd = self.mem[self.ip.0 as usize];
        let addr = (cmd.0 & !HI) as usize;
        dbg!(addr);
        let val = &mut self.mem[addr];

        if cmd.0 & HI != 0 {
            *val -= self.acc;
            self.acc = *val;
        } else if self.acc.0 < 0 {
            self.ip += cmd.0 & !HI;
        }
        self.ip += 1;
        self.ip &= !HI;

        let mmap = &mut self.mem[Self::INPUT].0;
        if *mmap == 0 {
            const SENTINEL: i8 = 0b_0001_0000;
            *mmap = self.keys | SENTINEL;
            self.keys = 0;
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ip: {:>3}", self.ip)?;
        writeln!(f, "acc: {:>3}", self.acc)?;

        let mut i = W(0_i8);
        for _ in 0..16 {
            for _ in 0..8 {
                let (l, r) = if i == self.ip { ('[', ']') } else { (' ', ' ') };
                write!(f, "{l}{:>4}{r}", self.mem[i.0 as usize])?;

                i += 1;
            }
            writeln!(f)?;
        }

        writeln!(f, "Input: {:0>4b}", self.keys)?;
        write!(f, "Output: {:0>8b}", self.mem[Self::OUTPUT])
    }
}

fn main() {
    #[rustfmt::skip]
    let mem = [
    //    _0    _1    _2    _3    _4    _5    _6    _7
    //    _8    _9    _a    _b    _c    _d    _e    _f
        0xff, 0xfe, 0xfe, 0x85,128-5,   -1,    0,    0, // 8_
           0,    0,    0,    0,    0,    0,    0,    0,

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
           0,    0,    0,    0,    0,    0,
        0b_0001_0000, // &input  = 126
        0b_0000_0000, // &output = 127
    ];

    let mut program = Program::new(0b_0000, 0x01, -1, mem);
    println!("{program}");

    for line in stdin().lines() {
        program.input(&line.unwrap());
        program.step();
        println!("{program}");
    }
}
