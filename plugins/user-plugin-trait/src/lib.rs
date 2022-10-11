// use std::ffi::*;

use std::{sync::Arc, any::Any, ops::Deref};

use libloading::Library;
use plugin::{LibraryHandler, Plugin, PluginInfo, PluginStor};
use serde::{Deserialize, Serialize};

#[allow(unused)]
pub struct UserPlugin {
    symbol: Box<dyn UserPluginTrait>,
}

impl Deref for UserPlugin {
    type Target = Box<dyn UserPluginTrait>;

    fn deref(&self) -> &Self::Target {
        &self.symbol
    }
}

#[allow(unused)]
pub struct UserLibraryHandler {
    lib: Arc<Library>,
}

unsafe impl Send for UserPlugin {}
unsafe impl Sync for UserPlugin {}

impl Plugin for UserPlugin {
    fn plugin_key() -> &'static str {
        "UserPluginTrait"
    }

    fn call(
        &self,
        _: Box<dyn Any + Sync + Send>,
    ) -> Result<Box<dyn Any + Sync + Send>, Box<dyn std::error::Error>> {
        todo!("Plugin::call")
    }
}

impl LibraryHandler for UserLibraryHandler {
    type Error = ();

    fn handler(lib: Arc<Library>, stor: &mut PluginStor) -> Result<(), plugin::Error<Self::Error>> {
        unsafe {
            let init_fn = lib
                .get::<fn() -> Box<dyn UserPluginTrait>>(b"get_user_plugin_trait\0")
                .map_err(plugin::Error::LibraryError)?;
            let symbol = init_fn();
            stor.register_plugin(Arc::new(PluginInfo::new(
                Box::new(UserPlugin { symbol }),
                lib,
            )));

            Ok(())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

pub trait UserPluginTrait {
    fn login(&self, user: String, password: String) -> String;
    fn gen_token(&self, arg: &Claims) -> String;
}
