# IR Parser [lib]

This is the irparser crate. It contains a lexer and parser that will parse a file
of valid citadel high-level ir into a stream of IR nodes from the frontend crate.

## Using irparser

```rust
use std::{io, fs, str};
use citadel_irparser::{IRLexer, IRParser};

// It is possible that we get an
// error, if the file at the specified
// path does not exist, so we should
// return a Result just in case.
fn main() -> io::Result<()> {
    let path = "path/to/your/file.chir";
    // We use the question mark operator
    // to return an Error value if the
    // file was not found.
    // If you do not want to deal with
    // the result, you can just call
    // `.unwrap()` instead and remove the
    // return type as well as the OK(())
    // at the end of the function
    let file_content = fs::read(path)?;
    // Creates the lexer. For this the file
    // content is convert to a &str. Here we
    // just call `.unwrap()` since we do not
    // want to mess with this second result
    let lexer = IRLexer::new(str::from_utf8(&file_content).unwrap());
    let mut parser = IRParser::new(&lexer);
    // This is the ir stream produced by the
    // parser. You can now use this for whatever
    // you want :)
    let ir_stream = parser.parse_program();
    Ok(())
}
```
