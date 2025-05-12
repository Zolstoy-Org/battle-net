#![forbid(unsafe_code)]

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic error")]
    GenericError,
    #[error("HTTP error: {0}")]
    HttpError(reqwest::Error),
}

pub struct Connection {
    region: RegionUriPart,
    token: String,
}

pub enum RegionUriPart {
    EU,
    US,
    APAC,
    CN,
}

impl RegionUriPart {
    fn value(&self) -> &str {
        match *self {
            RegionUriPart::EU => "eu",
            RegionUriPart::US => "us",
            RegionUriPart::APAC => "apac",
            RegionUriPart::CN => "cn",
        }
    }
}

pub enum LocaleUriPart {
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

impl LocaleUriPart {
    fn value(&self) -> &str {
        match *self {
            LocaleUriPart::EnUs => "en_US",
            LocaleUriPart::EsMx => "es_MX",
            LocaleUriPart::PtBr => "pt_BR",
            LocaleUriPart::DeDe => "de_DE",
            LocaleUriPart::EnGb => "en_GB",
            LocaleUriPart::EsEs => "es_ES",
            LocaleUriPart::FrFr => "fr_FR",
            LocaleUriPart::ItIt => "it_IT",
            LocaleUriPart::RuRu => "ru_RU",
            LocaleUriPart::KoKr => "ko_KR",
            LocaleUriPart::ZhTw => "zn_TW",
            LocaleUriPart::ZhCn => "zh_CN",
        }
    }
}

enum WoWRoute {
    Auctions(u32),
}

enum GameRoute {
    WoW(WoWRoute),
}

impl GameRoute {
    fn value(&self) -> String {
        match self {
            GameRoute::WoW(route) => {
                format!(
                    "wow/{route}",
                    route = match *route {
                        WoWRoute::Auctions(realm_id) => {
                            format!("connected-realm/{realm_id}/auctions")
                        }
                    }
                )
            }
        }
    }
}

impl Connection {
    pub fn new(region: RegionUriPart, token: &str) -> Connection {
        Connection {
            token: token.to_string(),
            region,
        }
    }

    fn get_uri(&self, game_route: GameRoute, locale: LocaleUriPart) -> String {
        format!("https://{region}.api.blizzard.com/data/{game_route_part}/connected-realm/106/auctions?namespace=dynamic-{region}&locale={locale}",
            region = self.region.value(), game_route_part = game_route.value(), locale = locale.value())
    }

    pub async fn get_auctions_by_realm_id(
        &self,
        realm_id: u32,
        locale: LocaleUriPart,
    ) -> Result<u32> {
        Ok(
            reqwest::get(self.get_uri(GameRoute::WoW(WoWRoute::Auctions(realm_id)), locale))
                .await
                .map_err(|err| Error::HttpError(err))?
                .text()
                .await
                .map_err(|err| Error::HttpError(err))?
                .split("\r\n")
                .count() as u32,
        )
    }
}
