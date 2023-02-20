use clap::Parser;
use tjuptatt::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    use tjuptatt::Cli;
    dbg!(Cli::parse());
    Ok(())
}
