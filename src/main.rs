use rand::Rng;
// use std::collections::BTreeMap;
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
    check_all();
    play_many(cards);
}

fn play_many(cards: Vec<u8>) {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut counter = 0;
    let mut highscore = 0;
    let mut best_game = deal(cards.clone(), false);
    let start = Instant::now();
    let mut rng = rand::thread_rng();
    // let mut btm: BTreeMap<(Vec<u8>, Vec<u8>), u16> = BTreeMap::new();

    loop {
        let mut newcards = cards.clone(); // [TODO] copy into instead?
        rng.shuffle(&mut newcards);
        for r in 0..51 {
            let mut c = newcards.clone();
            c.rotate_right(r);
            let p1d = deal(c.clone(), false);
            // if btm.contains_key(&(p1d.p1deal.clone(), p1d.p2deal.clone())) {
            //	continue;
            // }
            let mut p1g = p1d;

            let p1_pen_card_count = count_penalty_cards(&p1g.p1deal);
            if p1_pen_card_count > 11 || p1_pen_card_count < 5 {
                continue;
            }
            play_one(&mut p1g.game);
            // play_match(&mut p1g.game);
            // btm.insert((p1g.p1deal.clone(), p1g.p2deal.clone()), p1g.game.steps);
            if p1g.game.steps > highscore {
                highscore = p1g.game.steps;
                best_game = p1g;
                println!("{}", best_game);
            }
            let mut p2g = deal(c.clone(), false);
            play_one(&mut p2g.game);
            // play_match(&mut p2g.game);
            if p2g.game.steps > highscore {
                highscore = p2g.game.steps;
                best_game = p2g;
                println!("{}", best_game);
            }
            let p1gr_deal = c.clone().into_iter().rev().collect();
            let mut p1gr = deal(p1gr_deal, false);
            play_one(&mut p1gr.game);
            // play_match(&mut p1gr.game);
            if p1gr.game.steps > highscore {
                highscore = p1gr.game.steps;
                best_game = p1gr;
                println!("{}", best_game);
            }
            let p2gr_deal = c.clone().into_iter().rev().collect();
            let mut p2gr = deal(p2gr_deal, true);
            play_one(&mut p2gr.game);
            // play_match(&mut p2gr.game);
            if p2gr.game.steps > highscore {
                highscore = p2gr.game.steps;
                best_game = p2gr;
                println!("{}", best_game);
            }
            counter += 4;
        }
        if counter % 1000000 == 0 {
            println!(
                "{} best so far at {} games per second in play_many {}",
                best_game,
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

fn play_one(g: &mut Game) {
    while let Some(card) = g.p1hand.pop() {
        if card > 0 {
            // is this next card a penalty card?
            g.penalty = card;
            boring_card(g, card);
            // print_internal_state(g);
        } else {
            // it's not a penalty card, but we still have tribute to pay
            if g.penalty > 0 {
                pay_tribute(g, card);
            // print_internal_state(g);
            } else {
                // nothing going on, play a card into the pot
                boring_card(g, card);
                // print_internal_state(g);
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

fn boring_card(g: &mut Game, card: u8) {
    g.pot.push(card);
    g.swap(); // swap hands, other player is active
}

// the actual game type
// #[derive(PartialEq, PartialOrd, Ord, Eq)]
pub struct GameState {
    // initial_deal : (Vec<u8>,Vec<u8>),
    p1deal: Vec<u8>,
    p2deal: Vec<u8>,
    game: Game,
}
// #[derive(Decode, Encode, PartialEq, PartialOrd, Ord, Eq)]
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

// fn print_internal_state(g: &mut Game) {
//     let p1: String = g.p1hand.iter().map(show_card).collect();
//     let p2: String = g.p2hand.iter().map(show_card).collect();
//     let pot: String = g.pot.iter().map(show_card).collect();
//     // let tenth_sec = time::Duration::from_millis(100);
//     // thread::sleep(tenth_sec);
//     println!(
//	"{}                         {}                         {}",
//	p1, pot, p2
//     );
// }

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
