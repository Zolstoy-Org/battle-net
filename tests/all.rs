#[cfg(test)]
mod tests_battle_net {
    use core::str;
    use std::convert::Infallible;
    use std::net::SocketAddr;

    use battle_net::instance::Instance;
    use battle_net::instance::Locale;
    use battle_net::instance::Region;
    use http_body_util::BodyExt;
    use http_body_util::Full;
    use hyper::body::Bytes;
    use hyper::server::conn::http1;
    use hyper::service::service_fn;
    use hyper::{Request, Response};
    use hyper_util::rt::TokioIo;
    use tokio::net::TcpListener;

    async fn hello(
        request: Request<hyper::body::Incoming>,
    ) -> std::result::Result<Response<Full<Bytes>>, Infallible> {
        let tmp = request
            .into_body()
            .frame()
            .await
            .unwrap()
            .unwrap()
            .into_data()
            .unwrap()
            .to_vec();

        let body = str::from_utf8(&tmp).unwrap().to_string();

        println!("====>{body}");

        Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
    }

    async fn bootstrap_server() -> anyhow::Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 443));

        let listener = TcpListener::bind(addr).await?;

        tokio::task::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();

            let io = TokioIo::new(stream);

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

        let bnet_conn = Instance::new(Region::EU, "token1");
        assert_eq!(
            1,
            bnet_conn.get_auctions_by_realm_id(42, Locale::EnUs).await?
        );

        Ok(())
    }
}
