use lazy_music::app::App;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::default();
    Ok(())
}
