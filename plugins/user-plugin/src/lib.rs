use user_plugin_trait::UserPluginTrait;

use jsonwebtoken::{encode, EncodingKey, Header};

struct UserPlugin {}

impl UserPluginTrait for UserPlugin {
    fn login(&self, user: String, password: String) -> String {
        format!("UserPlugin#######:{}-{}", user, password)
    }

    fn gen_token(&self, arg: &user_plugin_trait::Claims) -> String {
        encode(
            &Header::default(),
            arg,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap()
    }
}

#[no_mangle]
pub fn get_user_plugin_trait() -> Box<dyn UserPluginTrait> {
    Box::new(UserPlugin {})
}

#[cfg(windows)]
use std::ffi::c_void;
#[cfg(windows)]
use windows::Win32::Foundation::HINSTANCE;

#[cfg(windows)]
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(
    hinstDLL: HINSTANCE,
    fdwReason: u32,
    lpvReserved: *const c_void,
) -> bool {
    use windows::Win32::System::SystemServices::{
        DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
    };

    match fdwReason {
        DLL_PROCESS_ATTACH => {
            println!("Dll {:?} DLL_PROCESS_ATTACH {:p}", hinstDLL, lpvReserved);
        }
        DLL_PROCESS_DETACH => {
            println!("Dll {:?} DLL_PROCESS_DETACH {:p}", hinstDLL, lpvReserved);
        }
        DLL_THREAD_ATTACH => {
            println!("Dll {:?} DLL_THREAD_ATTACH {:p}", hinstDLL, lpvReserved);
        }
        DLL_THREAD_DETACH => {
            println!("Dll {:?} DLL_THREAD_DETACH {:p}", hinstDLL, lpvReserved);
        }
        _ => panic!(),
    };
    true
}
