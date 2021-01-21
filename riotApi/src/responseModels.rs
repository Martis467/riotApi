use serde_derive::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SummonerResponse {
    /// Encrypted summoner ID. Max length 63 characters.
    pub id: String,
    /// Encrypted account ID. Max length 56 characters.
    pub account_id: String,
    /// Encrypted PUUID. Exact length of 78 characters.
    pub puuid: String,
    /// Summoner name.
    pub name: String,
    /// ID of the summoner icon associated with the summoner
    pub porifle_icon_id: i16,
    /// Date summoner was last modified specified as epoch milliseconds. 
    /// The following events will update this timestamp: summoner name change, summoner level change, or profile icon change.
    pub revision_date: i64,
    /// Summoner level associated with the summoner.
    pub summoner_level: i16,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMasteryResponse {
    /// Players champion mastery list
    pub champion_masteries: Vec<ChampionMastery>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionMastery {
    /// Champion ID for this entry.
    pub champion_id: i16,
    /// Champion level for specified player and champion combination.
    pub champion_level: i16,
    /// Total number of champion points for this player and champion combination - they are used to determine championLevel.
    pub champion_points: i32,
    /// Last time this champion was played by this player - in Unix milliseconds time format.
    pub last_play_time: i64,
    /// Number of points earned since current level has been achieved.
    pub champion_points_since_last_level: i32,
    /// Number of points needed to achieve next level. Zero if player reached maximum champion level for this champion.
    pub champion_points_until_next_level: i16,
    /// Is chest granted for this champion or not in current season.
    pub chest_granted: bool,
    /// The token earned for this champion at the current championLevel. When the championLevel is advanced the tokensEarned resets to 0.
    pub tokens_earned: i16,
    /// Summoner ID for this entry. (Encrypted)
    pub summoner_id: String,
}

pub struct RankResponse {
    pub rank_response: Vec<PlayerRank>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRank {
    /// Some kind of id to identify your unique league
    pub league_id: String,
    /// Que type, like solo or flex
    pub queue_type: QueueType,
    /// Ranked tier like silver, gold
    pub tier: Tier,
    /// ranked division like I, II
    pub rank: Rank,
    /// Player's encrypted summonerId.
    pub summoner_id: String,
    /// Current division league points
    pub league_points: i16,
    /// Wins this season
    pub wins: i16,
    /// Losses this season
    pub losses: i16,
    /// Is this a veteran? Wtf??
    pub veteran: bool,
    /// Is the player receiving penalties for inactivity?
    pub inactive: bool,
    /// Is the player new to ranked?
    pub fresh_blood: bool,
    /// Is the player on a winning streak?
    pub hot_streak: bool,
}

#[derive(Deserialize)]
#[serde(tag = "queueType")]
pub enum QueueType {
    Qtype(String)
}

#[derive(Deserialize)]
#[serde(tag = "tier")]
pub enum Tier {
    IRON,
    BRONZE,
    SILVER,
    GOLD,
    PLATINUM,
    DIAMOND,
    MASTER,
    GRAND_MASTER,
    CHALLANGER
}

#[derive(Deserialize)]
#[serde(tag = "rank")]
pub enum Rank {
    I,
    II,
    III,
    IV,
}

