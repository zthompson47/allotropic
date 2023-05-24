// This node creates a worker, sends it a message, and receives a reply.

use hello_ockam::Echoer;
use ockam::{Context, Result};

#[ockam::node]
async fn main(mut ctx: Context) -> Result<()> {
    // Start a worker, of type Echoer, at address "echoer"
    ctx.start_worker("echoer", Echoer).await?;

    // Send a message to the worker at address "echoer"
    ctx.send("echoer", "Hello Ockam!".to_string()).await?;

    // Wait to receive a reply and print it.
    let reply = ctx.receive::<String>().await?;
    println!("App received: {}", reply); // should print "Hello Ockam!"

    // Stop all workers, stop the node, clean up, and return.
    ctx.stop().await
}
