use oxide_ircd::helpers::{get_subscriber, init_subscriber};
use oxide_ircd::ircd::ircd::run;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("oxide_ircd".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);

    let listener = TcpListener::bind("127.0.0.1:6667").await.expect("Failed to bind to port 23");

    if let Err(e) = run(listener).await {
        tracing::error!("Application error: {}", e);
        std::process::exit(1);
    }
}
