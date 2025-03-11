use fibonacci::FibRequest;
use fibonacci::fibonacci_client::FibonacciClient;

pub mod fibonacci {
    tonic::include_proto!("fibonacci");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "http://[::1]:50051";
    let mut client = FibonacciClient::connect(addr).await?;

    let number: i64 = 90;
    let request = tonic::Request::new(FibRequest { number });
    let response = client.fib(request).await?;
    println!("\n\tfib({:?})={:?}\n", number, response.into_inner().result);

    Ok(())
}
