use crate::ApiCred;
use exchange_sign_hook::SignClosure;

#[derive(Clone)]
pub struct KeyClosure {
    pub api_key: String,
    pub closure: SignClosure,
}

impl KeyClosure {
    pub fn new(api_key: String, closure: SignClosure) -> Self {
        Self { api_key, closure }
    }
}

#[derive(Clone)]
pub enum Signer {
    Cred(ApiCred),
    Hook(KeyClosure),
}

impl From<ApiCred> for Signer {
    fn from(cred: ApiCred) -> Self {
        Signer::Cred(cred)
    }
}

impl From<KeyClosure> for Signer {
    fn from(closure: KeyClosure) -> Self {
        Signer::Hook(closure)
    }
}
