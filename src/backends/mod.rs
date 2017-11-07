use core::{BackendChannel, BackendId, Event, MessageContent};
use serde_json::Value;
use std::sync::mpsc::Sender;

pub mod irc;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum BackendType {
    Irc,
    Slack,
    Discord,
}

quick_error! {
    #[derive(Debug)]
    pub enum BackendError {
        LoginDataInvalid {}
    }
}

pub type BackendResult<T> = Result<T, BackendError>;

pub trait BotBackend {
    fn get_type(&self) -> BackendType;
    fn connect(&mut self) -> BackendResult<()>;
    fn join(&mut self, channel: &str) -> BackendResult<()>;
    fn send(&mut self, dst: BackendChannel, msg: MessageContent) -> BackendResult<()>;
    fn reconnect(&mut self) -> BackendResult<()>;
}

pub trait BotBackendBuilder {
    type Backend: BotBackend;
    fn build_backend(
        backend_id: BackendId,
        sender: Sender<Event>,
        config: Option<Value>,
    ) -> Self::Backend;
}

#[cfg(test)]
mod test {
    use backends::*;

    #[test]
    fn test_object_safety() {
        // if this compiles, BotBackend can be used as a trait object
        let f = |a: &mut BotBackend| { a.reconnect().unwrap(); };
    }
}