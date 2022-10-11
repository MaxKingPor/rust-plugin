# rust-plugin
* Rust 插件开发模板

# Quick Start
```Rust
use user_plugin_trait::{UserLibraryHandler, UserPlugin};

fn main() {
    let filename = if cfg!(target_os = "windows") {
        "user.dll"
    } else if cfg!(target_os = "linux") {
        "libuser.so"
    } else if cfg!(target_os = "macos") {
        "libuser.dylib"
    } else {
        panic!()
    };

    let mut stor = plugin::PluginStor::default();
    stor.load_library::<UserLibraryHandler, _>(filename)
        .unwrap();

    let userp = stor.get_plugin::<UserPlugin>().unwrap();
    let token = userp.login("user".into(), "password".into());
    println!("TokenIS: {}", token);
}


```