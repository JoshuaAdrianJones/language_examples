// get user input in Rust
use std::io; 
use std::io::Write; // bring flush() into scope

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}//hack?

fn main() -> io::Result<()>{
    println!("Hello Jibby, from RUST!");
    println!("Lets get some user input.");
    println!("What is the quote?");
    io::stdout().flush().unwrap();
    let mut quote = String::new();
    io::stdin().read_line(&mut quote).expect("Error getting quote");
    
    println!("Who said the quote?");
    io::stdout().flush().unwrap();
    let mut quote_author = String::new();
    io::stdin().read_line(&mut quote_author).expect("Error getting quote_author");

    let fmt_quote = strip_trailing_newline(&quote);
    let fmt_quote_author = strip_trailing_newline(&quote_author);
    
    // without applying strip_trailing_newline() the final '.' is 
    // put on a new line
    println!("The quote was \"{fmt_quote}\".");
    println!("and it was said by {fmt_quote_author}.");
    Ok(())
}

