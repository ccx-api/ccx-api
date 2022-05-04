use std::convert::TryFrom;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use actix_http::Uri;
use actix_rt::net::TcpStream;
use actix_service::Service;
use actix_tls::connect::ConnectError;
use actix_tls::connect::ConnectInfo;
use actix_tls::connect::Connection;
use tokio_socks::tcp::Socks5Stream;
use tokio_socks::Error as SocksConnectError;
use tokio_socks::TargetAddr;
use tokio_socks::ToProxyAddrs;

fn to_connect_error(e: SocksConnectError) -> ConnectError {
    ConnectError::Io(std::io::Error::new(std::io::ErrorKind::Other, e))
}

#[derive(Clone, Debug)]
pub struct SocksConnector<P: ToProxyAddrs>(pub P);

impl<P: ToProxyAddrs + 'static> SocksConnector<P> {
    async fn connect(
        proxy: P,
        req: ConnectInfo<Uri>,
    ) -> Result<Connection<Uri, Socks5Stream<TcpStream>>, ConnectError> {
        let target = TargetAddr::Domain(req.hostname().into(), req.port());
        let res = Socks5Stream::connect(proxy, target)
            .await
            .map_err(to_connect_error)?;
        let uri = format!("{}:{}", req.hostname(), req.port());
        let uri = Uri::try_from(&uri)
            // .map_err(|e| LibError::other(format!("Failed to parse {:?}: {:?}", uri, e)))?
            // FIXME unwrap.
            .unwrap();
        Ok(Connection::new(uri, res))
    }
}

impl<P> Service<ConnectInfo<Uri>> for SocksConnector<P>
where
    P: ToProxyAddrs + Copy + 'static,
{
    type Response = Connection<Uri, Socks5Stream<TcpStream>>;
    type Error = ConnectError;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ConnectInfo<Uri>) -> Self::Future {
        Box::pin(Self::connect(self.0, req))
    }
}
