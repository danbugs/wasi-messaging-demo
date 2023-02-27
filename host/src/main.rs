wasmtime::component::bindgen!({
    path: "../wit/messaging.wit",
    async: true,
});

use host::{add_to_linker, WasiCtx};
use wasi_cap_std_sync::WasiCtxBuilder;
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

struct MyPubsub;

#[async_trait::async_trait]
impl pubsub::Pubsub for MyPubsub {
    async fn open_broker(
        &mut self,
        _: std::string::String,
    ) -> std::result::Result<u32, wasmtime::component::Error<pubsub::Error>> {
        println!(">>> called open_broker");
        Ok(0)
    }

    async fn drop_broker(&mut self, _: u32) -> std::result::Result<(), anyhow::Error> {
        println!(">>> called drop_broker");
        Ok(())
    }

    async fn publish(
        &mut self,
        _: u32,
        _: pubsub::Channel,
        _: std::string::String,
    ) -> std::result::Result<(), wasmtime::component::Error<pubsub::Error>> {
        println!(">>> called publish");
        Ok(())
    }

    async fn subscribe(
        &mut self,
        _: u32,
        _: pubsub::Channel,
    ) -> std::result::Result<std::string::String, wasmtime::component::Error<pubsub::Error>> {
        println!(">>> called subscribe");
        Ok("".to_string())
    }

    async fn unsubscribe(
        &mut self,
        _: u32,
        _: std::string::String,
    ) -> std::result::Result<(), wasmtime::component::Error<pubsub::Error>> {
        println!(">>> called unsubscribe");
        Ok(())
    }
}

pub struct Ctx {
    pubsub: MyPubsub,
    wasi: WasiCtx,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pubsub = MyPubsub;

    let wasi = WasiCtxBuilder::new().build();

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;

    let mut store = Store::new(&engine, Ctx { pubsub, wasi });

    let mut linker = Linker::new(&engine);
    pubsub::add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.pubsub)?;

    add_to_linker(&mut linker, |ctx: &mut Ctx| &mut ctx.wasi)?;

    let component = Component::from_file(&engine, "guest.component.wasm")?;
    let (messaging, _) = Messaging::instantiate_async(&mut store, &component, &linker).await?;

    let res = messaging
        .on_receive(&mut store, &"fizzbuzz".to_string())
        .await?;

    println!(">>> called on_receive: {:#?}", res);

    Ok(())
}
