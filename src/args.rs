use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Port number. Defaults to 3300
    #[arg(short, long, default_value_t = 3300)]
    pub port: u16,
}

pub fn get_args() -> Args {
    Args::parse()
}
