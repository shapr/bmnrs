// use fastrand;
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
    // let mut g8344 = read_deck("---AJ--Q---------QAKQJJ-QK-----A----KJ-K--------A---");
    // let mut g8344 = read_game("---AJ--Q---------QAKQJJ-QK", "-----A----KJ-K--------A---");

    // play_one(&mut g8344);
    // println!("{} from g8344", g8344);

    // let top_games = [
    //	("--A-----Q-K---Q----Q----J-","--AQA--KJ--J-----K--J--K-A"),
    //	("------A-A------J----K--KA-","-----JJ-QQ-Q--K---Q----AJK"),
    //	("-Q----KK----QA-AQJAK-K----","-----------------J-J-AJ--Q"),
    //	("---A---A--QJ--J----K--QK-K","------JQ-------QA--K--A--J"),
    //	("-JK----K-QA----QAK------K-","------A-Q---J---J-JQ-A----"),
    //	("-------QJ---A---QKKQ--J-AJ","--------JAQ----A--K----K--"),
    //	("-------------Q-Q-A--J-AAKQ","-K------J-----A--J-QJK-K--"),
    //	("--AQK------QAJQK--A-------","--J-----JA--K-----J----QK-"),
    //	("---A-----------Q--K-K-J-Q-","-J-----KA-J-Q-Q--K--J-A--A"),
    //	("--J-A-A-JK--K---A--K---K--","---Q-AQ-Q---Q--J-J--------"),
    //	("------AJ--Q---AK---K-----Q","Q---------KKJQ-JA------JA-"),
    //	("--A--A-KK--J--J-K-JQ--J---","----KA-QQ---Q---------A---"),
    //	("K-----J--J-J-----------A-A","K-----Q--KKJA-------Q-Q-QA"),
    //	("-AK--------K------J---QKQ-","--Q-J-K---J-------J-QAA--A"),
    //	("K----------A--A------KJ--J","--KJ-Q---AA-Q-QQJ----K----"),
    //	("K----A----J--JJ----QK-KQ--","KJ-A--Q----------A------AQ"),
    //	("J---K-------J-J-QA------J-","-KQ-Q---A----A--QAKK------"),
    //	("-QJK---K--J-Q--J-AK----A-K","----------A--A----Q---J-Q-"),
    //	("K----AJ-----K---QK-Q--J--A","-A-------K-J---------QAQ-J"),
    //	("---Q---J--JA-QAQ---K------","K-----K-JQ----J---K----AA-"),
    //	("--------Q-J-----Q--Q--AJ--","-AK--Q-A--K--AK-J-----KJ--"),
    //	("--JK-----J-A-K----JQ---J--","----A-QQ-----AK----AQ--K--"),
    //	("J---KK---------J--Q----J--","---Q-K---A--Q-A-JAA---QK--"),
    //	("-----J-AA-------QA------J-","J-QQ----A-KQ--KK--------JK"),

    //	("---JQ---K-A----A-J-K---QK-","-J-----------AJQA----K---Q"),
    //	("-J--KA----A-Q--Q-A----KJ--","A------QKJ--Q-------KJ----"),
    //	("K-KK----K-A-----JAA--Q--J-","---Q---Q-J-----J------AQ--"),
    //	("----Q------A--K--A-A--QJK-","-Q--J--J---QK---K----JA---"),
    //	("-J-------Q------A--A--QKK-","-A-Q--J--J---Q--AJ-K---K--"),
    //	("--A-Q--J--J---Q--AJ-K---K-","-J-------Q------A--A--QKK-"),
    //	("---AK-Q--J----J--QKJ-Q----","------JK-----A--K--Q---AA-"),
    //	("A-AQ-----Q--K--AQ-------JJ","-J-A-KKJ--K-----------Q---"),
    //	("-AJ--QK--K----Q--J-A-KKJ--","---------JQ----------A-AQ-"),
    //	("-J------Q------AAA-----QQ-","K----JA-----------KQ-K-JJK"),
    //	("----K---A--Q-A--JJA------J","-----KK---------A-JK-Q-Q-Q"),
    //	("---AJ--Q---------QAKQJJ-QK","-----A----KJ-K--------A---")
    // ];
    // for g in top_games {
    //	let (p1,p2,r) = ratio_game(&read_game(g.0, g.1));
    //	println!("{:?}",(p1,p2,r));
    //	// games_to_check.push(read_game(g.0, g.1))
    // };
    // let mut g5791 = read_game("---JQ---K-A----A-J-K---QK-","-J-----------AJQA----K---Q");
    // play_for_reading(&mut g5791);
    let mut g8344 = read_game("---AJ--Q---------QAKQJJ-QK","-----A----KJ-K--------A---");
    play_one(&mut g8344);
    assert_eq!(g8344.steps, 8345);
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
	// fastrand::shuffle(&mut newcards.make_contiguous());
	let mut gvec : Vec<Game> = Vec::with_capacity(64);
	for r in 0 .. 51 {
	    let mut c = newcards.clone();
	    c.rotate_right(r);
	    let p1g = deal(c.clone(),false);
	    let p1sum = sum_ref_vd(&p1g.p1deal);
	    if p1sum > 27 || p1sum < 13 { // seems to work for the leader board?
		continue;
	    }
	    gvec.push(p1g);
	    gvec.push(deal(c, true));
	}
	for mut g in gvec {
	    play_one(&mut g);
	    counter += 1;
	    if g.steps > highscore {
		highscore = g.steps;
		best_game = g;
		println!("{}", best_game);
	    }
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
#[derive(Clone,Ord, PartialEq, Eq, PartialOrd)]
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

impl fmt::Display for Game { // how to display the game type
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
	let p1: String = self.p1deal.iter().map(show_card).collect();
	let p2: String = self.p2deal.iter().map(show_card).collect();
	write!(fmt, "{} {} {}", self.steps, p1, p2)
    }
}

fn read_game(p1: &str, p2: &str) -> Game { // create a game from two strings
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

fn sum_ref_vd(vd : &VecDeque<u8>) -> u8 {
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

impl Game {
    fn get_active_mut(&mut self) -> &mut VecDeque<u8> {
	if self.p1control {
	    &mut self.p1hand
	} else {
	    &mut self.p2hand
	}
    }
}

// fn play_for_reading(g: &mut Game) {
//     while let Some(card) = g.get_active_mut().pop_front() {
//	g.steps += 1; // add one to steps
//	if card > 0 {
//	    // is this next card a penalty card?
//	    g.penalty = card;
//	    boring_card(g, card);
//	    print_internal_state(g);
//	} else {
//	    // it's not a penalty card, but we still have tribute to pay
//	    if g.penalty > 0 {
//		pay_tribute(g, card);
//		print_internal_state(g);
//	    } else {
//		// nothing going on, play a card into the pot
//		boring_card(g, card);
//		print_internal_state(g);
//	    }
//	}
//     }
// }

// fn print_internal_state(g : &mut Game) {
//     let p1: String = g.p1hand.iter().map(show_card).collect();
//     let p2: String = g.p2hand.iter().map(show_card).collect();
//     let pot: String = g.pot.iter().map(show_card).collect();
//     // let tenth_sec = time::Duration::from_millis(100);
//     // thread::sleep(tenth_sec);
//     println!("{}                         {}                         {}",p1,pot, p2);
// }


// fn ratio_game(g: &Game) -> (u8,u8,f64) {
//     let sp1 = sum_ref_vd(&g.p1deal);
//     let sp2 = sum_ref_vd(&g.p2deal);
//     let ratio = sp1 as f64 / sp2 as f64;
//     return (sp1, sp2, ratio);
// }
