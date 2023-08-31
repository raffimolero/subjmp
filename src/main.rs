use std::{fmt::Display, io::stdin, num::Wrapping as W};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Program {
    mem: [W<i8>; 128],
    acc: W<i8>,
    ip: W<i8>,
}

impl Program {
    fn new(mem: [u8; 128]) -> Self {
        Self {
            mem: mem.map(|x: u8| W(x as i8)),
            acc: W(0),
            ip: W(0),
        }
    }

    fn step(&mut self) {
        const HI: i8 = -0b_1000_0000;
        let val = &mut self.mem[(self.ip.0 & !HI) as usize];
        if val.0 & HI == 0 {
            *val -= self.acc;
            self.acc = *val;
        } else if self.acc.0 < 0 {
            self.ip += val.0 & !HI;
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "acc: {:>3}", self.acc)?;
        writeln!(f, "ip: {:>3}", self.ip)?;
        let mut i = W(0_i8);
        'outer: for _ in 0..16 {
            for _ in 0..8 {
                let (l, r) = if i == self.ip { ('[', ']') } else { (' ', ' ') };
                write!(f, "{l}{:>3}{r}", self.mem[i.0 as usize])?;

                i += 1;
                if i.0 == 126 {
                    break 'outer;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        writeln!(f, "Input: {:0>4b}", self.mem[126].0 & 0b_1111)?;
        write!(f, "Output: {:0>8b}", self.mem[127])
    }
}

fn main() {
    #[rustfmt::skip]
    let mem = [
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,
        0b_0001_0000,     // &input = 126
        0b_1010_1110, // &output = 127
    ];

    let mut program = Program::new(mem);

    println!("Enter.");
    for _line in stdin().lines() {
        program.step();
        println!("{program}");
    }
}
