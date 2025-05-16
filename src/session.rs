use crate::error::Error;
use crate::result::Result;
use reqwest::Certificate;
use serde::{Deserialize, Serialize};
use std::vec;

pub struct Session {
    token: String,
    region: Region,
    address: String,
    ca_cert: Vec<u8>,
    port: u16,
}

#[derive(Debug, Clone)]
pub enum Region {
    Eu,
    Us,
    Apac,
    Cn,
}

impl Region {
    pub fn auth_domain(&self) -> &str {
        match *self {
            Region::Eu => "https://oauth.battle.net/token",
            Region::Us => "	https://oauth.battle.net/token",
            Region::Apac => "https://oauth.battle.net/token",
            Region::Cn => "https://oauth.battlenet.com.cn/token",
        }
    }

    pub fn api_subdomain(&self) -> &str {
        match *self {
            Region::Eu => "eu",
            Region::Us => "us",
            Region::Apac => "apac",
            Region::Cn => "cn",
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
    fn param_value(&self) -> &str {
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
    fn route(&self) -> String {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthInfo {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

pub struct Authenticator {
    client_id: String,
    client_secret: String,
    region: Region,
    auth_domain: String,
    api_domain: String,
    ca_cert: Vec<u8>,
    https: bool,
    port: u16,
}

impl Authenticator {
    pub fn new() -> Self {
        Authenticator {
            client_id: String::new(),
            client_secret: String::new(),
            region: Region::Us,
            auth_domain: String::new(),
            api_domain: String::new(),
            ca_cert: vec![],
            https: true,
            port: 443,
        }
    }

    pub fn client_id(mut self, client_id: String) -> Self {
        self.client_id = client_id;
        self
    }

    pub fn client_secret(mut self, client_secret: String) -> Self {
        self.client_secret = client_secret;
        self
    }

    pub fn region(mut self, region: Region) -> Self {
        self.region = region.clone();
        self
    }

    pub fn auth_domain(mut self, auth_domain: &str) -> Self {
        self.auth_domain = auth_domain.to_string();
        self
    }

    pub fn api_domain(mut self, api_domain: &str) -> Self {
        self.api_domain = api_domain.to_string();
        self
    }

    pub fn ca_cert(mut self, ca_cert: &[u8]) -> Self {
        self.ca_cert = ca_cert.to_vec();
        self
    }

    pub fn https(mut self, https: bool) -> Self {
        self.https = https;
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub async fn authenticate(self) -> Result<Session> {
        let token = reqwest::Client::new()
            .post(self.auth_domain)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await
            .map_err(|err| Error::HttpError(err))?
            .json::<AuthInfo>()
            .await
            .map_err(|err| Error::HttpError(err))?
            .access_token;

        Ok(Session {
            token,
            region: self.region,
            address: self.api_domain,
            ca_cert: self.ca_cert,
            port: self.port,
        })
    }
}

impl Session {
    fn api_url(&self, game_route: Game, locale: Locale) -> String {
        format!("https://{address}:{port}/data/{game_route_part}/connected-realm/106/auctions?namespace=dynamic-{region}&locale={locale}",
            address = self.address, port = self.port, region = self.region.subdomain(), game_route_part = game_route.route(), locale = locale.param_value())
    }

    pub async fn get_auctions_by_realm_id(&self, realm_id: u32, locale: Locale) -> Result<u32> {
        let uri = self.api_url(Game::WoW(WoW::Auctions(realm_id)), locale);

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
