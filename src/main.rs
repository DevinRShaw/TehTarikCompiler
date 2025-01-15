//The reading of file, command args and lex call are same as example 

// used to get the commandline arguments from the commandline.
use std::env;
// used to interact with the file system
use std::fs;

fn main() {

    // Let us get commandline arguments and store them in a Vec<String>
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Please provide an input file through the commandline arguments for the lex.");
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


    // print out the lex tokens parsed.

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

#[derive(Debug, Clone, PartialEq)] // Rust generates these traits for use by us in development
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

//handles keywords 
fn create_identifier(code: &str) -> Token {
  match code {
      "func" => Token::Func,
      "return" => Token::Return,
      "int" => Token::Int,
      "print" => Token::Print,
      "else" => Token::Else,
      "break" => Token::Break,
      "continue" => Token::Continue,
      "while" => Token::While,
      "if" => Token::If,
      "read" => Token::Read,
      _ => Token::Ident(String::from(code)), // For non-keyword identifiers
  }
}



// Extend this lex to work for all tokens above 
fn lex(code: &str) -> Result<Vec<Token>, String> {
    let bytes = code.as_bytes();
    let mut tokens: Vec<Token> = vec![];
  
    let mut i = 0;
    while i < bytes.len() {
      let c = bytes[i] as char;

  
      match c {

      //text handling for keywords and identifiers 
      // strategy is to read in the 
      'a'..='z' | 'A'..='Z' => {
        // Handle alphabetic characters
        let start = i;
        i += 1;
        while i < bytes.len() {
          let character = bytes[i] as char;
          if (character >= 'a' && character <= 'z') || (character >= 'A' && character <= 'Z') || (character >= '0' && character <= '9'){
              i += 1;
          } else {
              break; // Exit the loop if it's not an alphabetical character
          }
        }
        let end = i;
        //aab3
        let string_token = &code[start..end];

        tokens.push(create_identifier(string_token));
      }


    
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

        // Check if the next character is alphabetic, which would indicate an invalid variable
        if i < bytes.len() && (bytes[i] as char).is_alphabetic() {
          return Err(format!("Invalid variable name starting with a number at: {}", &code[start..]));
        }

        let string_token = &code[start..end];
        let number_value = string_token.parse::<i32>().unwrap();
        let token = Token::Num(number_value);
        tokens.push(token);
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
            i += 2; 
            //go to the next scan 
            continue;
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
            i += 2; 
            continue;
          }
        }
        //avoid else with continue, less code that way 
        tokens.push(Token::Greater);
        i += 1; 
      }


      '!' => {
        if i + 1 < bytes.len(){ //if able, check next char for = to make <= token, longer token prefered 
          if bytes[i+1] == b'=' {
            tokens.push(Token::NotEqual);
            i += 2; 
            continue;
          }
        }

        //need to have error handling, ! will not accept 
        return Err(format!("Unrecognized symbol '{}'", c));
      }

      //change this to match the logic of = or == symbols 
      '=' => {
        if i + 1 < bytes.len(){ //if able, check next char for = to make <= token, longer token prefered 
          if bytes[i+1] == b'=' {
            tokens.push(Token::Equality);
            i += 2; 
            continue;
          }
        }

        //avoid else with continue, less code that way 
        tokens.push(Token::Assign);
        i += 1;
      }

      ' ' | '\n' => {
        i += 1; // Skip whitespace and newlines
      }

      '+' => {
        tokens.push(Token::Plus);
        i += 1;
      }
      '-' => {
        tokens.push(Token::Subtract);
        i += 1;
      }
      '*' => {
        tokens.push(Token::Multiply);
        i += 1;
      }
      '/' => {
        tokens.push(Token::Divide);
        i += 1;
      }
      '%' => {
        tokens.push(Token::Modulus);
        i += 1;
      }
    
      '(' => {
        tokens.push(Token::LeftParen);
        i += 1;
      }
      ')' => {
        tokens.push(Token::RightParen);
        i += 1;
      }
      '{' => {
        tokens.push(Token::LeftCurly);
        i += 1;
      }
      '}' => {
        tokens.push(Token::RightCurly);
        i += 1;
      }
      '[' => {
        tokens.push(Token::LeftBracket);
        i += 1;
      }
      ']' => {
        tokens.push(Token::RightBracket);
        i += 1;
      }
      ',' => {
        tokens.push(Token::Comma);
        i += 1;
      }
      ';' => {
        tokens.push(Token::Semicolon);
        i += 1;
      }
      _ => {
        return Err(format!("Unrecognized symbol '{}'", c));
      }
    }
  }
    
    tokens.push(Token::End);
    return Ok(tokens); //part of the error handling of the result aspect of rust 
  }
 



  #[cfg(test)]
  mod tests {
      use super::*;
  
      #[test]
      fn test_valid_identifier() {
          let input = "abc def ghi";
          let expected_tokens = vec![
              Token::Ident("abc".to_string()),
              Token::Ident("def".to_string()),
              Token::Ident("ghi".to_string()),
              Token::End,
          ];
  
          let result = lex(input);
          assert_eq!(result, Ok(expected_tokens));
      }
  
      #[test]
      fn test_invalid_identifier() {
          let input = "1abc";
          
          let result = lex(input);
          assert_eq!(result, Err("Invalid variable name starting with a number at: 1abc".to_string()));
      }
  
      #[test]
      fn test_number_token() {
          let input = "123 456 789";
          let expected_tokens = vec![
              Token::Num(123),
              Token::Num(456),
              Token::Num(789),
              Token::End,
          ];
  
          let result = lex(input);
          assert_eq!(result, Ok(expected_tokens));
      }
  
      #[test]
      fn test_comment_ignoring() {
          let input = "var x # this is a comment\nvar y";
          let expected_tokens = vec![
              Token::Ident("var".to_string()),
              Token::Ident("x".to_string()),
              Token::Ident("var".to_string()),
              Token::Ident("y".to_string()),
              Token::End,
          ];
  
          let result = lex(input);
          assert_eq!(result, Ok(expected_tokens));
      }
  
      #[test]
      fn test_operator_tokens() {
          let input = "+ - * /";
          let expected_tokens = vec![
              Token::Plus,
              Token::Subtract,
              Token::Multiply,
              Token::Divide,
              Token::End,
          ];
  
          let result = lex(input);
          assert_eq!(result, Ok(expected_tokens));
      }
  
      #[test]
      fn test_symbol_tokens() {
          let input = "( ) { } [ ] , ;";
          let expected_tokens = vec![
              Token::LeftParen,
              Token::RightParen,
              Token::LeftCurly,
              Token::RightCurly,
              Token::LeftBracket,
              Token::RightBracket,
              Token::Comma,
              Token::Semicolon,
              Token::End,
          ];
  
          let result = lex(input);
          assert_eq!(result, Ok(expected_tokens));
      }
  
      #[test]
      fn test_multi_char_tokens() {
          let input = "< <= > >= == =";
          let expected_tokens = vec![
              Token::Less,
              Token::LessEqual,
              Token::Greater,
              Token::GreaterEqual,
              Token::Equality,
              Token::Assign,
              Token::End,
          ];
  
          let result = lex(input);
          assert_eq!(result, Ok(expected_tokens));
      }
  
      #[test]
      fn test_unrecognized_symbol() {
          let input = "&";
          
          let result = lex(input);
          assert_eq!(result, Err("Unrecognized symbol '&'".to_string()));
      }
  
      #[test]
      fn test_empty_input() {
          let input = "";
          let expected_tokens = vec![Token::End];
          
          let result = lex(input);
          assert_eq!(result, Ok(expected_tokens));
      }
  
      #[test]
      fn test_invalid_token_with_digit() {
          let input = "123abc";
          
          let result = lex(input);
          assert_eq!(result, Err("Invalid variable name starting with a number at: 123abc".to_string()));
      }
  }
  