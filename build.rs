use std::{error::Error, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(windows)]
    let output = Command::new("./build_grammar.bat").output()?;
    #[cfg(not(windows))]
    let output = Command::new("./build_grammar.bash").output()?;

    if output.stderr.len() > 0 {
        eprintln!("!!!ANTLR ERROR!!!");
        eprintln!("{}", String::from_utf8(output.stderr).unwrap());
        Err(
            "Antlr failed to create grammar code.  Refer to the error above for error information."
                .into(),
        )
    } else {
        Ok(())
    }
}
