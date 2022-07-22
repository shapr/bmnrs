use rand::Rng;
use std::collections::VecDeque;
use std::fmt;
use std::time::Instant;

fn main() {
    let mut cards: VecDeque<u8> = VecDeque::with_capacity(64);
    for _c in 0..4 {
        cards.push_back(4); // four Aces
        cards.push_back(3); // four Kings
        cards.push_back(2); // four Queens
        cards.push_back(1); // four Jacks
    }
    for _c in 0..36 {
        cards.push_back(0); // 36 cards that don't matter
    }

    let mut g8344 = read_game("---AJ--Q---------QAKQJJ-QK", "-----A----KJ-K--------A---");
    play_one(&mut g8344);
    assert_eq!(g8344.steps, 8345); // sanity check, convert to test?
    play_many(cards);
}

fn play_many(cards: VecDeque<u8>) {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut counter = 0;
    let mut highscore = 0;
    let mut best_game = deal(cards.clone(), false);
    let start = Instant::now();
    let mut rng = rand::thread_rng();

    loop {
        let mut newcards = cards.clone(); // [TODO] copy into instead?
        rng.shuffle(&mut newcards.make_contiguous());
        for r in 0..51 {
            let mut c = newcards.clone();
            c.rotate_right(r);
            let mut p1g = deal(c.clone(), false);
            // let p1_penaltycount = count_penalty_cards(&p1g.p1deal);
            // if p1_penaltycount < 4 || p1_penaltycount > 12 {
            //	continue;
            // }
            let p1sum = sum_penalty_cards(&p1g.p1deal);
            if p1sum > 27 || p1sum < 13 {
                // seems to work for the leader board?
                continue;
            }
            play_one(&mut p1g);
            if p1g.steps > highscore {
                highscore = p1g.steps;
                best_game = p1g;
                println!("{}", best_game);
            }
            let mut p2g = deal(c.clone(), false);
            play_one(&mut p2g);
            if p2g.steps > highscore {
                highscore = p2g.steps;
                best_game = p2g;
                println!("{}", best_game);
            }
            counter += 2;
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
    while let Some(card) = g.get_active_mut().pop_front() {
        g.steps += 1; // add one to steps
        if card > 0 {
            // is this next card a penalty card?
            g.penalty = card;
            boring_card(g, card);
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
}

fn pay_tribute(g: &mut Game, card: u8) {
    // penalty is active, and this is not a penalty card
    g.pot.push_back(card); // put this card in the pot
    g.penalty -= 1; // subtract one from penalty
    if g.penalty == 0 {
        // battle is done, add pot to the non-active player's hand
        if g.p1control {
            g.p2hand.append(&mut g.pot);
        } else {
            g.p1hand.append(&mut g.pot);
        }
        g.p1control ^= true; // invert p1control, winner is now active player
    }
}

fn boring_card(g: &mut Game, card: u8) {
    g.pot.push_back(card);
    g.p1control ^= true; // other player is active
}

// the actual game type
pub struct Game {
    p1deal: VecDeque<u8>,
    p2deal: VecDeque<u8>,
    p1hand: VecDeque<u8>,
    p2hand: VecDeque<u8>,
    pot: VecDeque<u8>,
    p1control: bool,
    penalty: u8,
    steps: u16,
}

// how to display the game type
impl fmt::Display for Game {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let p1: String = self.p1deal.iter().map(show_card).collect();
        let p2: String = self.p2deal.iter().map(show_card).collect();
        write!(fmt, "{} {} {}", self.steps, p1, p2)
    }
}

// create a game from two strings
fn read_game(p1: &str, p2: &str) -> Game {
    let p1deal: VecDeque<u8> = p1.chars().map(read_card).collect();
    let p2deal: VecDeque<u8> = p2.chars().map(read_card).collect();
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

fn sum_penalty_cards(vd: &VecDeque<u8>) -> u8 {
    return vd.iter().sum();
}

fn deal(mut cards: VecDeque<u8>, swap: bool) -> Game {
    // assert_eq!(cards.len(),52);
    let mut deal1: VecDeque<u8> = VecDeque::with_capacity(64);
    let mut deal2: VecDeque<u8> = VecDeque::with_capacity(64);
    if swap {
        deal2.append(&mut cards.split_off(26)); // first 26 cards dealt to p2
        deal1.append(&mut cards); // last 26 cards dealt to p1
    } else {
        deal1.append(&mut cards.split_off(26)); // first 26 cards dealt to p1
        deal2.append(&mut cards); // last 26 cards dealt to p2
    }
    // crap out and die if I screwed it up
    // assert_eq!(deal1.len(),26,"swap is {} and deal1 was {} which is WRONG",swap, deal1.len());
    // assert_eq!(deal2.len(),26,"swap is {} and deal2 was {} which is WRONG",swap, deal2.len());
    let g = make_game(deal1, deal2);
    // println!("deal created {}", g);
    return g;
}

fn make_game(deal1: VecDeque<u8>, deal2: VecDeque<u8>) -> Game {
    return Game {
        p1deal: deal1.clone(),
        p2deal: deal2.clone(),
        p1hand: deal1,
        p2hand: deal2,
        pot: VecDeque::with_capacity(64),
        p1control: true,
        penalty: 0,
        steps: 0,
    };
}

impl Game {
    fn get_active_mut(&mut self) -> &mut VecDeque<u8> {
        if self.p1control {
            &mut self.p1hand
        } else {
            &mut self.p2hand
        }
    }
}
