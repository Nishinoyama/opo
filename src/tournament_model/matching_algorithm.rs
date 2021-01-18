use super::players::Player;

fn invert_left_shift( mut k: usize ) -> usize {
    let mut count = 0;
    while ( k & 1 ) != 1 {
        count += 1;
        k >>= 1;
    }
    count
}

pub fn matching_build(players: &mut Vec<&Player>) -> Result<Vec<Option<usize>>, String> {

    let mut matchable_players: Vec<&&Player> = players.iter().filter(|p| !p.is_dropped()).collect();
    matchable_players.sort();
    matchable_players.reverse();
    let dummy_player = &&Player::dummy(usize::max_value());
    if matchable_players.len() % 2 == 1 {
        matchable_players.push(dummy_player);
    }
    let matchable_number: usize = matchable_players.len();

    for ext in 6..26 {
        let mut dp = vec![vec![i32::max_value(); 1 << ext]; matchable_number+1];
        let mut rb = vec![vec![(-1,false); 1 << ext]; matchable_number+1];
        dp[0][0] = 0;
        for ni in 0..matchable_number+1 {
            for bi in 0..(1<<ext) {
                if dp[ni][bi] == i32::max_value() {
                    continue;
                }
                let nni = ni + 1;
                if bi & 1 == 1 {
                    if dp[ni][bi] < dp[nni][bi >> 1] {
                        dp[nni][bi >> 1] = dp[ni][bi];
                        rb[nni][bi >> 1] = (bi as i32, false);
                    }
                    continue;
                }
                if ni == matchable_number {
                    break;
                }
                for pi in 0..ext {
                    let ppi = ni + pi + 1;
                    if ppi >= matchable_number {
                        break;
                    }
                    if ( (bi >> 1) & (1 << pi) ) != 0 {
                        continue;
                    }
                    let player = matchable_players[ni];
                    let opponent = matchable_players[ppi];
                    let bbi = (bi >> 1) | (1 << pi);
                    let mut cost = player.points();
                    let opponent_id = if ppi == usize::max_value() {
                        cost -= opponent.points();
                        Some(player.id())
                    } else {
                        None
                    };
                    if !player.had_matched_id(opponent_id) {
                        if dp[ni][bi] + cost < dp[nni][bbi] {
                            dp[nni][bbi] = dp[ni][bi] + cost;
                            rb[nni][bbi] = (bi as i32, true);
                        }
                    }
                }
            }
        }

        // rollback
        let mut matching_list = vec![None; players.len()];
        let mut rbn = matchable_number;
        let mut rbb:usize = 0;
        let matching_success = loop {
            let tmp_rbb = rb[rbn][rbb].0;
            if tmp_rbb < 0 {
                break false;
            }
            if rb[rbn][rbb].1 {
                let transition = (rbb<<1) - tmp_rbb as usize;
                let lid = matchable_players[rbn - 1].id();
                let rid = matchable_players[rbn - 1 + invert_left_shift(transition)].id();
                if lid < matchable_number && rid < matchable_number {
                    matching_list[lid] = Some(rid);
                    matching_list[rid] = Some(lid);
                }
            }
            rbn -= 1;
            rbb = tmp_rbb as usize;
            if rbn == 0 && rbb == 0 {
                break true;
            }
        };

        if matching_success {
            return Ok(matching_list);
        }

    }

    Err("No satisfying matching!".to_string())

}
