#![forbid(unsafe_code)]

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error")]
    GenericError,
}

pub struct Connection {
    region: Region,
    token: String,
}

pub enum Region {
    EU,
    US,
    APAC,
    CN,
}

impl Region {
    fn value(&self) -> &str {
        match *self {
            Region::EU => "eu",
            Region::US => "us",
            Region::APAC => "apac",
            Region::CN => "cn",
        }
    }
}

pub enum Locale {
    EnUs,
    EsMx,
    PtBr,
    DeDe,
    EnGb,
    EsEs,
    FrFr,
    ItIt,
    RuRu,
    KoKr,
    ZhTw,
    ZhCn,
}

impl Locale {
    fn value(&self) -> &str {
        match *self {
            Locale::EnUs => "en_US",
            Locale::EsMx => "es_MX",
            Locale::PtBr => "pt_BR",
            Locale::DeDe => "de_DE",
            Locale::EnGb => "en_GB",
            Locale::EsEs => "es_ES",
            Locale::FrFr => "fr_FR",
            Locale::ItIt => "it_IT",
            Locale::RuRu => "ru_RU",
            Locale::KoKr => "ko_KR",
            Locale::ZhTw => "zn_TW",
            Locale::ZhCn => "zh_CN",
        }
    }
}

enum Game {
    WoW,
}

impl Game {
    fn value(&self) -> &str {
        match *self {
            Game::WoW => "wow",
        }
    }
}
impl Connection {
    pub fn new(region: Region, token: &str) -> Connection {
        Connection {
            token: token.to_string(),
            region,
        }
    }

    fn get_uri(&self, game: Game, locale: Locale) -> String {
        format!("https://{region}.api.blizzard.com/data/{game}/connected-realm/106/auctions?namespace=dynamic-{region}&locale={locale}",
            region = self.region.value(), game = game.value(), locale = locale.value())
    }

    pub async fn get_nb_auctions(&self) -> u32 {}
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
        // loop {
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
        // }
        Ok(())
    }

    #[tokio::test]
    async fn case_01_nb_auctions() -> anyhow::Result<()> {
        bootstrap_server().await?;

        let bnet_conn = Connection::new("test");
        assert_eq!(0, bnet_conn.get_nb_auctions().await);

        Ok(())
    }
}
