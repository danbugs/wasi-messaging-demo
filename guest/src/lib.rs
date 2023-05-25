use crate::messaging_types::{GuestConfigurationResult, MessageResult};

wit_bindgen::generate!({
    path: "../wit", 
});

struct MyGuest;

impl guest::Guest for MyGuest {
    fn configure() -> Result<GuestConfigurationResult, u32> {
        println!("host called configure");

        let gcr = GuestConfigurationResult {
            channels: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            extensions: None,
        };

        Ok(gcr)
    }

    fn handler(ms: Vec<MessageResult>) -> Result<(), u32> {
        println!("handling message");

        Ok(())
    }
}

export_messaging!(MyGuest);