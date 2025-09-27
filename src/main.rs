use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

/// A simple, sequential DNS subdomain scanner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The target domain to scan
    #[arg(short, long)]
    domain: String,

    /// The path to the wordlist file
    #[arg(short, long)]
    wordlist: String,
}

#[tokio::main]
async fn main() {
    // ---- Code Explanation: The CLI Struct ----
    // This uses the `clap` crate to parse command-line arguments.
    // Instead of manually handling `env::args()`, we define a struct.
    // `clap` automatically generates help messages and handles parsing.
    let args = Cli::parse();

    println!("Scanning for subdomains on: {}", &args.domain);
    println!("Using wordlist: {}", &args.wordlist);

    // Create a resolver instance once, to be reused for all lookups.
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    // ---- Code Explanation: File Handling ----
    // `File::open` attempts to open the file at the path specified.
    // It returns a `Result`, so we use `.expect()` for a simple crash on error.
    let file = File::open(&args.wordlist).expect("Failed to open wordlist file");

    // `BufReader` provides a buffered way to read the file, which is efficient,
    // especially for large files. `lines()` gives us an iterator over each line.
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // Each `line` is also a `Result` in case of a reading error.
        let word = line.expect("Failed to read line from wordlist");

        // ---- Code Explanation: String Formatting ----
        // The `format!` macro is an easy way to build a new String.
        // Here, we combine the word from the list with the target domain.
        let subdomain = format!("{}.{}", word, &args.domain);

        // We perform the lookup for the constructed subdomain.
        match resolver.lookup_ip(&subdomain).await {
            Ok(lookup) => {
                // If the lookup is successful, we iterate over the found IPs.
                for ip in lookup.iter() {
                    println!("[+] Found: {} -> {}", subdomain, ip);
                }
            }
            // If the lookup fails (Err), it means the subdomain likely doesn't exist.
            // We do nothing and just continue to the next word in the list.
            Err(_) => {
                continue;
            }
        }
    }
}
