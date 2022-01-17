use std::borrow::Cow;
use std::convert::TryFrom;
use std::future::Future;
use std::pin::Pin;
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
use tokio_socks::ToProxyAddrs;

fn to_connect_error(e: SocksConnectError) -> ConnectError {
    ConnectError::Io(std::io::Error::new(std::io::ErrorKind::Other, e))
}

async fn connect_socks<P: ToProxyAddrs + 'static>(
    proxy: P,
    req: Connect<Uri>,
) -> Result<Connection<Uri, Socks5Stream>, ConnectError> {
    let res = Socks5Stream::connect(
        proxy,
        TargetAddr::Domain(Cow::Borrowed(req.host()), req.port()),
    )
    .await
    .map_err(to_connect_error)?;
    Ok(Connection::new(
        res,
        Uri::try_from(format!("{}:{}", req.host(), req.port())).unwrap(), // since req.req is private >:(
    ))
}

#[derive(Clone, Debug)]
pub struct SocksConnector<P: ToProxyAddrs>(pub P);

impl<P> Service for SocksConnector<P>
where
    P: ToProxyAddrs + Copy + 'static,
{
    type Request = Connect<Uri>;
    type Response = Connection<Uri, Socks5Stream>;
    type Error = ConnectError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        Box::pin(connect_socks(self.0, req))
    }
}
