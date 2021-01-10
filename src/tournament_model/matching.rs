
#[derive(Clone)]
/// model of matching result
pub struct Matching {
    /// uid of match
    uid: usize,
    /// what round number does this match begin
    round_number: i32,
    /// id of player
    player_id: usize,
    /// id of opponent
    opponent_id: usize,
    /// the count that the player won games
    win_count: i32,
    /// the count that the player draw games
    draw_count: i32,
    /// the count that the player lost games
    lose_count: i32,
    /// did the player withdraw?
    player_withdraw: bool,
    /// did the opponent withdrew?
    opponent_withdraw: bool,
    /// is there opponent? ( on matching processing, there is not a matched opponent )
    no_opponent: bool,
    /// player has dropped? ( dropped player matching is not count as round )
    dropped: bool,
}

impl Matching {
    pub fn new(uid: usize, round_number: i32, player_id: usize, opponent_id: usize, win_count: i32, draw_count: i32, lose_count: i32, player_withdraw: bool, opponent_withdraw: bool ) -> Self {
        if player_id == opponent_id {
            panic!("player and opponent have same id!")
        }
        Matching { uid, round_number, player_id, opponent_id, win_count, draw_count, lose_count, player_withdraw, opponent_withdraw, no_opponent: false, dropped: false }
    }
    /// give to no opponent player
    pub fn no_opponent_new(uid: usize, round_number: i32, player_id: usize ) -> Self {
        Matching { uid, round_number, player_id, opponent_id: 0, win_count: 0, draw_count: 0, lose_count: 0, player_withdraw: false, opponent_withdraw: false, no_opponent: true, dropped: false }
    }
    /// give to dropped player
    pub fn dropped_new(uid: usize, round_number: i32, player_id: usize ) -> Self {
        Matching { uid, round_number, player_id, opponent_id: 0, win_count: 0, draw_count: 0, lose_count: 0, player_withdraw: true, opponent_withdraw: false, no_opponent: false, dropped: true }
    }
    pub fn rev(m: &Matching) -> Self {
        Self::new(m.uid, m.round_number, m.opponent_id, m.player_id, m.lose_count, m.draw_count, m.win_count, m.opponent_withdraw, m.player_withdraw)
    }
    pub fn player_id(&self) -> usize {
        self.player_id
    }
    pub fn opponent_id(&self) -> usize {
        self.opponent_id
    }
    pub fn win_count(&self) -> i32 {
        self.win_count
    }
    pub fn lose_count(&self) -> i32 {
        self.lose_count
    }
    pub fn draw_count(&self) -> i32 {
        self.draw_count
    }
    /// did both player and opponent is withdraw or win is equal to lose
    pub fn is_draw(&self) -> bool {
        self.player_withdraw && self.opponent_withdraw || self.is_valid() && self.win_count == self.lose_count
    }
    /// did only opponent withdraw or is win greater than lose
    pub fn is_win(&self) -> bool {
        self.no_opponent || ( !self.is_draw() && !self.player_withdraw && ( self.opponent_withdraw || !self.player_withdraw && self.win_count > self.lose_count ))
    }
    /// did only player withdraw or is win less than lose
    pub fn is_lose(&self) -> bool {
        !self.is_win() && !self.is_draw()
    }
    /// if someone withdrew or no opponent, match is invalid
    pub fn is_valid(&self) -> bool {
        !self.player_withdraw && !self.opponent_withdraw && !self.no_opponent && !self.dropped
    }
    /// if no opponent, no matching
    pub fn is_no_opponent(&self) -> bool {
        self.no_opponent
    }
    /// if dropped, dont count as match
    pub fn is_dropped(&self) -> bool {
        self.dropped
    }
    pub fn matching_points(&self) -> i32 {
        if self.is_win() {
            3
        } else if self.is_draw() {
            1
        } else {
            0
        }
    }
    pub fn game_win_percentage(&self) -> f64 {
        ( self.win_count * 3 + self.draw_count ) as f64 / ( self.win_count + self.draw_count + self.lose_count ) as f64 / 3.0
    }
}

#[test]
fn test_is_avail() {
    let m = Matching::new(1, 0, 0, 1, 10, 0, 10, false, false);
    assert!( m.is_valid() );
    let m = Matching::new(1, 0, 0, 1, 10, 0, 10, true, false);
    assert!( !m.is_valid() );
    let m = Matching::new(1, 0, 0, 1, 10, 0, 10, true, true);
    assert!( !m.is_valid() );
    let m = Matching::new(1, 0, 0, 1, 10, 0, 10, false, true);
    assert!( !m.is_valid() );
}

#[test]
fn test_result() {
    let m = Matching::new(1, 0, 0, 1, 10, 0, 10, false, false);
    assert!(m.is_draw());
    assert!(!m.is_win());
    assert!(!m.is_lose());
    assert_eq!(m.matching_points(), 1);
    let m = Matching::new(1, 0, 0, 1, 10, 0, 5, false, false);
    assert!(!m.is_draw());
    assert!(m.is_win());
    assert!(!m.is_lose());
    assert_eq!(m.matching_points(), 3);
    let m = Matching::new(1, 0, 0, 1, 5, 0, 10, false, false);
    assert!(!m.is_draw());
    assert!(!m.is_win());
    assert!(m.is_lose());
    assert_eq!(m.matching_points(), 0);
    let m = Matching::new(1, 0, 0, 1, 10, 0, 10, true, false);
    assert!(!m.is_draw());
    assert!(!m.is_win());
    assert!(m.is_lose());
    assert_eq!(m.matching_points(), 0);
    let m = Matching::new(1, 0, 0, 1, 10, 0, 10, false, true);
    assert!(!m.is_draw());
    assert!(m.is_win());
    assert!(!m.is_lose());
    assert_eq!(m.matching_points(), 3);
    let m = Matching::new(1, 0, 0, 1, 10, 0, 10, true, true);
    assert!(m.is_draw());
    assert!(!m.is_win());
    assert!(!m.is_lose());
    assert_eq!(m.matching_points(), 1);
    let m = Matching::no_opponent_new(1,0,0);
    assert!(!m.is_draw());
    assert!(m.is_win());
    assert!(!m.is_lose());
    assert_eq!(m.matching_points(), 3);
    let m = Matching::dropped_new(1,0,0);
    assert!(!m.is_draw());
    assert!(!m.is_win());
    assert!(m.is_lose());
    assert_eq!(m.matching_points(), 0);
}