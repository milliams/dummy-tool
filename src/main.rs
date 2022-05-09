use std::{thread, time};

use argh::FromArgs;

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

    let pause = time::Duration::from_secs(2);
    thread::sleep(pause);

    Ok(())
}
