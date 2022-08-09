use rand::Rng;
use std::fmt;
use std::mem::swap;
use std::time::Instant;

fn main() {
    let mut cards: Vec<u8> = Vec::with_capacity(64);
    for _c in 0..4 {
        cards.push(4); // four Aces
        cards.push(3); // four Kings
        cards.push(2); // four Queens
        cards.push(1); // four Jacks
    }
    for _c in 0..36 {
        cards.push(0); // 36 cards that don't matter
    }
    {
        check_all();
        let (hand_one, hand_two) =
            read_hands("J----J--KQ--K-----QQ----K-", "---Q-A----AA-----A-JK---J-");
        record_26s_top(hand_one, hand_two);
    }
    play_many(cards);
}

fn play_many(cards: Vec<u8>) {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut counter = 0;
    let mut highscore = 0;
    let mut best_game = deal(cards.clone(), false);
    let mut best_game_unplay = deal(cards.clone(), false);
    let start = Instant::now();
    let mut rng = rand::thread_rng();

    loop {
        let mut newcards = cards.clone(); // [TODO] copy into instead?
        rng.shuffle(&mut newcards);
        for r in 0..51 {
            let mut c = newcards.clone();
            c.rotate_right(r);
            let mut p1d = deal(c.clone(), false);
            record_best_play_one(
                &mut p1d,
                &mut highscore,
                &mut best_game,
                &mut best_game_unplay,
            );
            record_best_play_one(
                &mut deal(c.clone(), true),
                &mut highscore,
                &mut best_game,
                &mut best_game_unplay,
            );
            record_best_play_one(
                &mut deal(c.clone().into_iter().rev().collect(), false),
                &mut highscore,
                &mut best_game,
                &mut best_game_unplay,
            );
            record_best_play_one(
                &mut deal(c.clone().into_iter().rev().collect(), true),
                &mut highscore,
                &mut best_game,
                &mut best_game_unplay,
            );
            counter += 4;
        }
        if counter % 1000000 == 0 {
            println!(
                "{} best shuffled, {} best unplayed at {} games per second in play_many {}",
                best_game,
                best_game_unplay,
                counter / start.elapsed().as_secs(),
                VERSION
            );
        }
        if counter > 10000000000 {
            // if counter > 2000000 {
            println!("{} games played", counter);
            break;
        }
    }
}

fn record_best_play_one(
    gs: &mut GameState,
    highscore: &mut u16,
    best_game: &mut GameState,
    best_game_unplay: &mut GameState,
) {
    // let p1_pen_card_count = count_penalty_cards(&g.p1deal);
    // if p1_pen_card_count > 11 || p1_pen_card_count < 5 {
    //	continue;
    // }
    play_one(&mut gs.game);

    if gs.game.steps > *highscore {
        *highscore = gs.game.steps;
        *best_game = gs.clone();
        println!("{}", best_game.clone());
        let (mut hand_one_unplay, mut hand_two_unplay) = (gs.p1deal.clone(), gs.p2deal.clone());
        hand_one_unplay.reverse();
        hand_two_unplay.reverse();
        let game_unplay = record_26s_top(hand_one_unplay, hand_two_unplay);
        if game_unplay.game.steps > best_game_unplay.game.steps {
            *best_game_unplay = game_unplay.clone();
            println!("{} best_game_unplay", best_game_unplay);
        }
    }
}

fn play_one(g: &mut Game) {
    while let Some(card) = g.p1hand.pop() {
        play_one_step(card, g);
        g.steps += 1; // add one to steps
                      // if g.steps > 8500 {
                      //     break; // this is a record breaker
                      // }
    }
}

fn play_one_step(card: u8, g: &mut Game) {
    if card > 0 {
        // is this next card a penalty card?
        penalty_card(g, card);
    } else {
        // it's not a penalty card, but we still have tribute to pay
        if g.penalty > 0 {
            pay_tribute(g, card);
        } else {
            // nothing going on, play a card into the pot
            boring_card(g, card);
        }
    }
}

fn penalty_card(g: &mut Game, card: u8) {
    g.penalty = card;
    boring_card(g, card);
}

fn boring_card(g: &mut Game, card: u8) {
    g.pot.push(card);
    g.swap(); // swap hands, other player is active
}

fn pay_tribute(g: &mut Game, card: u8) {
    // penalty is active, and this is not a penalty card
    g.pot.push(card); // put this card in the pot
    g.penalty -= 1; // subtract one from penalty
    if g.penalty == 0 {
        penalty_check(g);
    }
}

fn penalty_check(g: &mut Game) {
    // battle is done, add pot to the non-active player's hand
    g.pot.reverse();
    g.pot.append(&mut g.p2hand);
    g.p2hand.append(&mut g.pot); // add the pot
                                 // g.p2hand.splice(..0, g.pot.drain(..));
    g.swap(); // swap hands, winner is now active player
}

