

	
extern crate curl;
use curl::http;
extern crate webbrowser;

use std::env;
use std::hash::{Hash, Hasher};
use std::{thread, time};
use std::io::{self, Write};

// beep() doesn't do anything:
// extern crate pancurses;
// use pancurses::beep;     // sudo apt install libncurses5-dev


fn get_hash_of_page_at_url(url: &str) -> u64 {
	// queries the url, then hashes the body of the page, and returns the hash.
	
	// thanks to
	// https://github.com/hjr3/rust-get-data-from-url/blob/master/src/main.rs
	
	let resp = http::handle()
	     .get(url)
	     .exec()
	     .unwrap_or_else(|e| {
	         panic!("Failed to get {}; error is {}", url, e);
	     });
	
    if resp.get_code() != 200 {
        println!("HTTP response code {} is not 200.", resp.get_code());
        return 0;
    }

    let body = std::str::from_utf8(resp.get_body()).unwrap_or_else(|e| {
        panic!("Failed to parse response from {}; error is {}", url, e);
    });
    
    let mut s = std::collections::hash_map::DefaultHasher::new();
    body.hash(&mut s);
    let pagehash = s.finish();
    
    return pagehash;
}

fn query_loop_return_when_different(url: &str, pause_seconds: u64) -> (u64, u64) {
	// queries the url, stores the hash
	// sleeps
	// then prints a '.' and redoes it
	// returns only if hash is different.
	
	let pagehash = get_hash_of_page_at_url(&url);
    print!("page hash: {} of pagebody at url {}", pagehash, url);
	println!(" retesting every {} seconds:", pause_seconds);
	
	let pause = time::Duration::from_secs(pause_seconds);
	let mut count = 0;
	let linelength = 60;
	let mut pagehash_new ;
	
	loop {
		thread::sleep(pause);
		pagehash_new = get_hash_of_page_at_url(&url);
		if pagehash_new != pagehash { break; }
		
		print!(".");  io::stdout().flush().unwrap();
		
		count = count + 1;
		if count % linelength == 0 {print!("\n");}
	}
	
	return (pagehash, pagehash_new);
}

fn main() {
	
	// todo: put this at top of file, or into config file:
	let default_pause = "60".to_string();
	let default_url = "https://www.sec.gov/rules/sro/batsbzx.htm".to_string();
	
	
	let pause = env::args().nth(1).unwrap_or(default_pause).parse::<u64>().unwrap();
	let url = env::args().nth(2).unwrap_or(default_url);
	
	loop {
		let (ph1, ph2) = query_loop_return_when_different(&url, pause);
		print!("ALARM: hash of pagebody has changed from {} to {} !", ph1, ph2);
		
		if webbrowser::open(&url).is_ok() {
		    println!(" Opened webbrowser at '{}'.", url);
		}
	}
}