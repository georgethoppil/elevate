use elevate::{get_configuration, Application};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set up tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "elevate=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // getting config
    let configuration = get_configuration().expect("Failed to read configuration");
    tracing::debug!("config is {:?}", configuration);

    // setting up listener
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))
    .await?;

    // get router
    let app = Application::new(configuration).build().await?;

    // serve app
    axum::serve(listener, app).await?;
    Ok(())
}
