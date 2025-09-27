use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

#[tokio::main]
async fn main() {
    let domain_to_resolve = "www.google.com";
    println!("Resolving domain: {}", domain_to_resolve);

    // Create an asynchronous resolver
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    // Look up the IP addresses associated with the domain
    match resolver.lookup_ip(domain_to_resolve).await {
        Ok(lookup) => {
            for ip in lookup.iter() {
                println!("  Found IP: {}", ip);
            }
        }
        Err(e) => {
            eprintln!("Resolution failed: {}", e);
        }
    }
}
