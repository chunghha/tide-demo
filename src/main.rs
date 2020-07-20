use anyhow::Result;

mod fibonacci;

#[async_std::main]
async fn main() -> Result<()> {
  tide::log::start();

  let mut app = tide::new();
  app.at("/fib/:n").get(fibonacci::get_fibonacci);
  app.listen("0.0.0.0:8080").await?;
  Ok(())
}
