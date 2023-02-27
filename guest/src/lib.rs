use messaging::HandlerError;

use crate::pubsub::{open_broker, publish};

wit_bindgen_guest_rust::generate!({
    path: "../wit/messaging.wit",
});

struct MyMessaging;

impl messaging::Messaging for MyMessaging {
    fn on_receive(e: String) -> Result<(), HandlerError> {
        println!(">>> Received: {:#?}", &e);
        // process the data
        let msg = fizz_buzz(&e);

        // published the processed data
        let b = open_broker("my-messaging").unwrap();

        println!(">>> Publishing: {:#?}", &msg);
        publish(b, "rust", &msg).unwrap();

        Ok(())
    }
}

export_messaging!(MyMessaging);

// replace all instances of fizz in a word w/ buzz
fn fizz_buzz(word: &str) -> String {
    word.replace("fizz", "buzz")
}
