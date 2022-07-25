// use rand::Rng;
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
        // let mut test_game = read_game("-Q-Q-----JJ-K-----K-A--AKK", "---J-------A-----Q--J-A-Q-");
        let mut g8344 = read_game("---AJ--Q---------QAKQJJ-QK", "-----A----KJ-K--------A---");
        play_one(&mut g8344.game);
        assert_eq!(g8344.game.steps, 8345); // sanity check, convert to test?
    }
    // let unplay_game = read_game("-----K-A--AKK--Q---JQ------------JAJ---K---", "Q--J-A-Q-");
    // println!("{}",unplay_game);
    play_many(cards);
}

fn play_many(cards: Vec<u8>) {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut counter = 0;
    let mut highscore = 1;
    let mut best_game = deal(cards.clone(), false);
    let start = Instant::now();
    // let mut rng = rand::thread_rng();
    let mut games_best: Vec<GameState> = good_games();
    games_best.sort_by(|a, b| a.game.steps.cmp(&b.game.steps));
    // games_best.reverse();
    while let Some(mut gamestate) = games_best.pop() {
        // let mut newcards = cards.clone(); // [TODO] copy into instead?
        // rng.shuffle(&mut newcards);
        while let Some(mut wiggled_game) = wiggle_all(&mut gamestate).pop() {
            let p1_pen_card_count = count_penalty_cards(&wiggled_game.p1deal);
            if p1_pen_card_count > 11 || p1_pen_card_count < 5 {
                continue; // world record hands have no more than 11 and no less than 5 penalty cards
            }
            play_one(&mut wiggled_game.game);
            if wiggled_game.game.steps > highscore {
                highscore = wiggled_game.game.steps;
                let game_clone = wiggled_game.clone();
                best_game = game_clone;
                games_best.push(wiggled_game);
                println!("{}", best_game);
            }
            counter += 1;
            if counter % 3000000 == 0 {
                if let Some(recent_game) = games_best.last() {
                    println!(
			"{} best so far at {} games per second in play_many {}, games_best is {:?}, {} recent_game",
			best_game,
			counter / (start.elapsed().as_secs() + 1),
			VERSION,
			games_best,
			recent_game
		    );
                }
            }
            if counter > 10000000000 {
                // if counter > 2000000 {
                println!("{} games played", counter);
                break;
            }
        }
    }
}

fn play_one(g: &mut Game) {
    while let Some(card) = g.p1hand.pop() {
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
        g.steps += 1; // add one to steps
        if g.steps > 8500 {
            break; // this is a record breaker
        }
    }
}

fn pay_tribute(g: &mut Game, card: u8) {
    // penalty is active, and this is not a penalty card
    g.pot.push(card); // put this card in the pot
    g.penalty -= 1; // subtract one from penalty
    if g.penalty == 0 {
        // battle is done, add pot to the non-active player's hand
        g.pot.reverse();
        g.pot.append(&mut g.p2hand);
        g.p2hand.append(&mut g.pot); // add the pot
        g.swap(); // swap hands, winner is now active player
    }
}

fn penalty_card(g: &mut Game, card: u8) {
    g.penalty = card;
    g.pot.push(card);
    g.swap(); // swap hands, other player is active
}

fn boring_card(g: &mut Game, card: u8) {
    g.pot.push(card);
    g.swap(); // swap hands, other player is active
}

// the actual game type
#[derive(Debug, Clone)]
pub struct GameState {
    p1deal: Vec<u8>,
    p2deal: Vec<u8>,
    game: Game,
}
#[derive(Debug, Clone)]
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
        let p1: String = self.p1deal.iter().map(show_card).collect();
        let p2: String = self.p2deal.iter().map(show_card).collect();
        write!(fmt, "{} {} {}", self.game.steps, p1, p2)
    }
}

