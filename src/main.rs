pub mod demo;
use demo::demo_interface_client::DemoInterfaceClient;
use demo::PingRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DemoInterfaceClient::connect("http://127.0.0.1:8080").await?;

    let request = tonic::Request::new(PingRequest{
        message: "hello".into(),
    });
    let response = client.ping(request).await?;
    println!("response: {:?}", response);

    Ok(())
}