fn play_one_check(g: &mut Game, deals: &mut Vec<(Vec<u8>, Vec<u8>)>) {
    while let Some(card) = g.p1hand.pop() {
        if card > 0 {
            // is this next card a penalty card?
            penalty_card(g, card);
        } else {
            // it's not a penalty card, but we still have tribute to pay
            if g.penalty > 0 {
                pay_tribute_check(g, card, deals);
            } else {
                // nothing going on, play a card into the pot
                boring_card(g, card);
            }
        }
        g.steps += 1; // add one to steps
        if g.steps > 8500 {
            break; // this is a record breaker
        }
    }
}

fn pay_tribute_check(g: &mut Game, card: u8, deals: &mut Vec<(Vec<u8>, Vec<u8>)>) {
    // penalty is active, and this is not a penalty card
    g.pot.push(card); // put this card in the pot
    g.penalty -= 1; // subtract one from penalty
    if g.penalty == 0 {
        // battle is done, add pot to the non-active player's hand
        g.pot.reverse();
        g.pot.append(&mut g.p2hand);
        g.p2hand.append(&mut g.pot); // add the pot
        g.swap(); // swap hands, winner is now active player
        if g.p2hand.len() == 26 && g.p1hand.len() == 26 {
            deals.push((g.p1hand.clone(), g.p2hand.clone()));
        }
    }
}

// the actual game type
#[derive(Clone)]
pub struct GameState {
    // initial_deal : (Vec<u8>,Vec<u8>),
    p1deal: Vec<u8>,
    p2deal: Vec<u8>,
    game: Game,
}
#[derive(Clone)]
pub struct Game {
    p1hand: Vec<u8>,
    p2hand: Vec<u8>,
    pot: Vec<u8>,
    penalty: u8,
    steps: u16,
}

// how to display the game type
impl fmt::Display for GameState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let p1: String = self.p1deal.iter().rev().map(show_card).collect();
        let p2: String = self.p2deal.iter().rev().map(show_card).collect();
        write!(fmt, "{} {} {}", self.game.steps, p1, p2)
    }
}

// create a game from two strings
fn read_game(p1: &str, p2: &str) -> GameState {
    let (p1deal, p2deal) = read_hands(p1, p2);
    // p1deal.reverse();
    // p2deal.reverse();
    return make_game(p1deal, p2deal);
}

fn read_hands(p1: &str, p2: &str) -> (Vec<u8>, Vec<u8>) {
    let p1hand: Vec<u8> = p1.chars().map(read_card).collect();
    let p2hand: Vec<u8> = p2.chars().map(read_card).collect();
    return (p1hand, p2hand);
}

fn show_card(c: &u8) -> char {
    return match c {
        4 => 'A',
        3 => 'K',
        2 => 'Q',
        1 => 'J',
        0 => '-',
        _ => 'X',
    };
}

fn read_card(c: char) -> u8 {
    return match c {
        'A' => 4,
        'K' => 3,
        'Q' => 2,
        'J' => 1,
        '-' => 0,
        _ => 0,
    };
}

// fn count_penalty_cards(vd: &Vec<u8>) -> u8 {
//     return vd.into_iter().filter(|x| **x > 0).count() as u8;
// }

fn deal(mut cards: Vec<u8>, swap: bool) -> GameState {
    let mut deal1: Vec<u8> = Vec::with_capacity(64);
    let mut deal2: Vec<u8> = Vec::with_capacity(64);
    deal1.append(&mut cards.split_off(26)); // first 26 cards dealt to p1
    deal2.append(&mut cards); // last 26 cards dealt to p2
    let g: GameState;

    if swap {
        g = make_game(deal2, deal1);
    } else {
        g = make_game(deal1, deal2);
    }
    return g;
}

fn make_game(mut deal1: Vec<u8>, mut deal2: Vec<u8>) -> GameState {
    deal1.reverse(); // XXX I *do* really need this, because other code depends on it, ARGH
    deal2.reverse();
    return GameState {
        p1deal: deal1.clone(),
        p2deal: deal2.clone(),
        game: Game {
            p1hand: deal1,
            p2hand: deal2,
            pot: Vec::with_capacity(64),
            penalty: 0,
            steps: 0,
        },
    };
}

impl Game {
    // swap the hands when the other player becomes active
    fn swap(&mut self) {
        swap(&mut self.p1hand, &mut self.p2hand);
    }
}

fn check_it(p1: &str, p2: &str, steps: u16) {
    let (p1_copy, p2_copy) = (p1, p2).clone();
    let mut game_state = read_game(p1, p2);
    play_one(&mut game_state.game);
    println!("testing {} is {} {}", game_state, p1_copy, p2_copy);

    assert_eq!(game_state.game.steps, steps); // sanity check, convert to test?
}

