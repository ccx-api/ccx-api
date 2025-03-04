use std::sync::Arc;

use smart_string::{DisplayExt, PascalString};
use soketto::connection::Builder;
use soketto::handshake::Client;
use soketto::handshake::ServerResponse;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;
use tokio_rustls::rustls::ClientConfig;
use tokio_rustls::rustls::RootCertStore;
use tokio_rustls::rustls::pki_types::ServerName;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};
use url::Url;

#[derive(Debug, derive_more::Display, derive_more::From, derive_more::Error)]
pub enum WebSocketConnectError {
    #[display("Missing hostname")]
    MissingHostname,
    #[display("Missing port")]
    MissingPort,
    #[display("Bad hostname")]
    BadHostname,
    #[display("Bad url")]
    BadUrl(url::ParseError),
    #[display("IO error {_0}")]
    Io(std::io::Error),
    #[display("Handshake error {_0}")]
    Handshake(soketto::handshake::Error),
    #[display("Redirected to {location} with status code {status_code}")]
    Redirect { status_code: u16, location: String },
    #[display("Rejected with status code {status_code}")]
    Rejected { status_code: u16 },
}

#[tracing::instrument(skip_all, fields(stream_url = %stream_url))]
pub async fn websocket_builder(
    stream_url: &Url,
) -> Result<Builder<Compat<TlsStream<TcpStream>>>, WebSocketConnectError> {
    tracing::debug!("Establishing connection to {stream_url}");

    let host = stream_url
        .host_str()
        .ok_or(WebSocketConnectError::MissingHostname)?;
    let port = stream_url
        .port()
        .or_else(|| match stream_url.scheme() {
            "ws" => Some(80),
            "wss" => Some(443),
            _ => None,
        })
        .ok_or(WebSocketConnectError::MissingPort)?;

    let host_addr: PascalString<255> = format_args!("{host}:{port}")
        .try_to_fmt()
        .map_err(|_| WebSocketConnectError::BadHostname)?;

    tracing::debug!("resolving {host_addr}");

    let mut root_cert_store = RootCertStore::empty();
    root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();
    let connector = TlsConnector::from(Arc::new(config));
    let dnsname = ServerName::try_from(host.to_string()).unwrap();

    let stream = TcpStream::connect(host_addr.as_str()).await?;
    let stream = connector.connect(dnsname, stream).await?;

    let resource = match stream_url.query() {
        Some(q) => format!("{}?{}", stream_url.path(), q),
        None => stream_url.path().to_owned(),
    };

    tracing::debug!("requesting {host}, {resource}");
    let mut client = Client::new(stream.compat(), &host, &resource);

    match client.handshake().await? {
        ServerResponse::Accepted { .. } => Ok(client.into_builder()),
        ServerResponse::Redirect {
            status_code,
            location,
        } => {
            return Err(WebSocketConnectError::Redirect {
                status_code,
                location,
            });
        }
        ServerResponse::Rejected { status_code } => {
            return Err(WebSocketConnectError::Rejected { status_code });
        }
    }
}
