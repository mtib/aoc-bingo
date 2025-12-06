use crate::client::model::leaderboard::LeaderboardResponse;

pub struct AocClient {
    client: reqwest::Client,
}

impl AocClient {
    pub fn new() -> Self {
        AocClient {
            client: reqwest::Client::new(),
        }
    }

    /// Fetches leaderboard
    pub async fn fetch_leaderboard(
        &self,
        year: u32,
        board_id: u32,
        session_token: &str,
    ) -> Result<LeaderboardResponse, reqwest::Error> {
        let url = format!(
            "https://adventofcode.com/{}/leaderboard/private/view/{}.json",
            year, board_id
        );
        let response = self
            .client
            .get(&url)
            .header("Cookie", format!("session={}", session_token))
            .send()
            .await?;

        let body = response.json::<LeaderboardResponse>().await?;
        Ok(body)
    }
}
