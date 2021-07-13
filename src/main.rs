use std::io::*;
use winconsole::console;

fn main() {
    let mut s = stdin();    

    let mut word = String::new();
    for i in 0..5 {
        println!("{:?}", i);
        let sym = console::getch(false).unwrap();        
        print!("\u{0008}");
        if sym == '\t' {
            print!("\r");
            break;
        }
        word.push(sym);        
    }

    
    println!("{}", word);

    println!("Press Enter");
    let mut k = String::new();
    s.read_line(&mut k).unwrap();
}
