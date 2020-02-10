use rand::prelude::*;

fn main() {
    let mut s: Vec<std::string::String> = Vec::new();

    let mut rng = rand::thread_rng();
    for _i in 0..6{
        s.push(format!("{:x}", rng.gen_range(0x0, 0xf)));
    }
    println!("#{}", s.join(""));
}
