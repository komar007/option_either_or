# `option_either_or`

![Crates.io License](https://img.shields.io/crates/l/option_either_or)
[![Crates.io
Version](https://img.shields.io/crates/v/option_either_or)](https://crates.io/crates/option_either_or/)
![GitHub branch check runs](https://img.shields.io/github/check-runs/komar007/option_either_or/main)
[![docs.rs](https://img.shields.io/docsrs/option_either_or)](https://docs.rs/option_either_or)

Convert `Option<T>` into `Either<L, T>`.

## Overview

This crate introduces `either_or` and `either_or_else` to `Option` in order to facilitate
conversions from `Option` to `Either`.

It is mainly useful as generalizations of `unwrap_or`/`unwrap_or_else` where the provided default
value is of a different type than the `Some` variant, but both types implement some common trait.

## Examples

You can take advantage of `Either` implementing `Display` to provide a default value to
`Option<String>` without allocating another string...

```rust
use option_either_or::OptionEitherOr as _;
let x = Some(String::from("string"));
let y = x.either_or("default");
println!("{y}");
```

... or provide defaults to `impl Display` (or any other trait generically implemented by `Either`)
in a generic context...

```rust
fn transform(x: Option<impl Display>) -> impl Display {
    use option_either_or::OptionEitherOr as _;

    x.either_or_else(|| "default")
}
```

... or even provide defaults to a generic `Future`, like so:

```rust
use std::{future::Pending, time::Duration};

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
```
