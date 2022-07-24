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
    let mut highscore = 0;
    let mut best_game = deal(cards.clone(), false);
    let start = Instant::now();
    let mut rng = rand::thread_rng();

    loop {
	let mut newcards = cards.clone(); // [TODO] copy into instead?
	rng.shuffle(&mut newcards);
	for r in 0..51 {
	    let mut c = newcards.clone();
	    c.rotate_right(r);
	    let mut p1g = deal(c.clone(), false);
	    let p1sum = sum_penalty_cards(&p1g.p1deal);
	    if p1sum > 28 || p1sum < 12 {
		// seems to work for the leader board?
		continue;
	    }
	    play_one(&mut p1g.game);
	    if p1g.game.steps > highscore {
		highscore = p1g.game.steps;
		best_game = p1g;
		println!("{}", best_game);
	    }
	    let mut p2g = deal(c.clone(), false);
	    play_one(&mut p2g.game);
	    if p2g.game.steps > highscore {
		highscore = p2g.game.steps;
		best_game = p2g;
		println!("{}", best_game);
	    }
	    let p1gr_deal = c.clone().into_iter().rev().collect();
	    let mut p1gr = deal(p1gr_deal, false);
	    play_one(&mut p1gr.game);
	    if p1gr.game.steps > highscore {
		highscore = p1gr.game.steps;
		best_game = p1gr;
		println!("{}", best_game);
	    }
	    let p2gr_deal = c.clone().into_iter().rev().collect();
	    let mut p2gr = deal(p2gr_deal, true);
	    play_one(&mut p2gr.game);
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
	// if counter > 10000000000 {
	if counter > 2000000 {
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
	g.p2hand.reverse(); // go left to right
	g.p2hand.append(&mut g.pot); // add the pot
	g.p2hand.reverse(); // back to right to left
	g.swap(); // swap hands, winner is now active player
    }
}

fn boring_card(g: &mut Game, card: u8) {
    g.pot.push(card);
    g.swap(); // swap hands, other player is active
}

// the actual game type
pub struct GameState {
    p1deal: Vec<u8>,
    p2deal: Vec<u8>,
    game: Game,
}
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

fn sum_penalty_cards(vd: &Vec<u8>) -> u8 {
    return vd.iter().sum();
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