fn check_all() {
    check_it(
        "---AJ--Q---------QAKQJJ-QK",
        "-----A----KJ-K--------A---",
        8345,
    );
    check_it(
        "------------KAQ----J------",
        "-JQQK---K----JK--QA-A-JA--",
        4791,
    );
    check_it(
        "---JQ---K-A----A-J-K---QK-",
        "-J-----------AJQA----K---Q",
        5790,
    );
    check_it(
        "A-QK------Q----KA-----J---",
        "-JAK----A--Q----J---QJ--K-",
        6913,
    );
    check_it(
        "K-KK----K-A-----JAA--Q--J-",
        "---Q---Q-J-----J------AQ--",
        7158,
    );
    check_it(
        "----Q------A--K--A-A--QJK-",
        "-Q--J--J---QK---K----JA---",
        7208,
    );
    check_it(
        "--A-Q--J--J---Q--AJ-K---K-",
        "-J-------Q------A--A--QKK-",
        7226,
    );
    check_it(
        "-J------Q------AAA-----QQ-",
        "K----JA-----------KQ-K-JJK",
        7959,
    );
    check_it(
        "----K---A--Q-A--JJA------J",
        "-----KK---------A-JK-Q-Q-Q",
        7972,
    );
    check_it(
        "---Q--Q--J-Q-J----------A-",
        "--K-K-KAQ-AA-----J-J-----K",
        5676,
    );
    check_it(
        "-J-QAA-----Q---K---Q-K--K-",
        "-A-----Q---J---KJ-A-----J-",
        5328,
    );
}

