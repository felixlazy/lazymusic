use lazy_music::app::App;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;
    let mut app = App::default();
    app.run().await?;
    Ok(())
}
