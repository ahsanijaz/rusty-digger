use clap::Parser;
use futures::stream::{self, StreamExt};
use std::fs::File;
use std::io::{BufRead, BufReader};
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

/// A lightning-fast, asynchronous DNS subdomain scanner
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
    let args = Cli::parse();

    let file = File::open(&args.wordlist).expect("Failed to open wordlist file");
    let reader = BufReader::new(file);

    // Read all lines from the wordlist into a vector
    let wordlist: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    // Create a stream from our wordlist vector. A stream is like an async iterator.
    let stream = stream::iter(wordlist);

    // Create a single resolver to be shared across all async tasks
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    println!("Scanning for subdomains on: {}", &args.domain);

    stream
        .for_each_concurrent(200, |word| {
            // We need to clone the resolver and domain for each async task.
            // `clone()` is cheap here as it just copies a reference.
            let resolver = resolver.clone();
            let domain = args.domain.clone();

            // `tokio::spawn` creates a lightweight, asynchronous task.
            // We can create thousands of these without the overhead of system threads.
            async move {
                let subdomain = format!("{}.{}", word, domain);
                if let Ok(lookup) = resolver.lookup_ip(&subdomain).await {
                    for ip in lookup.iter() {
                        println!("[+] Found: {} -> {}", subdomain, ip);
                    }
                }
            }
        })
        .await; // `.await` waits for all the concurrent tasks to finish.
}
