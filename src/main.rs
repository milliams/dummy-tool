use std::{thread, time};

use argh::FromArgs;
use rand::Rng;

#[derive(FromArgs)]
/// Calculate the value of the interesting simulation
struct Args {
    /// number of iterations
    #[argh(positional)]
    num: u8,
}

fn main() -> Result<(), String> {
    let args: Args = argh::from_env();

    let a = args.num;
    println!("{:}", a);

    let milliseconds = rand::thread_rng().gen_range(500..2000);
    let pause = time::Duration::from_millis(milliseconds);
    thread::sleep(pause);

    Ok(())
}
