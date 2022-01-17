use crate::ApiCred;
use exchange_sign_hook::SignClosure;

#[derive(Clone)]
pub struct Hook {
    pub api_key: String,
    pub closure: SignClosure,
}

impl Hook {
    pub fn new(api_key: String, closure: SignClosure) -> Self {
        Self { api_key, closure }
    }
}

#[derive(Clone)]
pub enum Signer {
    Cred(ApiCred),
    Hook(Hook),
}

impl From<ApiCred> for Signer {
    fn from(cred: ApiCred) -> Self {
        Signer::Cred(cred)
    }
}

impl From<Hook> for Signer {
    fn from(hook: Hook) -> Self {
        Signer::Hook(hook)
    }
}
