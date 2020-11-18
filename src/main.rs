extern crate md2;
use md2::{Md2, Digest};
fn main() {
    let mut text = String::from("39");
    let mut hasher = Md2::new();
    // 运算一亿次
    for n in 0..1_0000_0000 {
        hasher.update(text);
        let result = hasher.finalize_reset();
        text = format!("{:x}", result).to_string();

        if n % 10_0000 == 0 {
            println!("‰{}", n / 10_0000);
        }
    }
    println!("value: {}", text);
}