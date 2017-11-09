use core::{Channel, Message};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumeEventHandling {
    Stop,
    Resume,
}

pub enum PluginEvent {
    None(ResumeEventHandling),
    Log(String, ResumeEventHandling),
    Send(Message, ResumeEventHandling),
}

pub trait Plugin {
    fn create(config: Option<Value>) -> Self
    where
        Self: Sized;
    fn plugin_priority(&self, msg: Message) -> i16;
    fn handle_command(&mut self, user: &str, channel: Channel, params: Vec<String>) -> PluginEvent;
    fn handle_message(&mut self, data: Message) -> PluginEvent;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PluginType {
    RandomChat,
    MessagePasser,
}
