extern crate tbr;

use std::thread;
use std::io::{self, Read, Write};

use tbr::ThreadedBufReader;
use tbr::Reader;

fn main() {

	let (r, w) = ThreadedBufReader::with_capacity(io::stdin(), 1024);
	
	let wt = thread::spawn(move || {
		loop {
			//thread::sleep_ms(0);
			w.fill_buf_local().unwrap();
		}
	});
	
	let rt = thread::spawn(move || {
		loop {
			{
				let echo = &r.read();
				if echo.len() > 0 {
					io::stdout().write_all(echo).unwrap();
					r.consume_local(echo.len());
				} else {
					r.compact();
				}
			}
			//thread::sleep_ms(100);
		}
	});

	let _res = wt.join();
	let _res = rt.join();
}
