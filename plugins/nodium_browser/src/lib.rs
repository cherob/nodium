use abi_stable::{StableAbi};
use nodium_pdk::{NodiumPlugin, NodiumPluginObject};

// CrateInfo and fetch_crates() implementation from previous answer

mod crate_info;
mod crate_view_table;
mod crates_fetch;

// use crates_fetch::crates_fetch;
// use crate_info::CrateInfo;
// use crate_view_table::crate_view_table;

#[repr(C)]
#[derive(StableAbi)]
pub struct NodiumPluginBrowser;

impl NodiumPlugin for NodiumPluginBrowser {
  fn name(&self) -> String {
      "Browser".to_string()
  }

  fn version(&self) -> String {
      "0.1.0".to_string()
  }

  fn as_object(&self) -> Box<dyn NodiumPluginObject> {
      Box::new(NodiumPluginBrowser)
  }
}

impl NodiumPluginBrowser {
  pub fn new() -> Self {
      NodiumPluginBrowser
  }
}


#[no_mangle]
pub extern "C" fn plugin() -> *mut NodiumPluginBrowser {
  Box::into_raw(Box::new(NodiumPluginBrowser::new()))
}

#[no_mangle]
pub extern "C" fn free_plugin(ptr: *mut NodiumPluginBrowser) {
    let _ = unsafe { Box::from_raw(ptr) };
}