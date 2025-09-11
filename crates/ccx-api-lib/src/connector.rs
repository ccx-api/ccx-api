use std::convert::TryFrom;
use std::future::Future;
use std::io::Error;
use std::io::ErrorKind;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;

use actix_http::Uri;
use actix_rt::net::ActixStream;
use actix_rt::net::Ready;
use actix_rt::net::TcpStream;
use actix_service::Service;
use actix_tls::connect::ConnectError;
use actix_tls::connect::ConnectInfo;
use actix_tls::connect::Connection;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::io::Interest;
use tokio::io::ReadBuf;
use tokio_socks::Error as SocksConnectError;
use tokio_socks::TargetAddr;
use tokio_socks::tcp::Socks5Stream;

fn to_connect_error(e: SocksConnectError) -> ConnectError {
    ConnectError::Io(Error::new(ErrorKind::Other, e))
}

#[derive(Clone, Debug)]
pub struct SocksConnector(pub Arc<str>);

impl SocksConnector {
    pub fn new(proxy: impl Into<Arc<str>>) -> Self {
        SocksConnector(proxy.into())
    }

    async fn connect(
        proxy: Arc<str>,
        req: ConnectInfo<Uri>,
    ) -> Result<Connection<Uri, W>, ConnectError> {
        let target = TargetAddr::Domain(req.hostname().into(), req.port());
        let res = Socks5Stream::connect(&*proxy, target)
            .await
            .map_err(to_connect_error)?;
        let uri = format!("{}:{}", req.hostname(), req.port());
        let uri = Uri::try_from(&uri)
            // .map_err(|e| LibError::other(format!("Failed to parse {:?}: {:?}", uri, e)))?
            // FIXME unwrap.
            .unwrap();
        Ok(Connection::new(uri, W(res)))
    }
}

impl Service<ConnectInfo<Uri>> for SocksConnector {
    type Response = Connection<Uri, W>;
    type Error = ConnectError;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ConnectInfo<Uri>) -> Self::Future {
        Box::pin(Self::connect(self.0.clone(), req))
    }
}

#[derive(Debug)]
pub struct W(Socks5Stream<TcpStream>);

impl AsyncRead for W {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        AsyncRead::poll_read(Pin::new(&mut self.0), cx, buf)
    }
}

impl AsyncWrite for W {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        AsyncWrite::poll_write(Pin::new(&mut self.0), cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        AsyncWrite::poll_flush(Pin::new(&mut self.0), cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Error>> {
        AsyncWrite::poll_shutdown(Pin::new(&mut self.0), cx)
    }
}

impl ActixStream for W {
    fn poll_read_ready(&self, cx: &mut Context<'_>) -> Poll<std::io::Result<Ready>> {
        let ready = self.0.ready(Interest::READABLE);
        tokio::pin!(ready);
        ready.poll(cx)
    }

    fn poll_write_ready(&self, cx: &mut Context<'_>) -> Poll<std::io::Result<Ready>> {
        let ready = self.0.ready(Interest::WRITABLE);
        tokio::pin!(ready);
        ready.poll(cx)
    }
}
