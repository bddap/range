use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "range", about = "print a range of numbers to stdout")]
struct Opt {
    start: u128,
    stop: u128,
    hop: u128,
}

fn main() {
    let Opt { start, stop, hop } = Opt::from_args();
    assert!(start < stop);
    assert!(hop > 0);
    let mut i = start;
    while i < stop {
        println!("{}", i);
        i += hop;
    }
}