// create a game from two strings
fn read_game(p1: &str, p2: &str) -> GameState {
    let p1deal: Vec<u8> = p1.chars().map(read_card).collect();
    let p2deal: Vec<u8> = p2.chars().map(read_card).collect();
    return make_game(p1deal, p2deal);
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

// fn sum_penalty_cards(vd: &Vec<u8>) -> u8 {
//     return vd.iter().sum();
// }

fn count_penalty_cards(vd: &Vec<u8>) -> u8 {
    return vd.into_iter().filter(|x| **x > 0).count() as u8;
}

fn deal(mut cards: Vec<u8>, swap: bool) -> GameState {
    let mut deal1: Vec<u8> = Vec::with_capacity(64);
    let mut deal2: Vec<u8> = Vec::with_capacity(64);
    deal1.append(&mut cards.split_off(26)); // first 26 cards dealt to p1
    deal2.append(&mut cards); // last 26 cards dealt to p2
    let mut g = make_game(deal1, deal2);
    if swap {
        g.game.swap();
    }
    return g;
}

fn make_game(mut deal1: Vec<u8>, mut deal2: Vec<u8>) -> GameState {
    deal1.reverse();
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

// fn print_internal_state(g : &mut Game) {
//     let p1: String = g.p1hand.iter().map(show_card).collect();
//     let p2: String = g.p2hand.iter().map(show_card).collect();
//     let pot: String = g.pot.iter().map(show_card).collect();
//     // let tenth_sec = time::Duration::from_millis(100);
//     // thread::sleep(tenth_sec);
//     println!("{}                         {}                         {}",p1,pot, p2);
// }

fn wiggle_all(g: &mut GameState) -> Vec<GameState> {
    let mut wiggled_games: Vec<GameState> = Vec::new();
    for c in 1..4 {
        let mut new_g = g.clone();
        let mut wiggled_this_card = wiggle(&mut new_g, c);
        wiggled_games.append(&mut wiggled_this_card);
    }
    return wiggled_games;
}

// swap the penalty_card with every other position that's not this penalty_card
fn wiggle(g: &mut GameState, penalty_card: u8) -> Vec<GameState> {
    let mut cards = g.p1deal.clone();
    cards.append(&mut g.p2deal);
    let consume_cards = cards.clone();
    let maybe_this_penalty_card_index = consume_cards.into_iter().position(|c| c == penalty_card);
    let mut not_this_penalty_card_indices = cards
        .iter()
        .enumerate()
        .filter_map(|(index, &c)| if c == 4 { Some(index) } else { None })
        .collect::<Vec<_>>();
    let mut gvec: Vec<GameState> = Vec::with_capacity(64);
    if let Some(ace_index) = maybe_this_penalty_card_index {
        while let Some(not_ace_index) = not_this_penalty_card_indices.pop() {
            let mut newcards = cards.clone();
            newcards.swap(ace_index, not_ace_index);
            gvec.push(deal(newcards, false));
        }
    }
    return gvec;
}

fn good_games() -> Vec<GameState> {
    let games = [
        // ("K-Q------A-Q----K-JAJJ---Q", "-KK------AA--J---------Q--"),
        // ("-----J-A--QQ-QA---K-------", "KJ-------K-------QAJ--JK-A"),
        // ("---Q-A--AK-A-Q--J--K---A--", "----K------J--Q--K--J---JQ"),
        // ("KA----QA-J----J--K-K---Q--", "-J-Q-----A-JQ-----A---K---"),
        // ("-A---Q--AJ-AJ--K-K--------", "----------Q-K---QA--JJ-QK-"),
        // ("--KQQQ--J--J--A-J-A---K-J-", "------A-----------KKA---Q-"),
        // ("----J--Q--J--A---A-JJK--K-", "---A-----QQ--Q--K-A--K----"),
        // ("---Q--KK----Q---A-J-------", "K-K--Q-J-AA-----J-A--Q--J-"),
        // ("K---------------K-QJ-A-KQJ", "J--AQ---JA-----K----A---Q-"),
        // ("----KKQ--AA--JQ---K-QJ-J--", "-A---K--A--Q----J---------"),
        // ("Q-KJ--A-------Q-J-----KA--", "---J----K---Q-JAK------Q-A"),
        // ("-QJ-------K--K--Q--AKJ---J", "-----AQ----J-A------Q--AK-"),
        // ("-K---K----J--QQ--J-A---AK-", "--AK-Q---J-----AJ--Q------"),
        // ("----------AA-K---QJ------K", "----QKQJ--JJ-A-AQ---K-----"),
        // ("-JA-QQ-----A---J----JK-K--", "------K--A-Q--K---Q-----AJ"),
        // ("--A-J-K--JJ-Q-K-QQ--------", "--------K-KQA-----J-A-A---"),
        // ("-Q--------J-Q------A-JQ---", "J------J--A-----KKA-Q-KKA-"),
        // ("KJA-----------------K---JQ", "---K--Q--Q--JQ-K-JA-A--A--"),
        // ("-QA---KK-----Q---Q---A---A", "K-------J---A-----J-J-QKJ-"),
        // ("--K----Q--J------Q------QK", "---JJ--JA---Q-A---A-KK-A--"),
        // ("J------A---K-A-Q--JA-K----", "J----A------Q-Q-QJ--K---K-"),
        // ("---K--Q--Q-J--------AK--KJ", "---QQ-----A---J--AK-J----A"),
        // ("---AQ----A--J-AK---K--QQ--", "--------KJ----K-A--J---QJ-"),
        // ("--Q--AK-Q---J----J----A---", "-Q-------KJK-----KA-QJ--A-"),
        // ("-----J---A-KAK----Q-KJ----", "--A-KJ--JA----Q----Q-Q----"),
        // ("J---K--Q---QK---K-J-K--J--", "----------JQ-AA---Q--A-A--"),
        // ("K-Q-J----Q--J---K---JA----", "---QA-----Q-A-K--K-A----J-"),
        // ("---KJAQ--JAA-A-----------J", "QQ--------Q---K-----J-K--K"),
        // ("Q--J---Q--Q-A---K-----J--A", "J--K-A-K-----JAQ--K-------"),
        // ("--A-----Q-K---Q----Q----J-", "--AQA--KJ--J-----K--J--K-A"),
        // ("JAA-----Q-Q---------KAQJK-", "-KQ-A----J--------K------J"),
        // ("---KK------Q-J------A-----", "JJ-Q---QA-Q---A-J-K-A-K---"),
        // ("------A-A------J----K--KA-", "-----JJ-QQ-Q--K---Q----AJK"),
        // ("-Q----KK----QA-AQJAK-K----", "-----------------J-J-AJ--Q"),
        // ("---A---A--QJ--J----K--QK-K", "------JQ-------QA--K--A--J"),
        // ("-JK----K-QA----QAK------K-", "------A-Q---J---J-JQ-A----"),
        // ("------J-A-A--K---AQ-J--J--", "--K----QK-QJ---K--QA------"),
        // ("----JK-A--------J--Q---A--", "QQ----A----Q-JKJ--K-A---K-"),
        // ("-------QJ---A---QKKQ--J-AJ", "--------JAQ----A--K----K--"),
        // ("---A-QK-J--Q-------K-A----", "-J-K-QA---Q------J-J---KA-"),
        // ("---AJ-K-Q-----J---AKQ-----", "-QJ----Q--K---K---A---J-A-"),
        // ("-QA-K--QK---Q--J----------", "A-J--J---A------A-K-KJQ---"),
        // ("----Q------JJ---A-K---K--Q", "A-K-----JA-K-JQ---------AQ"),
        // ("--JJ---Q---K-K-K--Q------A", "--K-----A---J--JQA-Q--A---"),
        // ("---KQQJ-A----J---AJ-------", "K------------A--KKQ--AQ--J"),
        // ("-------------Q-Q-A--J-AAKQ", "-K------J-----A--J-QJK-K--"),
        // ("K---A----A---J--J-K-----J-", "----Q--QK-AJQ----K--A-Q---"),
        // ("---A---A--A---Q-J--A-KK---", "--JQ------J--K----JQ--QK--"),
        // ("--AQK------QAJQK--A-------", "--J-----JA--K-----J----QK-"),
        // ("K-J--K------J-------JAA---", "--Q---Q--A-KK---J--Q--A--Q"),
        // ("---A-----------Q--K-K-J-Q-", "-J-----KA-J-Q-Q--K--J-A--A"),
        // ("--J-A-A-JK--K---A--K---K--", "---Q-AQ-Q---Q--J-J--------"),
        // ("--A----Q-K---K--A-JQ-Q-J--", "--K--A--Q--------K---JA-J-"),
        // ("------AJ--Q---AK---K-----Q", "Q---------KKJQ-JA------JA-"),
        // ("JQ--J----K---KJ----A--A---", "J-Q---------AQ-Q-----K--KA"),
        // ("---QK-A--A---A------J--J-K", "--K-A--K-----Q-----Q-QJJ--"),
        // ("--A--A-KK--J--J-K-JQ--J---", "----KA-QQ---Q---------A---"),
        // ("K-----J--J-J-----------A-A", "K-----Q--KKJA-------Q-Q-QA"),
        // ("--A-KJQK-J-K---------A-QK-", "---QA--Q----JJ--A---------"),
        // ("-AK--------K------J---QKQ-", "--Q-J-K---J-------J-QAA--A"),
        // ("K----------A--A------KJ--J", "--KJ-Q---AA-Q-QQJ----K----"),
        // ("K----A----J--JJ----QK-KQ--", "KJ-A--Q----------A------AQ"),
        // ("J---K-------J-J-QA------J-", "-KQ-Q---A----A--QAKK------"),
        // ("Q-K-A--A-----Q----J----JKQ", "--K-------AJ---Q-K-A-J----"),
        // ("-QJK---K--J-Q--J-AK----A-K", "----------A--A----Q---J-Q-"),
        // ("--J--J---------Q---KA-QQ--", "-K-K----AAKJ------A---Q--J"),
        // ("K----AJ-----K---QK-Q--J--A", "-A-------K-J---------QAQ-J"),
        // ("J--Q----Q-J----A----K--AJ-", "A--Q-----A------KQ-KK-J---"),
        // ("-Q-Q------Q------A--A-KJ--", "----AA----J-----KQ-JK--J-K"),
        // ("K----QJ---K---QA-K---JA---", "-J----------Q-J-A-----A-KQ"),
        // ("-K---JJK-Q--------A--KAA-J", "-----A--Q------J-----K-QQ-"),
        // ("---J---A-J-----K-A--JQ----", "J-------QK-KK---A-A---Q-Q-"),
        // ("-----Q--KQ----QA---Q------", "J-KK--J--A-A-J-J---A----K-"),
        // ("J----Q-K--A--J-A--J-Q-----", "-K-A-K--QQ--J------KA-----"),
        // ("---Q---J--JA-QAQ---K------", "K-----K-JQ----J---K----AA-"),
        // ("--------Q-J-----Q--Q--AJ--", "-AK--Q-A--K--AK-J-----KJ--"),
        // ("--JK-----J-A-K----JQ---J--", "----A-QQ-----AK----AQ--K--"),
        // ("--K-----QQ--A-KA--J--K--J-", "----A--Q-J----AK----Q--J--"),
        // ("-J----K-----QQ--A-KA--J--K", "--J-----A--Q-J----AK----Q-"),
        ("J---KK---------J--Q----J--", "---Q-K---A--Q-A-JAA---QK--"),
        ("-----J-AA-------QA------J-", "J-QQ----A-KQ--KK--------JK"),
    ];
    let mut gs: Vec<GameState> = Vec::with_capacity(64);
    for g in games {
        let mut this_g = read_game(g.0, g.1);
        play_one(&mut this_g.game);
        gs.push(this_g);
    }
    return gs;
}
