use runtime::time::Delay;
use std::time::{Duration, Instant};

let start = Instant::now();
let now = Delay::new(Duration::from_secs(3)).await;

let elapsed = now - start;
println!("elapsed: {}s", elapsed.as_secs());
