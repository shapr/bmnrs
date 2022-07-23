# bmnrs
fast beggar my neighbor in rust

# GO FAST?
Yes, run this with `RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C strip=symbols" cargo run --release`

# HOW FAST?
For my six core Xeon laptop, I open twelve terminals and run the above command once in each terminal.
I get about 190,000 games per second from each process, giving a total of 2.3 million games played per second.
