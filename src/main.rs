use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::fs::rename;
use std::{env::temp_dir, iter::repeat};

fn main() {
    let mut rng = thread_rng();
    let noise: String = repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(16)
        .collect();
    let name = String::from("node_modules-") + &noise;
    let path = temp_dir().join(name);
    match rename("node_modules", path) {
        Ok(()) => {
            println!("👋 bye node_modules!");
        }
        Err(e) => {
            eprintln!("Could not throw away node_modules: {e}");
            std::process::exit(1);
        }
    }
}
