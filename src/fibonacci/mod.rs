use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tide::{Body, Request, Response};

#[derive(Deserialize, Serialize)]
struct Fibonacci {
    number: usize,
    fibonacci: usize,
    duration: Duration,
}

pub async fn get_fibonacci(req: Request<()>) -> tide::Result<Response> {
    let n: usize = req.param("n").unwrap_or(0);
    let start = Instant::now();
    let fib_n = fib(n);
    let duration = Instant::now() - start;

    let fib = Fibonacci {
        number: n,
        fibonacci: fib_n,
        duration,
    };

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&fib)?);
    Ok(res)
}

fn fib(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
