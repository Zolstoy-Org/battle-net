#![forbid(unsafe_code)]

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error")]
    GenericError,
}

pub struct Connection {
    token: String,
}

impl Connection {
    pub fn new(token: &str) -> Connection {
        Connection {
            token: token.to_string(),
        }
    }

    pub async fn get_nb_auctions(&self) -> u32 {
        // battle_net_oauth::get_oauth_token("", "", "US")

        0
    }
}

#[cfg(test)]
mod tests_battle_net {
    use std::convert::Infallible;
    use std::net::SocketAddr;

    use http_body_util::Full;
    use hyper::body::Bytes;
    use hyper::server::conn::http1;
    use hyper::service::service_fn;
    use hyper::{Request, Response};
    use hyper_util::rt::TokioIo;
    use tokio::net::TcpListener;

    use crate::Connection;

    async fn hello(
        _: Request<hyper::body::Incoming>,
    ) -> std::result::Result<Response<Full<Bytes>>, Infallible> {
        Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
    }

    async fn bootstrap_server() -> anyhow::Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

        // We create a TcpListener and bind it to 127.0.0.1:3000
        let listener = TcpListener::bind(addr).await?;

        // We start a loop to continuously accept incoming connections
        loop {
            let (stream, _) = listener.accept().await?;

            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);

            // Spawn a tokio task to serve multiple connections concurrently
            tokio::task::spawn(async move {
                // Finally, we bind the incoming connection to our `hello` service
                if let Err(err) = http1::Builder::new()
                    // `service_fn` converts our function in a `Service`
                    .serve_connection(io, service_fn(hello))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }

    #[tokio::test]
    async fn case_01_nb_auctions() -> anyhow::Result<()> {
        bootstrap_server().await?;

        let bnet_conn = Connection::new("test");
        assert_eq!(0, bnet_conn.get_nb_auctions().await);

        Ok(())
    }
}
