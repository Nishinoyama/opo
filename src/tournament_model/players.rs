use std::cmp::Ordering;
use crate::tournament_model::matching::Matching;

#[derive(Default,Debug)]
/// # Player is player
///
/// ## Ordering
///
/// 1. dropping relation ( them who dropping is `true` is Less )
/// 2. Point Greater
/// 3. OpMatchWin Greater
/// 4. GameWin Greater
/// 5. OpGameWin Greater
/// 6. ID Less
pub struct Player{
    /// the player's id
    id: usize,
    /// the player's name
    name: String,
    /// did Player dropped on the tournament?
    dropped: bool,
    /// points the player gained in matches
    points: i32,
    match_win_percentage: f64,
    opponent_match_win_percentage: f64,
    game_win_percentage: f64,
    opponent_game_win_percentage: f64,
    matching_list: Vec<Matching>,
}

impl Player {
    pub fn new(id: usize, name: String) -> Self {
        let mut player: Player = Default::default();
        player.name = name;
        player.id = id;
        player
    }
    pub fn dummy(id: usize) -> Self {
        let mut player = Player::new(id, "!!DUMMY!!".to_string());
        player.dropped = true;
        player
    }
    pub fn points_mut(&mut self) -> &mut i32 {
        &mut self.points
    }
    pub fn match_win_percentage_mut(&mut self) -> &mut f64 {
        &mut self.match_win_percentage
    }
    pub fn opponent_match_win_percentage_mut(&mut self) -> &mut f64 {
        &mut self.opponent_match_win_percentage
    }
    pub fn game_win_percentage_mut(&mut self) -> &mut f64 {
        &mut self.game_win_percentage
    }
    pub fn opponent_game_win_percentage_mut(&mut self) -> &mut f64 {
        &mut self.opponent_game_win_percentage
    }
    pub fn matching_list_mut(&mut self) -> &mut Vec<Matching> {
        &mut self.matching_list
    }

    pub fn match_win_percentage(&self) -> f64 {
        self.match_win_percentage
    }
    pub fn opponent_match_win_percentage(&self) -> f64 {
        self.opponent_match_win_percentage
    }
    pub fn game_win_percentage(&self) -> f64 {
        self.game_win_percentage
    }
    pub fn opponent_game_win_percentage(&self) -> f64 {
        self.opponent_game_win_percentage
    }
    pub fn matching_list(&self) -> &Vec<Matching> {
        &self.matching_list
    }

    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn points(&self) -> i32 {
        self.points
    }

    pub fn add_matching(&mut self, matching: Matching) {
        self.matching_list.push(matching);
    }

    pub fn is_dropped(&self) -> bool {
        self.dropped
    }

    pub fn initialize_points(&mut self) {
        self.points = 0;
        self.game_win_percentage = 0.0;
        self.match_win_percentage = 0.0;
        self.opponent_game_win_percentage = 0.0;
        self.opponent_match_win_percentage = 0.0;
    }

    pub fn matched_round_number(&self) -> i32 {
        let mut count = 0;
        for matching in self.matching_list() {
            if !matching.is_dropped() {
                count += 1;
            }
        }
        count
    }

    pub fn calculate_points(&mut self) {
        self.points = 0;
        for matching in &self.matching_list {
            self.points += matching.matching_points();
        }
    }

    pub fn calculate_match_win_percentages(&mut self) {
        self.match_win_percentage = self.points as f64/ self.matched_round_number() as f64 / 3.0;
    }

    pub fn calculate_opponent_match_win_percentages(&mut self, players_mwp: &Vec<f64>) {
        let mut omwp = 0.0;
        let mut count = 0;
        for matching in self.matching_list() {
            if matching.is_valid() {
                omwp += f64::max( 1.0/3.0, players_mwp[matching.opponent_id()] );
                count += 1;
            }
        }
        self.opponent_match_win_percentage = if count > 0 {
            omwp / count as f64
        } else {
            0.0
        }
    }

    pub fn calculate_game_win_percentages(&mut self) {
        let mut gwp = 0.0;
        let mut count = 0;
        for matching in self.matching_list() {
            if matching.is_valid() {
                gwp += matching.game_win_percentage();
                count += 1
            }
        }
        self.game_win_percentage = if count > 0 {
            gwp / count as f64
        } else {
            0.0
        }
    }

    pub fn calculate_opponent_game_win_percentages(&mut self, players_gwp: &Vec<f64>) {
        let mut ogwp = 0.0;
        let mut count = 0;
        for matching in self.matching_list() {
            if matching.is_valid() {
                ogwp += players_gwp[matching.opponent_id()];
                count += 1;
            }
        }
        self.opponent_game_win_percentage = if count > 0 {
            ogwp / count as f64
        } else {
            0.0
        }
    }

