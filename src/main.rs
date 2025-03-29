use clap::Parser;
use cov2html::coverage::generate_report_from_file;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input binary file path
    #[arg(short, long)]
    input: String,

    /// HTML output path
    #[arg(short, long)]
    output: String,

    /// Source code path
    #[arg(short, long)]
    source: String,
}

fn main() {
    let args = Args::parse();
    
    match generate_report_from_file(&args.input, &args.source, &args.output) {
        Ok(_) => println!("Coverage report generated successfully"),
        Err(e) => println!("Error generating coverage report: {}", e),
    }
}
