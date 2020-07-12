use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tide::{Body, Request, Response};

#[derive(Deserialize, Serialize)]
struct Fibonacci {
  number: usize,
  fibonacci: usize,
  duration: Duration,
}

trait IFibonacci {
  fn generate(n: usize) -> Fibonacci;
}

impl IFibonacci for Fibonacci {
  fn generate(n: usize) -> Fibonacci {
    let start = Instant::now();
    let fib = Fib::fib(n);
    let duration = Instant::now() - start;

    Fibonacci { number: n, fibonacci: fib, duration }
  }
}

struct Fib {}

trait IFib {
  fn fib(n: usize) -> usize;
}

impl IFib for Fib {
  fn fib(n: usize) -> usize {
    match n {
      0 => 0,
      1 => 1,
      _ => Fib::fib(n - 1) + Fib::fib(n - 2),
    }
  }
}

pub async fn get_fibonacci(req: Request<()>) -> tide::Result<Response> {
  let n: usize = req.param("n").unwrap_or(0);

  let mut res = Response::new(200);
  res.set_body(Body::from_json(&Fibonacci::generate(n))?);

  Ok(res)
}
