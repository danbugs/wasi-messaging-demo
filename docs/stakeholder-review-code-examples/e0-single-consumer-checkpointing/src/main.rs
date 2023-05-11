use std::fs;

fn main() {
    // get NATS_CREDS from environment
    let nats_creds = std::env::var("NATS_CREDS").unwrap();

    // create a connection to the NATS server
    let connection = nats::Options::with_credentials(&nats_creds)
        .connect("connect.ngs.global")
        .unwrap();

    // subscribe to the topic
    let topic_name = "stakeholder-review";
    let sub = connection.subscribe(topic_name).unwrap();

    // grab simulated checkpoint from file
    let mut checkpoint: Option<String> = None;
    if let Ok(saved_checkpoint) = fs::read_to_string("./checkpoint.txt")
    {
        checkpoint = Some(saved_checkpoint);
    }

    loop {
        let recv = sub.next().unwrap();

        if let Some(ref cp) = checkpoint {
            // Note: I'm using the message data to match on a checkpoint,
            // but realistically we'd have some ID annotating
            // the message that we could use to match on as, this
            // way, it ignores duplicate messages that could be valid.
            if recv.data[..] == cp.as_bytes()[..] {
                println!("Checkpoint matched, ignoring message");
                continue;
            }
        }

        // process message, or whatever
        let msg = String::from_utf8(recv.data).unwrap();
        println!("Received: {}", msg);

        // save checkpoint
        checkpoint = Some(msg);
        fs::write("./checkpoint.txt", checkpoint.as_ref().unwrap()).unwrap();

        // sleep for 2 seconds waiting for next message
        println!("Sleeping for 2 seconds...");
        std::thread::sleep(std::time::Duration::from_secs(2));
    }


}
