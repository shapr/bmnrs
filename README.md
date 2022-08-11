# bmnrs
fast beggar my neighbor in rust

# GO FAST?
Yes, run this with `RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C strip=symbols" cargo run --release`

# HOW FAST?
For my six core Xeon laptop, running one process plays about 475,000 games per second.

With twelve open terminals each running the above command once in each terminal, roughly 250,000 games per second from each process, giving a total of about 3 million games played per second.