fn untrickable(hand: Vec<u8>) -> bool {
    let mut rev_hand_clone = hand.clone();
    rev_hand_clone.reverse();
    match rev_hand_clone.iter().position(|&x| x != 0) {
        Some(non_zero_position) => {
            let fnz = non_zero_position as usize;
            let (tail_zeros, pre) = rev_hand_clone.split_at(fnz);
            return tail_zeros.len() == pre[0] as usize;
        }
        _ => return false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    proptest! {
    #[test]
    fn prop_test_untrick(a in 1..5) {
    let mut c_v : Vec<u8> = vec![a as u8];
    for _z in 1 .. a+1 {
    c_v.push(0);
    }
    let mut res = c_v.clone();
    res.reverse();
    prop_assert_eq!(untrick(&mut c_v),vec![(res,vec![])]);
    }
    #[test]
    fn prop_test_untrickable_true(a in 1..5) {
    let mut c_v : Vec<u8> = vec![a as u8];
    for _z in 1 .. a+1 {
    c_v.push(0);
    }
    let mut res = c_v.clone();
    res.reverse();
    prop_assert_eq!(untrickable(c_v),true);
    }
    #[test]
    fn prop_test_untrickable_false(a in 1..5) {
    let mut c_v : Vec<u8> = vec![a as u8];
    for _z in 1 .. a {
    c_v.push(0);
    }
    let mut res = c_v.clone();
    res.reverse();
    prop_assert_eq!(untrickable(c_v),false);
    }
    }

    #[test]
    fn test_untrickable() {
        assert_eq!(untrickable(vec![1, 0]), true);
        assert_eq!(untrickable(vec![1]), false);
        assert_eq!(untrickable(vec![2, 0, 0]), true);
        assert_eq!(untrickable(vec![2, 0]), false);
        assert_eq!(untrickable(vec![3, 0, 0, 0]), true);
        assert_eq!(untrickable(vec![4, 0, 0, 0, 0]), true);
        assert_eq!(untrickable(vec![4, 0, 0]), false);
        assert_eq!(untrickable(vec![4, 0, 0, 0]), false);
    }
    #[test]
    fn test_untrick() {
        assert_eq!(untrick(&mut vec![1, 0]), vec![(vec![0, 1], vec![])]);
        assert_eq!(untrick(&mut vec![2, 0, 0]), vec![(vec![0, 0, 2], vec![])]);
        assert_eq!(
            untrick(&mut vec![3, 0, 0, 0]),
            vec![(vec![0, 0, 0, 3], vec![])]
        );
        assert_eq!(untrick(&mut vec![0, 0]), vec![]);
        assert_eq!(
            untrick(&mut vec![0, 1]),
            vec![(vec![1], vec![0]), (vec![1, 0], vec![])]
        );

        assert_eq!(
            untrick(&mut vec![0, 0, 1]),
            vec![
                (vec![1], vec![0, 0]),
                (vec![1, 0], vec![0]),
                (vec![1, 0, 0], vec![])
            ]
        );

        assert_eq!(
            untrick(&mut vec![0, 4]),
            vec![(vec![4], vec![0]), (vec![4, 0], vec![]),]
        );
        assert_eq!(
            untrick(&mut vec![0, 0, 4]),
            vec![
                (vec![4], vec![0, 0]),
                (vec![4, 0], vec![0]),
                (vec![4, 0, 0], vec![]),
            ]
        );
        assert_eq!(
            untrick(&mut vec![0, 4, 0, 3, 2]),
            vec![
                (vec![2], vec![0, 4, 0, 3]),
                (vec![2, 3], vec![0, 4, 0]),
                (vec![2, 3, 0], vec![0, 4]),
                (vec![2, 3, 0, 4], vec![0]),
                (vec![2, 3, 0, 4, 0], vec![])
            ]
        );
        assert_eq!(
            untrick(&mut vec![0, 4, 0, 3, 2, 0]),
            vec![
                (vec![0, 2], vec![0, 4, 0, 3]),
                (vec![0, 2, 3], vec![0, 4, 0]),
                (vec![0, 2, 3, 0], vec![0, 4]),
                (vec![0, 2, 3, 0, 4], vec![0]),
                (vec![0, 2, 3, 0, 4, 0], vec![])
            ]
        );
        assert_eq!(
            untrick(&mut vec![3, 0, 0, 0, 1, 0]),
            vec![
                (vec![0, 1], vec![3, 0, 0, 0]),
                (vec![0, 1, 0], vec![3, 0, 0]),
                (vec![0, 1, 0, 0], vec![3, 0]),
                (vec![0, 1, 0, 0, 0], vec![3])
            ]
        );
        assert_eq!(
            untrick(&mut vec![
                0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 3, 0, 0, 3,
                4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1, 0, 1, 0, 3, 4, 2, 0, 0
            ]),
            vec![
                (
                    vec![0, 0, 2],
                    vec![
                        0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 3,
                        0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1, 0, 1, 0, 3, 4
                    ]
                ),
                (
                    vec![0, 0, 2, 4],
                    vec![
                        0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 3,
                        0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1, 0, 1, 0, 3
                    ]
                ),
                (
                    vec![0, 0, 2, 4, 3],
                    vec![
                        0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 3,
                        0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1, 0, 1, 0
                    ]
                ),
                (
                    vec![0, 0, 2, 4, 3, 0],
                    vec![
                        0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 3,
                        0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1, 0, 1
                    ]
                )
            ]
        );
    }
    #[test]
    fn test_districk() {
        assert_eq!(districk(vec![], (vec![1, 0], vec![])), (vec![0], vec![1]));
        assert_eq!(
            districk(vec![], (vec![2, 0, 0], vec![])),
            (vec![0, 2], vec![0])
        );
        assert_eq!(
            districk(vec![], (vec![0, 1, 0], vec![0])),
            (vec![0, 0], vec![1, 0])
        );
        assert_eq!(
            districk(vec![], (vec![2, 0, 1, 0], vec![1, 0])),
            (vec![0, 0, 2, 1, 0], vec![1])
        );
        assert_eq!(
            districk(vec![], (vec![2, 0, 1, 0], vec![1, 0, 1])),
            (vec![0, 0, 2, 1, 0, 1], vec![1])
        );
        assert_eq!(
            districk(vec![1], (vec![2, 0, 1, 0], vec![1, 0, 1])),
            (vec![0, 0, 2, 1, 0, 1], vec![1, 1])
        );
        assert_eq!(
            districk(vec![], (vec![2, 0, 1, 0], vec![4, 0, 0, 0, 1, 0])),
            (vec![0, 0, 2, 4, 0, 0, 0, 1, 0], vec![1])
        );
        assert_eq!(
            districk(vec![], (vec![2, 0, 1, 0], vec![4, 0, 0, 0, 1, 0])),
            (vec![0, 0, 2, 4, 0, 0, 0, 1, 0], vec![1])
        );
    }
    #[test]
    fn test_split_at() {
        let (left, right) = [1, 2, 3].split_at(1);
        assert_eq!(left, vec![1]);
        assert_eq!(right, vec![2, 3]);
    }
    #[test]
    fn test_find() {
        let v = vec![0, 1];
        if let Some(i) = v.iter().find(|&x| x != &0) {
            assert_eq!(i, &1);
        }
    }
    #[test]
    fn test_unplay() {
        assert_eq!(unplay(vec![1, 0], vec![]), vec![(vec![1], vec![0])]);
        assert_eq!(
            unplay(vec![1, 0], vec![0, 1]),
            vec![(vec![1], vec![0, 0, 1])]
        );
        assert_eq!(
            unplay(vec![1, 0], vec![0, 1, 0]),
            vec![(vec![1], vec![0, 0, 1, 0])]
        );
        assert_eq!(
            unplay(vec![1, 0], vec![2, 0, 0]),
            vec![(vec![1], vec![0, 2, 0, 0])]
        );
        assert_eq!(
            unplay(vec![1, 0], vec![1, 0]),
            vec![(vec![1], vec![0, 1, 0])]
        );
        assert_eq!(unplay(vec![4, 0], vec![0, 2, 0]), vec![]);
        assert_eq!(
            unplay(vec![2, 0, 0], vec![0, 2, 0]),
            vec![(vec![2], vec![0, 0, 0, 2, 0])]
        );
        let output = vec![
            (
                vec![
                    2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 3,
                    0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1, 0, 1, 0, 3, 4,
                ],
                vec![0, 0],
            ),
            (
                vec![4, 0, 0],
                vec![
                    2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 3,
                    0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1, 0, 1, 0, 3,
                ],
            ),
            (
                vec![
                    3, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2,
                    3, 0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1, 0, 1, 0,
                ],
                vec![4, 0, 0],
            ),
            (
                vec![
                    1, 3, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0,
                    2, 3, 0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1, 0,
                ],
                vec![0, 4, 0, 0],
            ),
            (
                vec![
                    1, 1, 3, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0,
                    0, 2, 3, 0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0,
                ],
                vec![0, 0, 4, 0, 0],
            ),
            (
                vec![0, 0, 0, 4, 0, 0],
                vec![
                    1, 1, 3, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0,
                    0, 2, 3, 0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0,
                ],
            ),
            (
                vec![
                    0, 1, 1, 3, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2,
                    0, 0, 2, 3, 0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4,
                ],
                vec![0, 0, 0, 4, 0, 0],
            ),
            (
                vec![4, 0, 0, 4, 0, 0],
                vec![
                    0, 0, 1, 1, 3, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
                    2, 0, 0, 2, 3, 0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0,
                ],
            ),
            (
                vec![
                    0, 0, 0, 1, 1, 3, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
                    0, 2, 0, 0, 2, 3, 0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0,
                ],
                vec![4, 0, 0, 4, 0, 0],
            ),
            (
                vec![0, 4, 0, 0, 4, 0, 0],
                vec![
                    0, 0, 0, 1, 1, 3, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
                    0, 2, 0, 0, 2, 3, 0, 0, 3, 4, 0, 4, 0, 0, 1, 0,
                ],
            ),
            (
                vec![
                    0, 0, 0, 0, 1, 1, 3, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
                    0, 0, 2, 0, 0, 2, 3, 0, 0, 3, 4, 0, 4, 0, 0, 1,
                ],
                vec![0, 4, 0, 0, 4, 0, 0],
            ),
            (
                vec![0, 0, 4, 0, 0],
                vec![
                    1, 3, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0,
                    2, 3, 0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1,
                ],
            ),
            (
                vec![0, 4, 0, 0],
                vec![
                    3, 2, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2,
                    3, 0, 0, 3, 4, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1, 0, 1,
                ],
            ),
        ];
        let big = vec![
            0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 3, 0, 0, 3, 4,
            0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 1, 0, 1, 0, 3, 4, 2, 0, 0,
        ];
        assert_eq!(unplay(big, vec![]), output);
        let big_res = [
            (
                [
                    3, 2, 0, 0, 0, 0, 2, 0, 1, 2, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                ]
                .to_vec(),
                [
                    0, 0, 0, 0, 0, 0, 3, 4, 4, 0, 2, 4, 0, 0, 0, 0, 0, 3, 0, 1, 0, 0, 0, 0, 0, 4,
                    0, 0, 3,
                ]
                .to_vec(),
            ),
            (
                [
                    0, 0, 0, 0, 0, 0, 0, 3, 4, 4, 0, 2, 4, 0, 0, 0, 0, 0, 3, 0, 1, 0, 0, 0, 0, 0,
                    4, 0, 0, 3,
                ]
                .to_vec(),
                [
                    3, 2, 0, 0, 0, 0, 2, 0, 1, 2, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0,
                ]
                .to_vec(),
            ),
            (
                [
                    0, 3, 2, 0, 0, 0, 0, 2, 0, 1, 2, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0,
                ]
                .to_vec(),
                [
                    0, 0, 0, 0, 0, 0, 0, 3, 4, 4, 0, 2, 4, 0, 0, 0, 0, 0, 3, 0, 1, 0, 0, 0, 0, 0,
                    4, 0, 0, 3,
                ]
                .to_vec(),
            ),
            (
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 4, 0, 2, 4, 0, 0, 0, 0, 0, 3, 0, 1, 0, 0, 0, 0,
                    0, 4, 0, 0, 3,
                ]
                .to_vec(),
                [
                    0, 3, 2, 0, 0, 0, 0, 2, 0, 1, 2, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
                ]
                .to_vec(),
            ),
            (
                [
                    0, 0, 3, 2, 0, 0, 0, 0, 2, 0, 1, 2, 0, 0, 0, 1, 0, 0, 1, 0, 0,
                ]
                .to_vec(),
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 4, 0, 2, 4, 0, 0, 0, 0, 0, 3, 0, 1, 0, 0, 0, 0,
                    0, 4, 0, 0, 3,
                ]
                .to_vec(),
            ),
            (
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 4, 0, 2, 4, 0, 0, 0, 0, 0, 3, 0, 1, 0, 0, 0,
                    0, 0, 4, 0, 0, 3,
                ]
                .to_vec(),
                [0, 0, 3, 2, 0, 0, 0, 0, 2, 0, 1, 2, 0, 0, 0, 1, 0, 0, 1, 0].to_vec(),
            ),
            (
                [0, 0, 0, 3, 2, 0, 0, 0, 0, 2, 0, 1, 2, 0, 0, 0, 1, 0, 0, 1].to_vec(),
                [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 4, 0, 2, 4, 0, 0, 0, 0, 0, 3, 0, 1, 0, 0, 0,
                    0, 0, 4, 0, 0, 3,
                ]
                .to_vec(),
            ),
        ]
        .to_vec();
        assert_eq!(
            unplay(
                [2, 0, 0, 0, 0, 2, 0, 1, 2, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0]
                    .to_vec(),
                [0, 0, 0, 3, 4, 4, 0, 2, 4, 0, 0, 0, 0, 0, 3, 0, 1, 0, 0, 0, 0, 0, 4, 0, 0, 3]
                    .to_vec()
            ),
            big_res
        );
    }
    #[test]
    fn test_next_deck_26s() {
        // nextDeck26s (dispRev "-JA--Q-JK--------Q-JKK----") (dispRev "----J----A-A---Q-A----QK--")
        // ([0,0,0,0,4,0,4,0,0,0,2,0,4,0,0,0,0,2,3,0,0,4,0,0,1,0],[0,2,0,1,3,0,0,0,0,0,0,0,0,2,0,1,3,3,0,0,0,0,0,0,1,0])
        let (hand_one, hand_two) =
            read_hands("-JA--Q-JK--------Q-JKK----", "----J----A-A---Q-A----QK--");
        let res = next_deck_26s(hand_one, hand_two);
        assert_eq!(
            res,
            (
                [0, 0, 0, 0, 4, 0, 4, 0, 0, 0, 2, 0, 4, 0, 0, 0, 0, 2, 3, 0, 0, 4, 0, 0, 1, 0]
                    .to_vec(),
                [0, 2, 0, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 3, 3, 0, 0, 0, 0, 0, 0, 1, 0]
                    .to_vec()
            )
        );
        // nextDeck26s  (dispRev "-------J-J--Q-KK-------Q--") (dispRev "---Q---J--AA-A---AQKKJ----")
        // ([3,3,1,0,0,0,0,0,0,0,0,0,0,0,2,0,0,0,4,0,0,0,0,2,0,0].to_vec(),[2,0,0,0,0,0,1,0,0,1,1,0,0,0,0,4,2,4,0,3,0,4,3,0,0,0].to_vec())
        let (hand_one, hand_two) =
            read_hands("-------J-J--Q-KK-------Q--", "---Q---J--AA-A---AQKKJ----");
        let res = next_deck_26s(hand_one, hand_two);
        assert_eq!(
            res,
            (
                [3, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 0, 2, 0, 0]
                    .to_vec(),
                [2, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 4, 2, 4, 0, 3, 0, 4, 3, 0, 0, 0]
                    .to_vec()
            )
        );
        // nextDeck26s   (dispRev "J-K---------A-Q------QQ---") (dispRev "--A-AJ----KQ---JK---KA---J")
        // ([0,0,0,1,0,0,0,0,0,3,0,3,4,2,0,0,0,0,0,0,4,0,0,4,1,0].to_vec(),[0,0,0,0,3,2,2,0,0,0,0,0,0,0,0,1,0,2,0,3,0,0,0,4,1,0].to_vec())
        let (hand_one, hand_two) =
            read_hands("J-K---------A-Q------QQ---", "--A-AJ----KQ---JK---KA---J");
        let res = next_deck_26s(hand_one, hand_two);
        assert_eq!(
            res,
            (
                [0, 0, 0, 1, 0, 0, 0, 0, 0, 3, 0, 3, 4, 2, 0, 0, 0, 0, 0, 0, 4, 0, 0, 4, 1, 0]
                    .to_vec(),
                [0, 0, 0, 0, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 2, 0, 3, 0, 0, 0, 4, 1, 0]
                    .to_vec()
            )
        );
    }
    #[test]
    fn test_fast_play_next_26s() {
        /* fastPlayNext26s [] (Player 0 mempty (dispRev "J--A----K-Q------AA-J--KQ-")) (Player 0 mempty (dispRev "-J--Q--K-A-QK-----------J-")) (-1) 0
        ([0,0,0,0,4,0,0,0,0,4,0,0,0,0,0,0,0,3,1,2,0,0,0,4,1,0].to_vec(),[2,0,0,0,0,0,0,3,3,0,4,0,2,0,2,0,0,3,0,0,0,0,0,1,1,0].to_vec()) */
        let (hand_one, hand_two) =
            read_hands("J--A----K-Q------AA-J--KQ-", "-J--Q--K-A-QK-----------J-");
        assert_eq!(
            fast_play_next_26s(hand_one, hand_two),
            (
                [0, 0, 0, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 3, 1, 2, 0, 0, 0, 4, 1, 0]
                    .to_vec(),
                [2, 0, 0, 0, 0, 0, 0, 3, 3, 0, 4, 0, 2, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 1, 1, 0]
                    .to_vec()
            )
        );
        /* fastPlayNext26s [] (Player 0 mempty (dispRev "-JA--Q-JK--------Q-JKK----")) (Player 0 mempty (dispRev "----J----A-A---Q-A----QK--")) (-1) 0
        ([0,0,0,0,4,0,4,0,0,0,2,0,4,0,0,0,0,2,3,0,0,4,0,0,1,0].to_vec(),[0,2,0,1,3,0,0,0,0,0,0,0,0,2,0,1,3,3,0,0,0,0,0,0,1,0].to_vec()) */
        let (hand_one, hand_two) =
            read_hands("-JA--Q-JK--------Q-JKK----", "----J----A-A---Q-A----QK--");
        assert_eq!(
            fast_play_next_26s(hand_one, hand_two),
            (
                [0, 0, 0, 0, 4, 0, 4, 0, 0, 0, 2, 0, 4, 0, 0, 0, 0, 2, 3, 0, 0, 4, 0, 0, 1, 0]
                    .to_vec(),
                [0, 2, 0, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 1, 3, 3, 0, 0, 0, 0, 0, 0, 1, 0]
                    .to_vec()
            )
        );
    }
}

