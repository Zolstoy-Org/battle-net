use std::vec;

use reqwest::Certificate;

use crate::error::Error;
use crate::result::Result;

pub struct Instance {
    token: String,
    region: Region,
    address: String,
    ca_cert: Vec<u8>,
    port: u16,
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

enum WoW {
    Auctions(u32),
}

enum Game {
    WoW(WoW),
}

impl Game {
    fn value(&self) -> String {
        match self {
            Game::WoW(route) => {
                format!(
                    "wow/{route}",
                    route = match *route {
                        WoW::Auctions(realm_id) => {
                            format!("connected-realm/{realm_id}/auctions")
                        }
                    }
                )
            }
        }
    }
}

impl Instance {
    pub fn new(region: Region, token: &str) -> Instance {
        Instance {
            token: token.to_string(),
            address: format!("{}.api.blizzard.com", region.value()).to_string(),
            region,
            ca_cert: vec![],
            port: 443,
        }
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    pub fn set_ca_cert<'a>(&mut self, pem_cert_slice: Vec<u8>) {
        self.ca_cert = pem_cert_slice;
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    fn get_uri(&self, game_route: Game, locale: Locale) -> String {
        format!("https://{address}:{port}/data/{game_route_part}/connected-realm/106/auctions?namespace=dynamic-{region}&locale={locale}",
            address = self.address, port = self.port, region = self.region.value(), game_route_part = game_route.value(), locale = locale.value())
    }

    pub async fn get_auctions_by_realm_id(&self, realm_id: u32, locale: Locale) -> Result<u32> {
        let uri = self.get_uri(Game::WoW(WoW::Auctions(realm_id)), locale);

        let mut client_builder = reqwest::Client::builder().use_rustls_tls();

        if self.ca_cert.len() > 0 {
            client_builder =
                client_builder.add_root_certificate(Certificate::from_pem(&self.ca_cert).unwrap());
        }

        let client = client_builder.build().unwrap();

        let request = reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse(uri.as_str()).map_err(|_err| Error::GenericError)?,
        );

        let tmp = reqwest::RequestBuilder::from_parts(client, request)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|err| Error::HttpError(err))?
            .text()
            .await
            .map_err(|err| Error::HttpError(err))?
            .split("\r\n")
            .count() as u32;

        Ok(tmp)
    }
}
