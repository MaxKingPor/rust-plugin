use std::{any::Any, collections::HashMap, ffi::OsStr, ops::Deref, sync::Arc};

use libloading::Library;

#[allow(unused)]
pub struct PluginInfo<P: Plugin> {
    plugin: Box<P>,
    lib: Option<Arc<Library>>,
}

impl<P: Plugin> PluginInfo<P> {
    pub fn new(plugin: Box<P>, lib: impl Into<Option<Arc<Library>>>) -> Self {
        Self { plugin, lib: lib.into() }
    }
}

impl<P: Plugin> Deref for PluginInfo<P> {
    type Target = Box<P>;

    fn deref(&self) -> &Self::Target {
        &self.plugin
    }
}

pub struct PluginStor {
    plugins: HashMap<&'static str, Arc<dyn Any + Sync + Send>>,
}

impl PluginStor {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::default(),
        }
    }

    pub fn load_library<H: LibraryHandler, F: AsRef<OsStr>>(
        &mut self,
        filename: F,
    ) -> Result<(), Error<H::Error>> {
        unsafe {
            let lib = Library::new(filename).map_err(Error::LibraryError)?;
            H::handler(Arc::new(lib), self)
        }
    }

    pub fn register_plugin<P: Plugin>(
        &mut self,
        plugin: Arc<PluginInfo<P>>,
    ) -> Option<Arc<PluginInfo<P>>> {
        self.plugins
            .insert(P::plugin_key(), plugin)
            .and_then(|old| old.downcast().ok())
    }

    pub fn get_plugin<P: Plugin>(&self) -> Result<Arc<PluginInfo<P>>, Error<()>> {
        self.plugins
            .get(P::plugin_key())
            .map_or(Err(Error::LoadPluginError(())), |v| {
                v.clone().downcast().map_err(Error::TypeError)
            })
    }
}

impl Default for PluginStor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum Error<E> {
    LibraryError(libloading::Error),
    LoadPluginError(E),
    TypeError(Arc<dyn Any + Send + Sync>),
}

pub trait Plugin: Sync + Send + Any {
    fn call(
        &self,
        args: Box<dyn Any + Sync + Send>,
    ) -> Result<Box<dyn Any + Sync + Send>, Box<dyn std::error::Error>>;
    fn plugin_key() -> &'static str;
}

pub trait LibraryHandler {
    type Error;
    fn handler(lib: Arc<Library>, stor: &mut PluginStor) -> Result<(), Error<Self::Error>>;
}
