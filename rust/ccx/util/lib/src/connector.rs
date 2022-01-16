use std::borrow::Cow;
use std::convert::TryFrom;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

use actix_connect::Connect;
use actix_connect::ConnectError;
use actix_connect::Connection;
use actix_http::http::Uri;
use actix_service::Service;
use tokio_socks::tcp::Socks5Stream;
use tokio_socks::Error as SocksConnectError;
use tokio_socks::TargetAddr;

fn to_connect_error(e: SocksConnectError) -> ConnectError {
    ConnectError::Io(std::io::Error::new(std::io::ErrorKind::Other, e))
}

#[derive(Clone, Debug)]
pub struct SocksConnector {
    addr: Arc<str>,
}

impl SocksConnector {
    pub fn new(addr: impl Into<Arc<str>>) -> Self {
        let addr = addr.into();
        SocksConnector { addr }
    }

    async fn connect(
        proxy: Arc<str>,
        req: Connect<Uri>,
    ) -> Result<Connection<Uri, Socks5Stream>, ConnectError> {
        let target = TargetAddr::Domain(Cow::Borrowed(req.host()), req.port());
        let res = Socks5Stream::connect(proxy.as_ref(), target)
            .await
            .map_err(to_connect_error)?;
        Ok(Connection::new(
            res,
            Uri::try_from(format!("{}:{}", req.host(), req.port())).unwrap(),
        ))
    }
}

impl Service for SocksConnector {
    type Request = Connect<Uri>;
    type Response = Connection<Uri, Socks5Stream>;
    type Error = ConnectError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        Box::pin(Self::connect(self.addr.clone(), req))
    }
}
