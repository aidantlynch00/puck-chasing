use reqwest::{Client, Error};
use crate::types::string::PlayerId;
use crate::types::slapshot::RecentHistory;

pub async fn get_recent_history(
    client: &Client,
    id: &PlayerId
) -> Result<RecentHistory, Error> {
    let player_url = format!("https://slapshot.com/api/game/players/{}", id);

    client
        .get(player_url)
        .send()
        .await?
        .json::<RecentHistory>()
        .await
}
