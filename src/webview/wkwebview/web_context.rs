use crate::webview::CustomProtocolHandler;
use crate::{
  webview::web_context::WebContextData,
};

#[derive(Debug)]
pub struct WebContextImpl {
  protocols: Vec<*mut Box<CustomProtocolHandler>>,
}

impl WebContextImpl {
  pub fn new(_data: &WebContextData) -> Self {
    Self {
      protocols: Vec::new(),
    }
  }

  pub fn set_allows_automation(&mut self, _flag: bool) {}

  pub fn registered_protocols(&mut self, handler: *mut Box<CustomProtocolHandler>) {
    self.protocols.push(handler);
  }
}

impl Drop for WebContextImpl {
  fn drop(&mut self) {
    // We need to drop handler closures here
    unsafe {
      for ptr in self.protocols.iter() {
        if !ptr.is_null() {
          let _ = Box::from_raw(*ptr);
        }
      }
    }
  }
}
