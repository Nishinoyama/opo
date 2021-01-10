use crate::tournament_model::matching::Matching;
use crate::tournament_model::players::Player;

#[derive(Default)]
pub struct Tournament {
    player_number: i32,
    // available_player_number: i32,
    // matching_number: i32,
    players: Vec<Player>,
    matching_list: Vec<Matching>,
}

impl Tournament {
    /// make up json file tournament data. name is `tournament_name`.json.
    pub fn make_json() {
        unimplemented!();
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn calculate_points(&mut self) {
        for player in &mut self.players {
            player.calculate_points();
        }
    }

    fn calculate_match_win_percentages(&mut self) {
        for player in &mut self.players {
            player.calculate_match_win_percentages();
        }
    }

    fn calculate_opponent_match_win_percentages(&mut self) {
        let mut players_mwp = Vec::new();
        for player in &self.players {
            players_mwp.push(player.match_win_percentage());
        }
        for player in &mut self.players {
            player.calculate_opponent_match_win_percentages(&players_mwp);
        }
    }

    fn calculate_game_win_percentages(&mut self) {
        for player in &mut self.players {
            player.calculate_game_win_percentages();
        }
    }

    fn calculate_opponent_game_win_percentages(&mut self) {
        let mut players_gwp = Vec::new();
        for player in &self.players {
            players_gwp.push(player.match_win_percentage());
        }
        for player in &mut self.players {
            player.calculate_opponent_game_win_percentages(&players_gwp);
        }
    }

    fn aggregate_points(&mut self) {
        self.calculate_points();
        self.calculate_match_win_percentages();
        self.calculate_opponent_match_win_percentages();
        self.calculate_game_win_percentages();
        self.calculate_opponent_game_win_percentages();
    }

    pub fn aggregate_matches(&mut self, matches: Vec<Matching>) {
        // そのラウンドでマッチ結果の集計、マッチしてない人の特定
        let mut matched = Vec::with_capacity(self.player_number as usize);
        for _ in 0..self.player_number {
            matched.push(false);
        }
        for matching in matches {
            self.matching_list.push(matching);
        }

        // マッチ結果に基づき計算を行う
        self.aggregate_points();
        // self.matching_list.calculate_player_points(&mut self.players);
        // matching_lists::aggregate(players, matches);

    }

}

#[test]
fn test_add_player() {
    let mut t: Tournament = Default::default();
    for i in 0..10000 {
        let p: Player = Player::new(i, format!("{}abcd", i));
        t.add_player(p);
    }
    for i in 0..10000 {
        assert_eq!(*t.players[i].name(), format!("{}abcd", i));
    }
}