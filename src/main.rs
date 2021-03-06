use ferris_says::say;
use std::io::{stdout, BufWriter};
use structopt::StructOpt;


fn main() {
    log();
    let args = Cli::from_args();

    let stdout = stdout();
    let width = args.input.len();

    let mut writer = BufWriter::new(stdout.lock());
    say(args.input.as_ref(), width, &mut writer).unwrap();
}

fn log() {
    println!("printing...");
}

#[derive(StructOpt)]
struct Cli {
    input: String,
}