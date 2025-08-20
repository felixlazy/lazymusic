use lazy_music::app::App;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::default();
    match app.run().await {
        Ok(_) => println!("app run "),
        Err(e) => println!("{e}"),
    }
    Ok(())
}
