use std::io::{self, BufRead};
use std::time::{Duration, Instant};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// How many lines per interval
   #[arg(short, long, default_value_t = 1)]
   max_per_interval: u64,

   /// Interval in seconds
   #[arg(short, long, default_value_t = 1)]
   interval: u64,
}

fn main() {
    let args: Args = Args::parse();
    let stdin = io::stdin();

    let mut start = Instant::now();
    let mut limited: bool = false;
    let mut count = 0;
    let time_interval = Duration::from_secs(args.interval);

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in.");
        if  start.elapsed() > time_interval {
            limited = false;
            start = Instant::now();
        }
        if limited == true {
            continue;
        }
        println!("{line}");
        count = count + 1;
        if count >= args.max_per_interval && start.elapsed() <= time_interval {
            limited = true;
            count = 0;
        }
    }
}
