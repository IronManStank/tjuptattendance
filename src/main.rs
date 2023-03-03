#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use util::get_east_eight_now;
    println!("{}", get_east_eight_now().format("%Y-%m-%d %H:%M:%S"));
    println!("{}", get_east_eight_now().format("%Y-%m-%d %H:%M:%S"));
    Ok(())
}
