use std::borrow::Cow;
use std::collections::HashMap;
use std::future::Future;
use std::mem;
use std::sync::Arc;

use futures::channel::mpsc as fmpsc;
use futures::channel::oneshot;
use futures::SinkExt;
use futures::StreamExt;
use serde::Deserialize;
use serde::Serialize;
use smart_string::DisplayExt;
use smart_string::PascalString;
use soketto::handshake::Client;
use soketto::handshake::ServerResponse;
use soketto::Data;
use soketto::Incoming;
use tokio::net::lookup_host;
use tokio::net::TcpStream;
use tokio_rustls::rustls::pki_types::ServerName;
use tokio_rustls::rustls::ClientConfig;
use tokio_rustls::rustls::RootCertStore;
use tokio_rustls::TlsConnector;
use tokio_util::compat::Compat;
use tokio_util::compat::TokioAsyncReadCompatExt;
use url::Url;

use crate::spot::types::ws_requests::IgnorePayload;
use crate::spot::types::ws_requests::WsMessageId;
use crate::spot::types::ws_requests::WsMeta;
use crate::spot::types::ws_stream_name::StreamName;

pub struct WebSocketClient {
    cmd_tx: fmpsc::Sender<Command>,
}

struct Command {
    cmd: WsCommand,
    tx: oneshot::Sender<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub enum WsCommand {
    SubscribeOne([StreamName; 1]),
    SubscribeList(Vec<StreamName>),
    UnsubscribeOne([StreamName; 1]),
    UnsubscribeList(Vec<StreamName>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(
    tag = "method",
    content = "params",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum StreamCommand<'a> {
    Subscribe(Cow<'a, [StreamName]>),
    Unsubscribe(Cow<'a, [StreamName]>),
}

#[derive(Debug, thiserror::Error)]
pub enum WebSocketConnectError {
    #[error("Missing hostname")]
    MissingHostname,
    #[error("Missing port")]
    MissingPort,
    #[error("Bad hostname")]
    BadHostname,
    #[error("Bad url")]
    BadUrl(#[from] url::ParseError),
    #[error("IO error {0}")]
    Io(#[from] std::io::Error),
    #[error("Handshake error {0}")]
    Handshake(#[from] soketto::handshake::Error),
    #[error("Redirected to {location} with status code {status_code}")]
    Redirect { status_code: u16, location: String },
    #[error("Rejected with status code {status_code}")]
    Rejected { status_code: u16 },
}

impl WebSocketClient {
    pub fn connect(
        stream_url: Url,
    ) -> impl Future<Output = Result<(Self, fmpsc::Receiver<Vec<u8>>), WebSocketConnectError>> {
        async move {
            println!("Establishing connection to {stream_url}");

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

            // println!("resolving {host_addr}");

            let mut root_cert_store = RootCertStore::empty();
            root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
            let config = ClientConfig::builder()
                .with_root_certificates(root_cert_store)
                .with_no_client_auth();
            let connector = TlsConnector::from(Arc::new(config));
            let dnsname = ServerName::try_from(host.to_string()).unwrap();

            let stream = TcpStream::connect(host_addr.as_str()).await?;
            let mut stream = connector.connect(dnsname, stream).await?;

            // let socket = {
            //     let mut last_error = None;
            //     let mut addrs = lookup_host(&*host_addr).await?;
            //     loop {
            //         if let Some(addr) = addrs.next() {
            //             match TcpStream::connect(addr).await {
            //                 Ok(socket) => {
            //                     println!("connected to addr {}", addr);
            //                     break socket;
            //                 }
            //                 Err(e) => {
            //                     println!("connect to addr {} failed: {}", addr, e);
            //                     last_error = Some(e)
            //                 }
            //             }
            //         } else {
            //             Err(last_error.take().unwrap_or_else(|| {
            //                 std::io::Error::new(
            //                     std::io::ErrorKind::AddrNotAvailable,
            //                     "no addresses to connect to",
            //                 )
            //             }))?
            //         }
            //     }
            // };
            let resource = match stream_url.query() {
                Some(q) => format!("{}?{}", stream_url.path(), q),
                None => stream_url.path().to_owned(),
            };

            println!("requesting {host}, {resource}");
            let mut client = Client::new(stream.compat(), &host, &resource);

            let (sender, mut receiver) = match client.handshake().await? {
                ServerResponse::Accepted { .. } => client.into_builder().finish(),
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
            };
            let (cmd_tx, cmd_rx) = fmpsc::channel(4);
            let (stream_tx, stream_rx) = fmpsc::channel(4);
            tokio::spawn(async move {
                upstream_loop(sender, receiver, cmd_rx, stream_tx).await;
            });

            Ok((Self { cmd_tx }, stream_rx))
        }
    }
}

async fn upstream_loop(
    mut sender: soketto::Sender<impl futures::AsyncRead + futures::AsyncWrite + Unpin>,
    mut receiver: soketto::Receiver<impl futures::AsyncRead + futures::AsyncWrite + Unpin>,
    mut cmd_rx: fmpsc::Receiver<Command>,
    mut stream_tx: fmpsc::Sender<Vec<u8>>,
) {
    let mut next_id = 1;
    let mut requests: HashMap<i64, oneshot::Sender<Vec<u8>>> = HashMap::new();
    let mut buf = Vec::new();
    loop {
        tokio::select! {
            cmd = cmd_rx.next() => {
                let Command { cmd, tx } = match cmd {
                    Some(cmd) => cmd,
                    None => {
                        println!("command channel closed");
                        break;
                    }
                };
                let id = next_id;
                next_id += 1;
                requests.insert(id, tx);
                println!("command: {id} {cmd:?}");
                let message = serde_json::to_string(&WsMeta {
                    id: Some(WsMessageId::Int(id)),
                    payload: match &cmd {
                        WsCommand::SubscribeOne(list) => StreamCommand::Subscribe((&*list).into()),
                        WsCommand::SubscribeList(list) => StreamCommand::Subscribe((&*list).into()),
                        WsCommand::UnsubscribeOne(list) => StreamCommand::Unsubscribe((&*list).into()),
                        WsCommand::UnsubscribeList(list) => StreamCommand::Unsubscribe((&*list).into()),
                    }
                }).unwrap();
                sender.send_text(&message).await.unwrap();
            }
            message = receiver.receive(&mut buf) => {
                match message {
                    Ok(incoming) => match incoming {
                        Incoming::Data(data) => {
                            let count = match data {
                                Data::Text(count) | Data::Binary(count) => count,
                            };
                            if cfg!(feature = "debug_communication") {
                                println!("received {count} bytes");
                            }
                            debug_assert_eq!(count, buf.len());
                            let meta = preparse_meta(&buf).unwrap();
                            if let Some(WsMessageId::Int(id)) = meta.id {
                                if let Some(tx) = requests.remove(&id) {
                                    let _ = tx.send(mem::take(&mut buf));
                                }
                            } else {
                                let _ = stream_tx.send(mem::take(&mut buf)).await;
                            }
                        }
                        Incoming::Pong(_data) => {
                            if cfg!(feature = "debug_communication") {
                                println!("received pong");
                            }
                        }
                        Incoming::Closed(close_reason) => {
                            println!("connection closed: {:?}", close_reason);
                            break;
                        }
                    },
                    Err(reason) => {
                        println!("error receiving message: {:?}", reason);
                        break;
                    }
                }
            }
        }
    }
}

fn preparse_meta(data: &[u8]) -> Result<WsMeta<IgnorePayload>, serde_json::Error> {
    Ok(serde_json::from_slice(data)?)
}