// record26sTop h0 h1 = uncurry record26s_alt_disp (nextDeck26s h0 h1)
fn record_26s_top(hand_one: Vec<u8>, hand_two: Vec<u8>) -> GameState {
    let (hand_one_next, hand_two_next) = next_deck_26s(hand_one, hand_two);
    return record26s_alt_disp(hand_one_next, hand_two_next);
}

fn record26s_alt_disp(hand_one: Vec<u8>, hand_two: Vec<u8>) -> GameState {
    let mut best_game = make_game(hand_one.clone(), hand_two.clone());
    let all_unplays = unplay(hand_one.clone(), hand_two.clone());
    let unplay_26 = all_unplays
        .iter()
        .filter(|(h1, h2)| h1.len() == 26 && h2.len() == 26);
    for pair in unplay_26 {
        let mut g = make_game(pair.0.clone(), pair.1.clone());
        play_one(&mut g.game); // XXX sort here
        if g.game.steps > best_game.game.steps {
            best_game = g.clone();
        }
    }
    return best_game;
}

/* nextDeck26s :: [Word8] -> [Word8] -> ([Word8], [Word8])
nextDeck26s h0 h1 | length h0 >= 52 && length h1 <=0 = (h0, h1)
nextDeck26s h0 h1 | length h1 >= 52 && length h0 <=0 = (h1, h0)
nextDeck26s h0 h1 = fastPlayNext26s [] (Player 0 mempty h0) (Player 0 mempty h1) (-1) 0 */
fn next_deck_26s(hand_one: Vec<u8>, hand_two: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    // either the game is done and we bail, or we return the next post-trick state that could be normally dealt
    if hand_one.len() >= 52 && hand_two.len() <= 0 {
        return (hand_one, hand_two);
    } else if hand_two.len() >= 52 && hand_one.len() <= 0 {
        return (hand_two, hand_one);
    }
    return fast_play_next_26s(hand_one, hand_two);
}

