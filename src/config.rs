#[derive(Debug, clap::Parser)]
pub struct Config {
    #[arg(short, long)]
    database_url: String,
}
