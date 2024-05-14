use std::env;
use sysinfo::{
    Components, Disks, Networks, System,
};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("OS: {} {}", env::consts::OS, env::consts::ARCH);
    println!("Memory: {}MiB / {}MiB", 
             (sys.used_memory() / 100000),
             (sys.total_memory() / 100000), 
     );
}
