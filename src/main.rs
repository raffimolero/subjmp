use std::{io::stdin, num::Wrapping as W};

fn main() {
    let mut acc: W<i8> = W(0);
    let mut ip = W(0);
    #[rustfmt::skip]
    let mut mem: [W<i8>; 128] = [
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
          0,  0,  0,  0,  0,  0,  0,  0,
    ].map(W);

    for _line in stdin().lines() {
        const HI: i8 = -0b_1000_0000;
        let val = &mut mem[(ip.0 & !HI) as usize];
        if val.0 & HI == 0 {
            *val -= acc;
            acc = *val;
        } else if acc.0 < 0 {
            ip += val.0 & !HI;
        }

        println!("acc: {acc:>3}, ip: {ip:>3}");
        let mut i = W(0_i8);
        for _ in 0..16 {
            for _ in 0..8 {
                let (l, r) = if i == ip { ('[', ']') } else { (' ', ' ') };
                print!("{l}{:>3}{r}", mem[i.0 as usize]);
                i += 1;
            }
            println!();
        }
    }
}
