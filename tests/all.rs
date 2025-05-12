#[cfg(test)]
mod tests_battle_net {
    use std::convert::Infallible;
    use std::net::SocketAddr;

    use battle_net::Connection;
    use battle_net::RegionUriPart;
    use http_body_util::Full;
    use hyper::body::Bytes;
    use hyper::server::conn::http1;
    use hyper::service::service_fn;
    use hyper::{Request, Response};
    use hyper_util::rt::TokioIo;
    use tokio::net::TcpListener;

    async fn hello(
        _: Request<hyper::body::Incoming>,
    ) -> std::result::Result<Response<Full<Bytes>>, Infallible> {
        Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
    }

    async fn bootstrap_server() -> anyhow::Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 443));

        let listener = TcpListener::bind(addr).await?;

        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(hello))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });

        Ok(())
    }

    #[tokio::test]
    async fn case_01_nb_auctions() -> anyhow::Result<()> {
        bootstrap_server().await?;

        let bnet_conn = Connection::new(RegionUriPart::EU, "token1");
        assert_eq!(
            0,
            bnet_conn
                .get_auctions_by_realm_id(42, battle_net::LocaleUriPart::EnUs)
                .await?
        );

        Ok(())
    }
}
