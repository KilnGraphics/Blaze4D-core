use std::ffi::CStr;
use ash::vk;

pub trait DebugMessengerCallback: Send + Sync {
    fn on_message(
        &self,
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_types: vk::DebugUtilsMessageTypeFlagsEXT,
        message: &CStr,
        data: &vk::DebugUtilsMessengerCallbackDataEXT,
    );
}

pub struct RustLogDebugMessenger {
}

impl RustLogDebugMessenger {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl DebugMessengerCallback for RustLogDebugMessenger {
    fn on_message(&self, message_severity: vk::DebugUtilsMessageSeverityFlagsEXT, _: vk::DebugUtilsMessageTypeFlagsEXT, message: &CStr, _: &vk::DebugUtilsMessengerCallbackDataEXT) {
        if message_severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) {
            log::error!("{:?}", message);
        } else if message_severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING) {
            log::warn!("{:?}", message);
        } else if message_severity.contains(vk::DebugUtilsMessageSeverityFlagsEXT::INFO) {
            log::info!("{:?}", message);
        } else {
            log::info!("Unknown severity: {:?}", message);
        }
    }
}