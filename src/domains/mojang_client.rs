use log::info;
use reqwest::{Client, Url};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::Error;

const MOJANG_API_URL: &str = "https://api.mojang.com/";

pub struct MojangClient {
    client: Client,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum UsernameToUUIDResponse {
    Ok(UsernameToUUID),
    None,
    Err {
        error: String,
        error_message: String,
    },
}

#[derive(Debug, Deserialize)]
struct UsernameToUUID {
    #[allow(dead_code)]
    name: String,
    id: Uuid,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum UUIDToNameHistoryResponse {
    Ok(Vec<UUIDToNameHistory>),
    None,
    Err {
        error: String,
        error_message: String,
    },
}

#[derive(Debug, Deserialize)]
struct UUIDToNameHistory {
    name: String,
    #[allow(dead_code)]
    changed_to_at: Option<usize>,
}

impl MojangClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn username_to_uuid(&mut self, username: &str) -> Result<Option<Uuid>, Error> {
        let url =
            Url::parse(MOJANG_API_URL)?.join(&format!("/users/profiles/minecraft/{username}"))?;
        info!("{url}");
        let res = self.client.get(url).send().await?;
        info!("{:?}", res);

        let res = if res.status() == 204 {
            UsernameToUUIDResponse::None
        } else {
            res.json::<UsernameToUUIDResponse>().await?
        };
        match res {
            UsernameToUUIDResponse::Ok(res) => Ok(Some(res.id)),
            UsernameToUUIDResponse::None => Ok(None),
            UsernameToUUIDResponse::Err {
                error,
                error_message,
            } => {
                log::error!("you are lucky {error}, {error_message}");
                Ok(None)
            }
        }
    }

    async fn uuid_to_name_history(
        &mut self,
        uuid: &Uuid,
    ) -> Result<Option<Vec<UUIDToNameHistory>>, Error> {
        let url = Url::parse(MOJANG_API_URL)?.join(&format!("/user/profiles/{uuid}/names"))?;
        info!("{url}");
        let res = self.client.get(url).send().await?;
        info!("{:?}", res);

        let res = if res.status() == 204 {
            UUIDToNameHistoryResponse::None
        } else {
            res.json::<UUIDToNameHistoryResponse>().await?
        };
        match res {
            UUIDToNameHistoryResponse::Ok(res) => Ok(Some(res)),
            UUIDToNameHistoryResponse::None => Ok(None),
            UUIDToNameHistoryResponse::Err {
                error,
                error_message,
            } => {
                log::error!("you are lucky {error}, {error_message}");
                Ok(None)
            }
        }
    }

    pub async fn uuid_to_name(&mut self, uuid: &Uuid) -> Result<Option<String>, Error> {
        if let Some(hists) = self.uuid_to_name_history(uuid).await? {
            if let Some(hist) = hists.iter().next() {
                return Ok(Some(hist.name.to_string()));
            }
        }

        Ok(None)
    }
}
