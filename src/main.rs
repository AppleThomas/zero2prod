// use zero2prod::run;
use zero2prod::startup::run;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    // panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    run(listener)?.await

}