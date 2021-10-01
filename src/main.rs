extern crate ramentimer;

use std::thread::sleep;
use std::time::Duration;

use ramentimer::Clock;

use structopt::StructOpt;
#[derive(StructOpt)]
pub struct Config {
    minutes: u64,
}

fn main() {
    let args = Config::from_args();
    let clock = Clock::new(args.minutes);
    start(clock);
}

fn start(clock: Clock) {
    println!("{}分待ちます。", clock.get_minutes());
    let millis = Duration::from_secs(clock.get_seconds());

    sleep(millis);

    println!("{}分経ちました。", clock.get_minutes());
}
