#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tjuptop::get_now;
    println!("{}", get_now().format("%Y-%m-%d %H:%M:%S"));
    println!("{}", get_now().format("%Y-%m-%d %H:%M:%S"));
    Ok(())
}
