use anyhow::Result;
use chat_server::{get_router, AppConfig};
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
    let app_config = AppConfig::load()?;
    let addr = format!("0.0.0.0:{}", app_config.service.port);
    let api = get_router(app_config);
    let listener = TcpListener::bind(&addr).await?;
    info!("listener on {}", addr);
    axum::serve(listener, api.into_make_service()).await?;
    Ok(())
}

// #[cfg(test)]
// mod test {

//     #[test]
//     fn test() {
//         assert_eq!(1, 1);
//     }
// }
