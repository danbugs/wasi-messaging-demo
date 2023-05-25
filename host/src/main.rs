wasmtime::component::bindgen!({
    path: "../wit",
    world: "messaging",
    async: true,
});

use std::{thread, time::Duration};

use host::{add_to_linker, WasiCtx};
use messaging_types::MessagingTypes;
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

use crate::{consumer::Consumer, producer::Producer};

struct MyProducer;
struct MyConsumer;

struct MyTypes;

#[async_trait::async_trait]
impl MessagingTypes for MyTypes {
    async fn connect(
        &mut self,
        _: std::string::String,
    ) -> std::result::Result<std::result::Result<u32, u32>, anyhow::Error> {
        println!(">>> called connect");
        Ok(Ok(0))
    }

    async fn disconnect(&mut self, _: u32) -> std::result::Result<(), anyhow::Error> {
        println!(">>> called disconnect");
        Ok(())
    }

    async fn drop_error(&mut self, _: u32) -> std::result::Result<(), anyhow::Error> {
        println!(">>> called drop_error");
        Ok(())
    }

    async fn trace(&mut self, _: u32) -> std::result::Result<std::string::String, anyhow::Error> {
        println!(">>> called trace");
        Ok("".to_string())
    }
}

#[async_trait::async_trait]
impl Producer for MyProducer {
    async fn send(
        &mut self,
        c: u32,
        ch: messaging_types::Channel,
        msg: Vec<messaging_types::MessageResult>,
    ) -> std::result::Result<std::result::Result<(), u32>, anyhow::Error> {
        println!(">>> called publish");
        Ok(Ok(()))
    }
}

#[async_trait::async_trait]
impl Consumer for MyConsumer {
    async fn subscribe_try_receive(&mut self, c: u32, ch: String, ms_timeout: u32) -> std::result::Result<std::result::Result<std::option::Option<Vec<messaging_types::MessageResult>>, u32>, anyhow::Error> { todo!() }
    async fn subscribe_receive(&mut self, c: u32, ch: String) -> std::result::Result<std::result::Result<Vec<messaging_types::MessageResult>, u32>, anyhow::Error> { todo!() }
    async fn update_guest_configuration(&mut self, c: messaging_types::GuestConfiguration) -> std::result::Result<std::result::Result<(), u32>, anyhow::Error> { todo!() }
    async fn complete_message(&mut self, msg: messaging_types::MessageResult) -> std::result::Result<std::result::Result<(), u32>, anyhow::Error> { todo!() }
    async fn abandon_message(&mut self, msg: messaging_types::MessageResult) -> std::result::Result<std::result::Result<(), u32>, anyhow::Error> { todo!() }
}

pub struct Ctx {
    producer: MyProducer,
    consumer: MyConsumer,
    types: MyTypes,
    wasi: WasiCtx,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let producer = MyProducer;
    let consumer = MyConsumer;
    let types = MyTypes;

    let wasi = WasiCtxBuilder::new().build();

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;

    let mut store = Store::new(
        &engine,
        Ctx {
            producer,
            consumer,
            types,
            wasi,
        },
    );

    let mut linker = Linker::new(&engine);
    producer::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.producer)?;
    consumer::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.consumer)?;
    messaging_types::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.types)?;

    add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi)?;

    let component = Component::from_file(&engine, "guest.component.wasm")?;
    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    let res = messaging.guest.call_configure(&mut store).await?;

    // pretend to configure

    thread::sleep(Duration::from_secs(1));

    // pretend to have received a message
    let msg = messaging_types::MessageParam {
        data: &vec![1, 2, 3][..],
        format: messaging_types::FormatSpec::Http,
        metadata: None,
    };

    let res = messaging.guest.call_handler(&mut store, &vec![msg]).await?;

    println!(">>> called run: {:#?}", res);

    Ok(())
}