// why only one? why not all of them?
fn fast_play_next_26s(hand_one: Vec<u8>, hand_two: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let mut g = make_game(hand_one, hand_two);
    let mut deals: Vec<(Vec<u8>, Vec<u8>)> = vec![];
    play_one_check(&mut g.game, &mut deals);
    return match deals.first() {
        Some((h1, h2)) => {
            let (mut h1_ret, mut h2_ret) = (h1.clone(), h2.clone());
            h1_ret.reverse();
            h2_ret.reverse();
            return (h1_ret, h2_ret);
        }
        None => (vec![], vec![]),
    };
}

fn unplay(mut hand_one: Vec<u8>, hand_two: Vec<u8>) -> Vec<(Vec<u8>, Vec<u8>)> {
    if !(untrickable(hand_one.clone())) && (hand_one.len() < 52) {
        return vec![];
    };
    // concatMap (loopUnplay h1) (untrick h0)
    let foo = untrick(&mut hand_one);
    let bar: Vec<(Vec<u8>, Vec<u8>)> = foo
        .iter()
        .flat_map(|trick| loop_unplay(hand_two.clone(), trick))
        .collect();
    return bar;
}

fn loop_unplay(losing_hand: Vec<u8>, (nl, nw): &(Vec<u8>, Vec<u8>)) -> Vec<(Vec<u8>, Vec<u8>)> {
    let (nl_new, nw_new) = districk(losing_hand, (nl.to_vec(), nw.to_vec()));
    let mut tail = unplay(nl_new.clone(), nw_new.clone());
    let mut result = vec![(nl_new, nw_new)];
    result.append(&mut tail);
    return result;
}

