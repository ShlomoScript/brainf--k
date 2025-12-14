use brainf_ck::cli::App;
use brainf_ck::{compile, BfError, Interpreter};
use clap::Parser;
use clio::Input;
use std::io::Read;

fn main() -> Result<(), BfError> {
    let args = App::parse();

    let mut source = String::new();

    if let Some(mut file) = args.input {
        // CASE 1: --input FILE   (file)
        file.read_to_string(&mut source)?;
    } else if let Some(s) = args.string {
        // CASE 2: --string "<bf>"
        source = s;
    } else {
        // CASE 3: no args → try stdin (pipe)
        let mut stdin = Input::std(); // clio stdin reader
        stdin.read_to_string(&mut source)?;
    }

    // Compile and run
    let (code, _spans) = compile(&source)?;
    let mut vm = Interpreter::new(code, args.output);

    vm.run()?;

    Ok(())
}
