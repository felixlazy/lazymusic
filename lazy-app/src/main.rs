use lazy_app::app::App;
use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::default();
    app.run().await?;
    Ok(())
}
