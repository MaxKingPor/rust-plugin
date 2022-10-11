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
