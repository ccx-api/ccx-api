use std::sync::Arc;
use std::time::Duration;

pub use awc::Client;
pub use awc::ClientRequest;
pub use awc::ClientResponse;
pub use awc::Connector;
pub use rustls::ClientConfig;
pub use rustls::OwnedTrustAnchor;
pub use rustls::RootCertStore;

pub use crate::Proxy;

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

pub fn client_config(h1_only: bool) -> Arc<ClientConfig> {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let mut cfg = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    if h1_only {
        cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
    }
    Arc::new(cfg)
}

pub fn make_client(h1_only: bool, proxy: Option<&Proxy>) -> Client {
    let cfg = client_config(h1_only);
    match proxy {
        Some(proxy) => client_with_proxy(cfg, proxy),
        None => client_without_proxy(cfg),
    }
}

pub fn client_without_proxy(cfg: Arc<ClientConfig>) -> Client {
    let connector = Connector::new().rustls(cfg).timeout(CONNECT_TIMEOUT);
    Client::builder()
        .connector(connector)
        .timeout(CLIENT_TIMEOUT)
        .finish()
}

pub fn client_with_proxy(_cfg: Arc<rustls::ClientConfig>, _proxy: &Proxy) -> Client {
    // let connector = Connector::new()
    //     .rustls(cfg)
    //     .connector(SocksConnector::new(proxy.addr()))
    //     .timeout(CONNECT_TIMEOUT);
    // Client::builder()
    //     .connector(connector)
    //     .timeout(CLIENT_TIMEOUT)
    //     .finish()
    todo!("FIX client_with_proxy")
}
