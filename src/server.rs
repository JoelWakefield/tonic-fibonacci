use std::collections::HashMap;

use tonic::{Request, Response, Status, transport::Server};

use fibonacci::fibonacci_server::{Fibonacci, FibonacciServer};
use fibonacci::{FibRequest, FibResponse};

pub mod fibonacci {
    tonic::include_proto!("fibonacci");
}

fn fib(n: i64) -> i64 {
    let mut memo = HashMap::<i64, i64>::new();

    for i in 1..n + 1 {
        memo.insert(
            i,
            if i <= 2 {
                1
            } else {
                memo.get(&(i - 1)).unwrap() + memo.get(&(i - 2)).unwrap()
            },
        );
    }

    return *memo.get(&n).unwrap();
}

#[derive(Debug, Default)]
pub struct FibonacciService {}

#[tonic::async_trait]
impl Fibonacci for FibonacciService {
    async fn fib(&self, request: Request<FibRequest>) -> Result<Response<FibResponse>, Status> {
        println!("Recieved request: {:?}", request);

        let response = FibResponse {
            result: fib(request.into_inner().number),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let fibonacci = FibonacciService::default();

    Server::builder()
        .add_service(FibonacciServer::new(fibonacci))
        .serve(addr)
        .await?;

    Ok(())
}
