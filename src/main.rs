use elevate::{get_configuration, Application};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "elevate=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let configuration = get_configuration().expect("Failed to read configuration");
    tracing::debug!("config is {:?}", configuration);
    Application::build(configuration).run().await?;
    Ok(())
}