fn districk(
    losing_hand: Vec<u8>,
    (rtrick, winning_hand): (Vec<u8>, Vec<u8>),
) -> (Vec<u8>, Vec<u8>) {
    let (a, b) = districk_go(losing_hand, winning_hand, vec![], rtrick);
    return (b, a);
}

fn districk_go(
    mut other_hand: Vec<u8>,
    mut taking_hand: Vec<u8>,
    mut zeros: Vec<u8>,
    rtrick: Vec<u8>,
) -> (Vec<u8>, Vec<u8>) {
    if rtrick.len() == 0 {
        // go oh' th' zs [] = foldr (\c (oh, th) -> (th, c:oh)) (th', oh') zs
        return zeros
            .iter()
            .rfold((taking_hand, other_hand), |acc: (Vec<u8>, Vec<u8>), &x| {
                // (\c (oh, th) -> (th, c:oh))
                let mut new_snd = vec![x];
                new_snd.append(&mut acc.0.clone());
                let new_fst = acc.1.clone();
                return (new_fst, new_snd);
            });
    }
    if rtrick[0] == 0 {
        let (c, rt) = rtrick.split_at(1);
        let mut new_zeros = c.to_vec();
        new_zeros.append(&mut zeros);
        return districk_go(other_hand, taking_hand, new_zeros, rt.to_vec());
    }
    // go oh th p (c:rt) = go (c:th) (p<>oh) [] rt
    let (c, new_rtrick) = rtrick.split_at(1);
    let mut new_other_hand = c.to_vec();
    new_other_hand.append(&mut taking_hand);
    let mut new_taking_hand = zeros;
    new_taking_hand.append(&mut other_hand);
    let new_rev_trick = new_rtrick.to_vec();
    // let mut new_taking_hand = zeros.append(&mut other_hand);
    return districk_go(new_other_hand, new_taking_hand, vec![], new_rev_trick);
}

