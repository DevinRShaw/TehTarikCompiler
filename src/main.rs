//The reading of file, command args and lex call are same as example 

// used to get the commandline arguments from the commandline.
use std::env;
// used to interact with the file system
use std::fs;

fn main() {

    // Let us get commandline arguments and store them in a Vec<String>
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file through the commandline arguments for the lexer.");
        return;
    }

    if args.len() > 2 {
        println!("Too many commandline arguments.");
        return;
    }

    // read the entire file contents, storing them inside 'code' as a string.
    let filename = &args[1];
    let code = match fs::read_to_string(filename) { //this is a rust style code block, keep that in mind 
      Err(error) => {
          println!("**Error. File \"{}\": {}", filename, error);
          return;
      }

      Ok(code) => {
          code
      } 
    };

    let tokens = match lex(&code) {
      Err(error_message) => {
          println!("**Error**");
          println!("----------------------");
          println!("{}", error_message);
          println!("----------------------");
          return;
      }

      Ok(data) => data,
      
      };


    // print out the lexer tokens parsed.

    println!("----------------------");
    println!("Finished Lexing the file {}", filename);
    println!("File Contents:");
    println!("{code}");
    println!("Here are the Results:");
    println!("----------------------");
    for t in &tokens {
      println!("{:?}", t);
    }

}




//have to expand the Token enum to include assignment tokens too 

#[derive(Debug, Clone)] // Rust generates these traits for use by us in development
enum Token {
    Plus,             // +
    Subtract,         // -
    Multiply,         // *
    Divide,           // /
    Modulus,          // %
    Assign,           // =
    Less,             // <
    LessEqual,        // <=
    Greater,          // >
    GreaterEqual,     // >=
    Equality,         // ==
    NotEqual,         // !=
    Num(i32),         // 10311517 (numbers)
    Ident(String),    // variable_name
    If,               // if
    While,            // while
    Read,             // read
    Func,             // func
    Return,           // return
    Int,              // int
    Print,            // print
    Else,             // else
    Break,            // break
    Continue,         // continue
    LeftParen,        // (
    RightParen,       // )
    LeftCurly,        // {
    RightCurly,       // }
    LeftBracket,      // [
    RightBracket,     // ]
    Comma,            // ,
    Semicolon,        // ;
    End,              // Marks the end of the list of tokens
}



// Extend this lexer to work for all tokens above 
fn lex(code: &str) -> Result<Vec<Token>, String> {
    let bytes = code.as_bytes();
    let mut tokens: Vec<Token> = vec![];
  
    let mut i = 0;
    while i < bytes.len() {
      let c = bytes[i] as char;
  
      match c {
    
      //does this need to account for invalid variable namings? original give errors for (digit)(char)* which is invalid in this language? 
      '0'..='9' => {
        let start = i;
        i += 1;
        while i < bytes.len() {
          let digit = bytes[i] as char;
          if digit >= '0' && digit <= '9' {
            i += 1;
          } else {
            break; //this handles invalid namings alrady I believe 
          }
        }
        let end = i;
        let string_token = &code[start..end];
        let number_value = string_token.parse::<i32>().unwrap();
        let token = Token::Num(number_value);
        tokens.push(token);
      }
  
      '+' => {
        tokens.push(Token::Plus);
        i += 1;
      }
  
      ' ' | '\n' => {
        i += 1;
      }
      
      //comments do not need to become tokens as they carry zero meaning to the program, we can just skip that part 
        //Q: how to handle the idea of two chars for one symbol here? 
        //A: b'\n' is a byte literal 
      '#' => {
        while bytes[i] != b'\n'{
          i += 1; 
        }
      }

      //example from lecture guided this one (< and <= are possible tokens)
      //lookahead, if valid then go to second accept state, else accept original 

      '<' => {
        if i + 1 < bytes.len(){ //if able, check next char for = to make <= token, longer token prefered 
          if bytes[i+1] == b'=' {
            tokens.push(Token::LessEqual);
            i += 1; 
            break;
          }
        }
        //avoid else with breaks, less code that way 
        tokens.push(Token::Less);
        i += 1; 
      }


      //very similar logic to above code 

      '>' => {
        if i + 1 < bytes.len(){ //if able, check next char for = to make <= token, longer token prefered 
          if bytes[i+1] == b'=' {
            tokens.push(Token::GreaterEqual);
            i += 1; 
            break;
          }
        }
        //avoid else with breaks, less code that way 
        tokens.push(Token::Greater);
        i += 1; 
      }
  
      _ => {
        return Err(format!("Unrecognized symbol '{}'", c));
      }
  
      }
    }
  
    tokens.push(Token::End);
    return Ok(tokens);
  }