use super::matching::Matching;
use super::players::Player;

#[derive(Default, Debug)]
pub struct Tournament {
    player_number: i32,
    // available_player_number: i32,
    // matching_number: i32,
    players: Vec<Player>,
    // matching_list: Vec<Matching>,
}

impl Tournament {
    /// make up json file tournament data. name is `tournament_name`.json.
    // pub fn make_json() {
    //     unimplemented!();
    // }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
        self.player_number += 1;
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
            players_gwp.push(player.game_win_percentage());
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
            if matching.is_reversible() {
                let opponent_id = matching.opponent_id();
                self.players[opponent_id].add_matching(Matching::rev(&matching));
                matched[opponent_id] = true;
            }
            let player_id = matching.player_id();
            if matched[player_id] {
                panic!("Duplicated Matching!: {:?}", matching);
            }
            self.players[player_id].add_matching(matching);
            matched[player_id] = true;
        }

        // マッチ結果に基づき計算を行う
        self.aggregate_points();

    }

    /// return `Vec<Option<usize>>` whose `Some(n)` n-th element number has n-th id player's opponent i
    /// if element is `None`, no-opponent or player is dropped (No matching)
    pub fn matching_build(&self) -> Result<Vec<Option<usize>>, String> {
        crate::tournament_model::matching_algorithm::matching_build(&self.players)
    }

    pub fn greedy_matching_build(&self) -> Result<Vec<Option<usize>>, String> {
        crate::tournament_model::matching_algorithm::matching_build_greed(&self.players)
    }

    pub fn player_number(&self) -> usize {
        self.players.len()
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
    assert_eq!(t.player_number, 10000);
}

#[test]
fn test_aggregate_matches() {
    use crate::assert_ap;
    let mut t: Tournament = Default::default();
    for i in 0..4 {
        let p: Player = Player::new(i, format!("{}abcd", i));
        t.add_player(p);
    }
    let m = vec![
        Matching::new(1, 0, 1, 3, 0, 1, false, false),
        Matching::new(1, 2, 3, 0, 0, 2, false, false),
    ];
    t.aggregate_matches(m);
    let m = vec![
        Matching::new(2, 0, 3, 3, 0, 1, false, false),
        Matching::dropped_new(2,1),
        Matching::no_opponent_new(2,2),
    ];
    t.aggregate_matches(m);
    assert_eq!(t.players[0].points(), 6);
    assert_eq!(t.players[1].points(), 0);
    assert_eq!(t.players[2].points(), 3);
    assert_eq!(t.players[3].points(), 3);
    assert_eq!(t.players[0].match_win_percentage(), 1.0);
    assert_eq!(t.players[1].match_win_percentage(), 0.0);
    assert_eq!(t.players[2].match_win_percentage(), 0.5);
    assert_eq!(t.players[3].match_win_percentage(), 0.5);
    assert_ap!(t.players[0].opponent_match_win_percentage(), 0.42, 0.01);
    assert_ap!(t.players[1].opponent_match_win_percentage(), 1.00, 1e-5);
    assert_ap!(t.players[2].opponent_match_win_percentage(), 0.50, 1e-5);
    assert_ap!(t.players[3].opponent_match_win_percentage(), 0.75, 1e-5);
    assert_ap!(t.players[0].game_win_percentage(), 0.75, 1e-5);
    assert_ap!(t.players[1].game_win_percentage(), 0.25, 1e-5);
    assert_ap!(t.players[2].game_win_percentage(), 0.00, 1e-5);
    assert_ap!(t.players[3].game_win_percentage(), 0.625,1e-5);
    assert_ap!(t.players[0].opponent_game_win_percentage(), 0.4375,1e-5);
    assert_ap!(t.players[1].opponent_game_win_percentage(), 0.75,  1e-5);
    assert_ap!(t.players[2].opponent_game_win_percentage(), 0.625, 1e-5);
    assert_ap!(t.players[3].opponent_game_win_percentage(), 0.375, 1e-5);
}

#[test]
fn test_matching_build() {
    let mut t: Tournament = Default::default();
    for i in 0..2000 {
        let p: Player = Player::new(i, format!("{}abcd", i));
        t.add_player(p);
    }

    for _ in 0..20 {
        let ol = t.matching_build().unwrap();
        let mut mt = vec![false; t.players.len()];
        let mut ml = Vec::new();

        // test matching lists are symmetric and no previous matching duplication
        for i in 0..t.players.len() {
            match *ol.get(i).unwrap() {
                Some(n) => {
                    assert_eq!(i, ol.get(n).unwrap().unwrap());
                    assert!(!t.players.get(i).unwrap().had_matched_id(Some(n)));
                },
                None => {
                    assert!(!t.players.get(i).unwrap().had_matched_id(None));
                },
            }
        }

        // make sample of results of matches
        for (i,o) in ol.into_iter().enumerate() {
            match o {
                Some(n) => {
                    if !mt[n] {
                        mt[i] = true;
                        mt[n] = true;
                        ml.push(Matching::new(1,i,n,1,0,0,false,false));
                    }
                },
                None => {
                    ml.push(Matching::no_opponent_new(1,i));
                }
            }
        }

        t.aggregate_matches(ml);
    }
}

#[test]
fn test_greedy_matching_build() {
    let mut t: Tournament = Default::default();
    for i in 0..2000 {
        let p: Player = Player::new(i, format!("{}abcd", i));
        t.add_player(p);
    }

    for _ in 0..20 {
        let ol = t.greedy_matching_build().unwrap();
        let mut mt = vec![false; t.players.len()];
        let mut ml = Vec::new();

        // test matching lists are symmetric and no previous matching duplication
        for i in 0..t.players.len() {
            match *ol.get(i).unwrap() {
                Some(n) => {
                    assert_eq!(i, ol.get(n).unwrap().unwrap());
                    assert!(!t.players.get(i).unwrap().had_matched_id(Some(n)));
                },
                None => {
                    assert!(!t.players.get(i).unwrap().had_matched_id(None));
                },
            }
        }

        // make sample of results of matches
        for (i,o) in ol.into_iter().enumerate() {
            match o {
                Some(n) => {
                    if !mt[n] {
                        mt[i] = true;
                        mt[n] = true;
                        ml.push(Matching::new(1,i,n,1,0,0,false,false));
                    }
                },
                None => {
                    ml.push(Matching::no_opponent_new(1,i));
                }
            }
        }

        t.aggregate_matches(ml);
    }
}
