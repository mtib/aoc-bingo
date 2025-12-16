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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialze_year_2018() {
        let json_data = r#"{"num_days":25,"day1_ts":1606798800,"owner_id":2465123,"members":{"2465123":{"local_score":0,"global_score":0,"name":"Markus Becker","id":2465123,"last_star_ts":1700409015,"completion_day_level":{"1":{"1":{"get_star_ts":1700408539,"star_index":null},"2":{"star_index":null,"get_star_ts":1700409015}}},"stars":2},"724629":{"global_score":0,"name":"LFalch","local_score":90,"stars":20,"last_star_ts":1608944425,"completion_day_level":{"4":{"1":{"star_index":4,"get_star_ts":1607540214},"2":{"star_index":5,"get_star_ts":1607542389}},"11":{"1":{"get_star_ts":1608944425,"star_index":17}},"2":{"2":{"get_star_ts":1607537451,"star_index":1},"1":{"get_star_ts":1607536619,"star_index":0}},"7":{"1":{"star_index":10,"get_star_ts":1607875847},"2":{"star_index":11,"get_star_ts":1607876493}},"3":{"1":{"get_star_ts":1607538546,"star_index":2},"2":{"star_index":3,"get_star_ts":1607539339}},"5":{"1":{"star_index":6,"get_star_ts":1607544423},"2":{"star_index":7,"get_star_ts":1607544819}},"6":{"2":{"star_index":9,"get_star_ts":1607873667},"1":{"get_star_ts":1607873246,"star_index":8}},"1":{"1":{"star_index":null,"get_star_ts":1606850589},"2":{"star_index":null,"get_star_ts":1606850739}},"9":{"2":{"get_star_ts":1607879957,"star_index":15},"1":{"star_index":14,"get_star_ts":1607879542}},"8":{"1":{"get_star_ts":1607877157,"star_index":12},"2":{"get_star_ts":1607877943,"star_index":13}},"10":{"1":{"get_star_ts":1607880859,"star_index":16}}},"id":724629},"1546568":{"local_score":0,"name":"Varvara","global_score":0,"stars":0,"id":1546568,"last_star_ts":0,"completion_day_level":{}},"2320819":{"stars":0,"id":2320819,"completion_day_level":{},"last_star_ts":0,"local_score":0,"name":"gaetjen","global_score":0},"2724821":{"local_score":0,"name":"throwpedro","global_score":0,"stars":0,"id":2724821,"completion_day_level":{},"last_star_ts":0}},"event":"2020"}"#;
        let leaderboard: LeaderboardResponse =
            serde_json::from_str(json_data).expect("Failed to deserialize JSON");
        assert_eq!(leaderboard.num_days, 25);
        assert_eq!(leaderboard.owner_id, 2465123);
        assert_eq!(leaderboard.members.len(), 5);
    }
}
