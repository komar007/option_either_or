use std::{
    future::{Future, Pending},
    time::Duration,
};

use option_either_or::OptionEitherOr;

async fn await_or_sleep(f: Option<impl Future<Output = ()>>) {
    f.either_or_else(|| async move {
        println!("doing nothing by default and sleeping...");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("...slept");
    })
    .await
}

#[tokio::main]
async fn main() {
    await_or_sleep(Some(async move {
        for _ in 0..4 {
            println!("something real");
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }))
    .await;

    println!("---");

    await_or_sleep(Option::<Pending<()>>::None).await;
}
