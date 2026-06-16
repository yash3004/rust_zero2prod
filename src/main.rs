use zero2prod::configurations::get_configuration;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_config = get_configuration().expect("Failed to read configuration").application;
    let listener = std::net::TcpListener::bind(format!("{}:{}", app_config.host, app_config.port))?;
    run(listener)?.await
}
