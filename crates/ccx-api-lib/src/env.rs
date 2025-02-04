use std::env::var;

pub fn env_var_with_prefix(prefix: &str, postfix: &str) -> Option<String> {
    var(format!("{}_{}", prefix, postfix)).ok()
}
