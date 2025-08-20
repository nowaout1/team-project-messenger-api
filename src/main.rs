use messenger_api::{AppConfig, create_app};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    if let Err(error) = dotenvy::dotenv() {
        tracing::warn!("Failed to load .env: {error:?}")
    }

    let config = init_config()?;

    tracing::info!("Listening on {}", &config.listen_address);

    let listener = tokio::net::TcpListener::bind(&config.listen_address).await?;
    let app = create_app(config).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

fn init_config() -> anyhow::Result<AppConfig> {
    let listen_address = dotenvy::var("LISTEN_ADDRESS")?.leak();
    let authentication_address = dotenvy::var("AUTHENTICATION_ADDR")?.leak();

    Ok(AppConfig {
        listen_address,
        authentication_address,
    })
}
