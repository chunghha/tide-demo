use anyhow::Result;
use async_std::task;
use tide::log;

mod fibonacci;

fn main() -> Result<()> {
    log::start();

    task::block_on(async {
        let mut app = tide::new();
        app.at("/fib/:n").get(fibonacci::get_fibonacci);
        app.listen("0.0.0.0:8080").await?;
        Ok(())
    })
}
