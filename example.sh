export cnt=0; while [ true ]; do ((cnt++)); printf "Hello %10.10d\n" $cnt; done | pv | cargo run --release 
