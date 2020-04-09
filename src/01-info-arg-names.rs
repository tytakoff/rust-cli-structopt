use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustcli", about = "Rust CLI examples with StructOpt")]
struct Opt {
    #[structopt(long)]
    title: String,

    #[structopt(short)]
    cutoff: i32,

    #[structopt(short, long)]
    message: String
}

fn main() {
    let opt = Opt::from_args();
    println!("opt = {:?}", opt);
}