    pub fn had_matched_id(&self, search_id: Option<usize>) -> bool {
        let mut existed = false;
        match search_id {
            Some(id) =>
                for matching in self.matching_list() {
                    existed = existed || matching.opponent_id() == id;
                },
            None =>
                for matching in self.matching_list() {
                    existed = existed || matching.is_no_opponent();
                }
        }
        existed
    }

}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.dropped ^ other.dropped {
            if self.dropped {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            if !self.points.eq(&other.points) {
                self.points.cmp(&other.points)
            } else {
                if !self.opponent_match_win_percentage.eq(&other.opponent_match_win_percentage) {
                    self.opponent_match_win_percentage.partial_cmp(&other.opponent_match_win_percentage).unwrap()
                } else {
                    if !self.game_win_percentage.eq(&other.game_win_percentage) {
                        self.game_win_percentage.partial_cmp(&other.game_win_percentage).unwrap()
                    } else {
                        if !self.opponent_game_win_percentage.eq(&other.opponent_game_win_percentage) {
                            self.opponent_game_win_percentage.partial_cmp(&other.opponent_game_win_percentage).unwrap()
                        } else {
                            other.id.cmp(&self.id)
                        }
                    }
                }
            }
        }
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        !(self.dropped ^ other.dropped) &&
        self.points == other.points &&
        self.opponent_match_win_percentage == other.opponent_match_win_percentage &&
        self.game_win_percentage == other.game_win_percentage &&
        self.opponent_game_win_percentage == self.opponent_game_win_percentage &&
        self.id == other.id
    }
}

impl Eq for Player {}

#[test]
fn test_player_construct() {
    let p = Player::new(0, "あ😁し😁は😁ら".to_string());
    assert_eq!(*p.name(), "あ😁し😁は😁ら".to_string());
    assert_eq!(p.id, 0);
    assert!(!p.dropped);
}

#[test]
fn test_add_matching() {
    // referenced: https://kirisamemagic.diarynote.jp/201401060210226433/
    let mut p = Player::new(0, "あ😁し😁は😁ら".to_string());
    p.add_matching(Matching::new(0, 0, 1, 2, 0, 0, false, false));
    p.add_matching(Matching::new(0, 0, 2, 1, 1, 0, false, false));
    p.add_matching(Matching::new(0, 0, 3, 2, 0, 1, false, false));
    p.add_matching(Matching::new(0, 0, 4, 0, 0, 0, false, true));
    p.add_matching(Matching::new(0, 0, 5, 0, 0, 2, false, false));
    assert_eq!(p.matching_list[0].draw_count(), 0);
    assert_eq!(p.matching_list[1].win_count(), 1);
    assert!(!p.matching_list[3].is_valid());
}

#[test]
fn test_points_calculation() {
    // referenced: https://kirisamemagic.diarynote.jp/201401060210226433/
    let mut p = Player::new(0, "あ😁し😁は😁ら".to_string());
    p.add_matching(Matching::new(0, 0, 1, 2, 0, 0, false, false));
    p.add_matching(Matching::new(0, 0, 2, 1, 1, 0, false, false));
    p.add_matching(Matching::new(0, 0, 3, 2, 0, 1, false, false));
    p.add_matching(Matching::new(0, 0, 4, 0, 0, 0, false, true));
    p.add_matching(Matching::new(0, 0, 5, 0, 0, 2, false, false));
    p.calculate_points();
    p.calculate_opponent_match_win_percentages(&vec![0.800,0.067,0.500,0.667,0.000,0.867]);
    p.calculate_game_win_percentages();
    assert_eq!(p.points, 12);
    assert!(p.opponent_match_win_percentage >= 0.590); // 0.591
    assert!(p.opponent_match_win_percentage <= 0.592);
    assert!(p.game_win_percentage >= 0.583); // 0.5833
    assert!(p.game_win_percentage <= 0.584);
}

#[test]
fn test_special_points() {
    let mut p = Player::new(0, "あ😁し😁は😁ら".to_string());
    p.add_matching(Matching::new(0, 0, 1, 2, 1, 0, false, false));
    p.add_matching(Matching::new(0, 0, 2, 1, 1, 0, true, false));
    p.add_matching(Matching::new(0, 0, 3, 2, 0, 1, false, true));
    p.add_matching(Matching::no_opponent_new(0, 0));
    p.add_matching(Matching::dropped_new(0, 0));
    p.calculate_points();
    p.calculate_opponent_match_win_percentages(&vec![0.800,0.067,0.500,0.667,0.000,0.867]);
    p.calculate_game_win_percentages();
    p.calculate_opponent_game_win_percentages(&vec![0.800,0.067,0.500,0.667,0.000,0.867]);
    assert_eq!(p.points, 9);
    assert!(p.opponent_match_win_percentage >= 0.332); // 0.3333
    assert!(p.opponent_match_win_percentage <= 0.334);
    assert!(p.game_win_percentage >= 0.776); // 0.777
    assert!(p.game_win_percentage <= 0.778);
    assert!(p.opponent_game_win_percentage >= 0.066); // 0.067
    assert!(p.opponent_game_win_percentage <= 0.068);
}

#[test]
fn test_players_ord() {
    let mut ps = Vec::new();
    let mut p1 = Player::new(0, "あ😁し😁は😁ら".to_string());
    p1.points = 1;
    let mut p2 = Player::new(1, "あ😁し😁は😁ら".to_string());
    p2.points = 2;
    let mut p3 = Player::new(2, "あ😁し😁は😁ら".to_string());
    p3.points = 0;
    let mut p4 = Player::new(3, "あ😁し😁は😁ら".to_string());
    p4.points = 1;
    assert_eq!(p1.cmp(&p2), Ordering::Less);
    assert_eq!(p1.cmp(&p3), Ordering::Greater);
    assert_eq!(p1.cmp(&p4), Ordering::Greater); // ID is less than p4, so Ordering is Greater
    ps.push(p1);
    ps.push(p2);
    ps.push(p3);
    ps.push(p4);
    ps.sort();
    assert_eq!(ps[0].id, 2);
    assert_eq!(ps[3].id, 1);
}