fn untrick(hand: &mut Vec<u8>) -> Vec<(Vec<u8>, Vec<u8>)> {
    hand.reverse();
    let mut result = untrick_go(0, 0, vec![], hand);
    for r in &mut result[..] {
        r.0.reverse();
    }
    return result;
}

// this could be a loop if recursion becomes a problem
fn untrick_go(
    fvs: usize,
    zs: usize,
    trick: Vec<u8>,
    reversed_hand: &mut Vec<u8>,
) -> Vec<(Vec<u8>, Vec<u8>)> {
    // ugh, really?
    if reversed_hand.len() == 0 {
        return vec![];
    }
    let (b, rhand) = reversed_hand.split_at(1);
    let mut rev_rhand = rhand.to_vec().clone();
    rev_rhand.reverse();
    let trick_clone = trick.clone();
    let newtrick = ([b, &trick_clone[..]].concat(), rev_rhand.clone());
    if b[0] == 0 && fvs > 0 {
        let newtrick_clone = newtrick.clone();
        let mut tail = untrick_go(fvs, zs + 1, newtrick_clone.0, &mut rhand.to_vec());
        let mut result = vec![newtrick];
        result.append(&mut tail);
        return result;
    }
    if b[0] == 0 {
        return untrick_go(fvs, zs + 1, newtrick.0, &mut rhand.to_vec().clone());
    }
    if fvs == 0 && b[0] as usize == zs {
        let fvs_b = fvs + b[0] as usize;
        let newtrick_clone = newtrick.clone();
        let mut result = untrick_go(fvs_b, fvs_b, newtrick_clone.0, &mut rhand.to_vec().clone());
        let mut trick_res: Vec<(Vec<u8>, Vec<u8>)> = vec![newtrick];
        trick_res.append(&mut result);
        return trick_res;
    }
    if fvs + b[0] as usize == zs {
        return vec![];
    }
    if fvs + b[0] as usize >= zs {
        let fvs_b = fvs + b[0] as usize;
        let newtrick_clone = newtrick.clone();
        let mut result = untrick_go(fvs_b, fvs_b, newtrick_clone.0, &mut rhand.to_vec().clone());
        let mut trick_res: Vec<(Vec<u8>, Vec<u8>)> = vec![newtrick];
        trick_res.append(&mut result);
        return trick_res;
    }
    return vec![];
}
