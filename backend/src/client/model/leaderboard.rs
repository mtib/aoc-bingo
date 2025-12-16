use crate::model::leaderboard::{AocMemberId, Day, Part};

///
/// ```json
/// {"event":"2025","day1_ts":1764565200,"members":{"2465123":{"id":2465123,"local_score":47,"completion_day_level":{"2":{"2":{"get_star_ts":1764778741,"star_index":14},"1":{"get_star_ts":1764778268,"star_index":13}},"3":{"1":{"star_index":15,"get_star_ts":1764779972},"2":{"star_index":16,"get_star_ts":1764785912}},"1":{"1":{"star_index":11,"get_star_ts":1764776283},"2":{"get_star_ts":1764776400,"star_index":12}},"4":{"1":{"star_index":18,"get_star_ts":1764837679},"2":{"get_star_ts":1764839506,"star_index":19}},"6":{"2":{"star_index":31,"get_star_ts":1765025509},"1":{"get_star_ts":1765022936,"star_index":30}},"5":{"2":{"get_star_ts":1764923896,"star_index":25},"1":{"star_index":24,"get_star_ts":1764923510}}},"name":"Markus Becker","last_star_ts":1765025509,"stars":12},"724629":{"id":724629,"local_score":53,"completion_day_level":{"5":{"2":{"star_index":27,"get_star_ts":1764928685},"1":{"star_index":26,"get_star_ts":1764926246}},"3":{"2":{"get_star_ts":1764742450,"star_index":8},"1":{"star_index":7,"get_star_ts":1764738321}},"2":{"2":{"get_star_ts":1764697576,"star_index":6},"1":{"star_index":5,"get_star_ts":1764696233}},"1":{"2":{"star_index":4,"get_star_ts":1764694372},"1":{"star_index":2,"get_star_ts":1764692728}},"4":{"2":{"get_star_ts":1764842773,"star_index":21},"1":{"get_star_ts":1764842313,"star_index":20}},"6":{"2":{"get_star_ts":1765000341,"star_index":29},"1":{"get_star_ts":1764998385,"star_index":28}}},"last_star_ts":1765000341,"name":"LFalch","stars":12},"1546568":{"stars":0,"local_score":0,"completion_day_level":{},"name":"Varvara","last_star_ts":0,"id":1546568},"2724821":{"id":2724821,"name":"throwpedro","last_star_ts":1764870465,"completion_day_level":{"2":{"2":{"get_star_ts":1764772361,"star_index":9},"1":{"star_index":3,"get_star_ts":1764693292}},"3":{"1":{"star_index":10,"get_star_ts":1764773628},"2":{"star_index":17,"get_star_ts":1764806122}},"1":{"1":{"star_index":0,"get_star_ts":1764585193},"2":{"star_index":1,"get_star_ts":1764597431}},"4":{"1":{"star_index":22,"get_star_ts":1764858501},"2":{"get_star_ts":1764870465,"star_index":23}}},"local_score":32,"stars":8},"2320819":{"completion_day_level":{},"local_score":0,"name":"gaetjen","last_star_ts":0,"stars":0,"id":2320819}},"owner_id":2465123,"num_days":12}
/// ```
#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct LeaderboardResponse {
    pub event: String,
    pub day1_ts: u64,
    pub members: std::collections::HashMap<AocMemberId, MemberResponse>,
    pub owner_id: AocMemberId,
    pub num_days: u32,
}

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct MemberResponse {
    pub id: AocMemberId,
    pub local_score: u32,
    pub completion_day_level:
        std::collections::HashMap<Day, std::collections::HashMap<Part, StarInfoResponse>>,
    pub name: String,
    pub last_star_ts: u64,
    pub stars: u32,
}

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct StarInfoResponse {
    pub get_star_ts: u64,
    pub star_index: Option<u32>,
}
