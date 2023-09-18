/*A CLI that query S3 for a string using Clap 

Bug! This doesn't yet find partial  matches.
TODO: Fix this.
We need to add unit tests for partial string matches and also better logging

*/
use clap::Parser;
use query_s3_cli::get_query_buckets;

/// CLI tool to query S3 for a string
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The string
    #[arg(short, long)]
    query: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let query = args.query;
    let buckets = get_query_buckets(&query).await;
    println!("{:?}", buckets);
}




