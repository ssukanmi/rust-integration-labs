use std::error::Error;

use bit_patterns::{float, q7::Q7, random};

fn main() -> Result<(), Box<dyn Error>> {
    let n: f32 = -42.42;
    let (sign, exp, frac) = float::to_parts(n);
    let (sign_, exp_, mant) = float::decode(sign, exp, frac);
    let n_ = float::from_parts(sign_, exp_, mant);
    println!("Float {} is represented in binary as :", n);
    println!("{:032b}", n.to_bits());
    println!("sign: {:b}\nexponent: {:b}\nmatissa: {:b}", sign, exp, frac);
    println!("{} * {} * {}", sign_, exp_, mant);
    dbg!(n_);

    let q = Q7(-128);
    let f = f64::from(q);
    let q_ = Q7::from(f);
    dbg!(q);
    dbg!(f);
    dbg!(q_);

    println!(
        "max of input range: {:08b} -> {:?}",
        0xff,
        random::mock_rand(0xff)
    );
    println!(
        "mid of input range: {:08b} -> {:?}",
        0x7f,
        random::mock_rand(0x7f)
    );
    println!(
        "min of input range: {:08b} -> {:?}",
        0x00,
        random::mock_rand(0x00)
    );

    Ok(())
}
