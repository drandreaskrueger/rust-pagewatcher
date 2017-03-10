# rust-pagewatcher
Hammers a webpage - as soon as it has changed, open it in browser.  

*(The SEC might publish today whether the Bitcoin ETF gets accepted or refused.)*    

Useful if you wait for a page to be updated. 

### This is how it works:

* query a url (default url of the SEC, or 2nd arg given)
* hash the pagebody
* if nothing has happened, sleep (default 60, or 1st arg given) seconds
* if hash has changed (i.e. page has changed), write ALARM, and open webbrowser
* repeat. It never stops, until Ctrl-C.


### quickstart:

```
git clone https://github.com/drandreaskrueger/rust-pagewatcher.git
cd rust-pagewatcher
cargo build --release

target/release/pagewatcher
```

### arguments

* sleep only 10 seconds: `target/release/pagewatcher 10` 
* open a different url: `target/release/pagewatcher 10 http://www.andreaskrueger.de/test/test.html`

N.B.: `webbrowser::open` will only work when the protocol is given (`http://`)

### TODO
This is my first `rust` program ever. So it might not be good yet. But it works.  

Ideas:

* beep when something has happened
  * terminal beep (tried curses, nothing happened)
  * simply open a webpage which plays a song.


### donationware
If you want to, you can reward me by sending me cryptocurrencies. Contact me, and tell me your favorite coin, and I generate an address for that.
 
