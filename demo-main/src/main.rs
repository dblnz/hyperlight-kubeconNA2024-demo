mod hyperlight;
mod non_hyperlight;

use warp::Filter;

#[cfg(not(any(feature = "gdb", feature = "crashdump")))]
const DEMO_GUEST_PATH: &str = "./demo-guest";
#[cfg(any(feature = "gdb", feature = "crashdump"))]
const DEMO_GUEST_PATH: &str = "./demo-guest-debug";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Warm up the sandbox pool
    hyperlight::warm_up_pool().await;

    let routes = non_hyperlight::hello_world()
        .or(hyperlight::get_vm_count())
        .or(hyperlight::hello_world::cold())
        .or(hyperlight::hello_world::warm())
        .or(hyperlight::safety::deref_raw_null_ptr())
        .or(hyperlight::safety::stack_overflow());

    println!("Listening at http://127.0.0.1:3030/ 🚀...");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
