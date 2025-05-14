use reqwest::Certificate;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::CertificateDer;
use rustls::RootCertStore;
use webpki_roots::TLS_SERVER_ROOTS;

use crate::error::Error;
use crate::result::Result;

pub struct Instance {
    token: String,
    region: Region,
    address: String,
    root_cert_store: RootCertStore,
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
            root_cert_store: RootCertStore {
                roots: TLS_SERVER_ROOTS.into(),
            },
        }
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    pub fn add_ca_cert<'a>(&mut self, cert_slice: &'a [u8]) {
        self.root_cert_store.add_parsable_certificates(
            CertificateDer::pem_slice_iter(cert_slice).map(|result| result.unwrap()),
        );
    }

    fn get_uri(&self, game_route: Game, locale: Locale) -> String {
        format!("https://{address}/data/{game_route_part}/connected-realm/106/auctions?namespace=dynamic-{region}&locale={locale}",
            address = self.address, region = self.region.value(), game_route_part = game_route.value(), locale = locale.value())
    }

    pub async fn get_auctions_by_realm_id(&self, realm_id: u32, locale: Locale) -> Result<u32> {
        let uri = self.get_uri(Game::WoW(WoW::Auctions(realm_id)), locale);

        let client = reqwest::Client::builder()
            .use_rustls_tls()
            // .add_root_certificate(
            //     Certificate::from_pem(&self.root_cert_store.roots.first().unwrap().subject.concat()                    .unwrap(),
            // )
            .build()
            .unwrap();

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
