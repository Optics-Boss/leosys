use std::env;

fn main() {
    println!("OS: {} {}", env::consts::OS, env::consts::ARCH);
    for (key, value) in env::vars() {
        println!("{key:?}: {value:?}");
    }
}
