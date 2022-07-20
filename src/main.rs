 use fastrand;
//use rand::Rng;
use std::collections::VecDeque;
use std::fmt;
use std::time::Instant;

impl fmt::Display for Game {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
	let p1: String = self.p1deal.iter().map(show_card).collect();
	let p2: String = self.p2deal.iter().map(show_card).collect();
	write!(fmt, "{} {} {}", self.steps, p1, p2)
    }
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

fn read_game(p1: &str, p2: &str) -> Game {
    let p1deal: VecDeque<u8> = p1.chars().map(read_card).collect();
    let p2deal: VecDeque<u8> = p2.chars().map(read_card).collect();
    return make_game(p1deal, p2deal);
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

#[derive(Clone)]
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

fn main() {
    let mut cards: VecDeque<u8> = VecDeque::with_capacity(64);
    for _c in 0..4 {
	cards.push_back(4);
	cards.push_back(3);
	cards.push_back(2);
	cards.push_back(1);
    }
    for _c in 0..36 {
	cards.push_back(0);
    }

    let mut g3895 = read_game("----K-JA---K---KQ--J-----K-", "-----A--J-J--Q--A---A-Q-Q");
    // assert_eq!(play_some_pieces(g3895).steps, 3895);
    play_some_pieces(&mut g3895);
    assert_eq!(g3895.steps, 3895);
    play_some_many(cards);
}

fn play_some_many(cards: VecDeque<u8>) {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut counter = 0;
    let mut highscore = 0;
    let mut best_game = deal(cards.clone(), false);
    let start = Instant::now();
    // let mut rng = rand::thread_rng();

    loop {
	let mut newcards = cards.clone(); // [TODO] copy into instead?
	// rng.shuffle(&mut newcards.make_contiguous());
	fastrand::shuffle(&mut newcards.make_contiguous());
	let mut game = deal(newcards.clone(), false);
	// let this_game = play_some_pieces(game);
	play_some_pieces(&mut game);
	if game.steps > highscore {
	    highscore = game.steps;
	    best_game = game;
	    println!("{}", best_game);
	}
	let mut game2 = deal(newcards, true);
	play_some_pieces(&mut game2);
	counter += 2;
	if game2.steps > highscore {
	    highscore = game2.steps;
	    best_game = game2;
	    println!("p2 {}", best_game);
	}
	if counter % 30000000 == 0 {
	    println!(
		"{} best so far at {} games per second in play_some_many {}",
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


impl Game {
    fn get_active_mut(&mut self) -> &mut VecDeque<u8> {
	if self.p1control {
	    &mut self.p1hand
	} else {
	    &mut self.p2hand
	}
    }
}

fn play_some_pieces(g: &mut Game){
    while let Some(card) = g.get_active_mut().pop_front() {
	g.steps += 1; // add one to steps
	if card > 0 {
	    // is this next card a penalty card?
	    penalty_card(g, card);
	} else {
	    // it's not a penalty card, but we still have tribute to pay
	    if g.penalty > 0 {
		no_penalty_card(g, card);
	    } else {
		// nothing going on, play a card into the pot
		boring_card(g, card);
	    }
	}
    }
}

fn penalty_card(g: &mut Game, card: u8) {
    g.p1control ^= true;
    g.penalty = card; // set the new penalty value
    g.pot.push_back(card); // add this card to the front of the pot
}

fn no_penalty_card(g: &mut Game, card: u8) {
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
    g.p1control ^= true;
}

fn deal(mut cards: VecDeque<u8>, swap: bool) -> Game {
    let mut deal1: VecDeque<u8> = VecDeque::with_capacity(64);
    let mut deal2: VecDeque<u8> = VecDeque::with_capacity(64);
    if swap {
	deal2.append(&mut cards.split_off(25)); // first 26 cards dealt to p1
	deal1.append(&mut cards); // last 26 cards dealt to p2
    } else {
	deal1.append(&mut cards.split_off(25)); // first 26 cards dealt to p1
	deal2.append(&mut cards); // last 26 cards dealt to p2
    }
    return make_game(deal1, deal2);
}

fn make_game(deal1: VecDeque<u8>, deal2: VecDeque<u8>) -> Game {
    let game = Game {
	p1deal: deal1.clone(),
	p2deal: deal2.clone(),
	p1hand: deal1,
	p2hand: deal2,
	pot: VecDeque::with_capacity(64),
	p1control: true,
	penalty: 0,
	steps: 0,
    };
    return game;
}
