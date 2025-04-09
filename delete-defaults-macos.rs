use std::collections::BTreeSet;

use clap::Parser;
use delete_defaults_macos::{defaults_delete_domains, Result, DEFAULT_DOMAINS};
use iocore::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "delete_defaults_macos")]
pub struct Cli {
    #[arg()]
    domains: Vec<String>,

    #[arg(short, long)]
    output_path: Path,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    if args.output_path.exists() {
        eprintln!("{} exists", &args.output_path);
        std::process::exit(1);
    }
    let mut domains = BTreeSet::<String>::new();
    domains.extend(args.domains.clone());
    domains.extend(DEFAULT_DOMAINS.iter().map(|n| n.to_string()).collect::<Vec<String>>());

    let result = defaults_delete_domains(Vec::<String>::from_iter(domains.iter().map(|n|n.to_string())));

    args.output_path.write(serde_json::to_string_pretty(&result)?.as_bytes())?;
    Ok(())
}
