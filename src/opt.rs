use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(short, long, default_value = "127.0.0.1")]
    pub address: String,

    #[structopt(short, long, default_value = "80")]
    pub port: u16,

    #[structopt(short, long, default_value = "4")]
    pub thread_pool_size: usize,
}
