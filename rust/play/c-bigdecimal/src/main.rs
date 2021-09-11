use std::str::FromStr;

use bigdecimal::BigDecimal;

fn main() {
    //let a = BigDecimal::from(1.1);
    let b = BigDecimal::from_str("1.1").unwrap();

    println!("a: {}, b: {}", b, b);
}
