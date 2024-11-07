use clap::Parser;

#[derive(Parser)]
#[command(name = "GitHub PR Reporter")]
#[command(about = "Generates a report of GitHub pull requests for a given month.")]
pub struct Cli {
    #[arg(short, long, default_value_t = 2024)]
    pub year: i32,

    #[arg(short, long)]
    pub month: u32,

    #[arg(short, long, default_value = "table")]
    pub output: String,
}
