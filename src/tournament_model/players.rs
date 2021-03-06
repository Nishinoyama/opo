use std::cmp::Ordering;
use super::matching::Matching;

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

    pub fn matched_round_number(&self) -> usize {
        self.matching_list().into_iter()
            .filter(|matching| !matching.is_dropped())
            .count()
    }

    pub fn calculate_points(&mut self) {
        self.points =
            self.matching_list().into_iter()
            .map(|matching| matching.matching_points())
            .sum()
    }

    fn any_percentage(opponents_wp: Vec<f64>) -> f64 {
        let count = opponents_wp.len();
        let sum = opponents_wp.into_iter().sum::<f64>();
        return if count > 0 {
            sum / count as f64
        } else {
            0.0
        }
    }

    fn matching_list_to_filtered_mapped_percentage_list<PercentageFn>(matching_list: &Vec<Matching>, mut percentage_fn: PercentageFn ) -> Vec<f64>
    where PercentageFn: FnMut(&Matching) -> f64 {
        matching_list.into_iter()
            .filter(|matching| matching.is_valid())
            .map(|matching| percentage_fn(matching))
            .collect()
    }

    pub fn calculate_match_win_percentages(&mut self) {
        self.match_win_percentage =
            self.points as f64 / self.matched_round_number() as f64 / 3.0;
    }

    pub fn calculate_opponent_match_win_percentages(&mut self, players_mwp: &Vec<f64>) {
        let omwp_list: Vec<f64> = Self::matching_list_to_filtered_mapped_percentage_list(self.matching_list(), |matching|{
            f64::max(1.0/3.0, *players_mwp.get(matching.opponent_id()).unwrap())
        });
        self.opponent_match_win_percentage = Self::any_percentage(omwp_list);
    }

    pub fn calculate_game_win_percentages(&mut self) {
        let gwp_list: Vec<f64> = Self::matching_list_to_filtered_mapped_percentage_list(self.matching_list(), |matching|{
            matching.game_win_percentage()
        });
        self.game_win_percentage = Self::any_percentage(gwp_list);
    }

    pub fn calculate_opponent_game_win_percentages(&mut self, players_gwp: &Vec<f64>) {
        let ogwp_list: Vec<f64> = Self::matching_list_to_filtered_mapped_percentage_list(self.matching_list(), |matching|{
            *players_gwp.get(matching.opponent_id()).unwrap()
        });
        self.opponent_game_win_percentage = Self::any_percentage(ogwp_list)
    }

    pub fn had_matched_id(&self, search_id: Option<usize>) -> bool {
        match search_id {
            Some(id) =>
                self.matching_list().into_iter()
                    .find(|matching| matching.opponent_id() == id).is_some(),
            None =>
                self.matching_list().into_iter()
                    .find(|matching| matching.is_no_opponent()).is_some(),
        }
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
    use crate::assert_ap;
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
    assert_ap!(p.opponent_match_win_percentage, 0.591, 0.001); // 0.591
    assert_ap!(p.game_win_percentage, 0.5833, 0.0001); // 0.5833
}

#[test]
fn test_special_points() {
    use crate::assert_ap;
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
    assert_ap!(p.opponent_match_win_percentage, 0.333, 0.001); // 0.3333
    assert_ap!(p.game_win_percentage, 0.777, 0.001); // 0.777
    assert_ap!(p.opponent_game_win_percentage, 0.067, 0.001); // 0.777
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
