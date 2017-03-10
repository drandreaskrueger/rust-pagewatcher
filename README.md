# rust-pagewatcher
Hammers webpage - when it has changed, open it in browser.  

(the page on which the SEC publishes whether the ETF gets accepted/refused.)    

Useful if you wait for a page to be updated. This is how it works:

* query a url (default url of the SEC, or 2nd arg given)
* hash the pagebody
* if nothing has happened, sleep (default 60, or 1st arg given) seconds
* if hash has changed (i.e. page has changed), write ALARM, and open webbrowser
* repeat. It never stops, until Ctrl-C.

### TODO
This is my first `rust` program ever. So it might not be good yet. But it works.  

Ideas:

* beep when something has happened
  * terminal beep (tried curses, nothing happened)
  * simply open a webpage which plays a song.

* 

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

webbrowser::open will only work when protocol is given (`http://`)
