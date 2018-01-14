
extern crate glsl;
extern crate nom;

use std::process;
// use std::io::prelude::*;

mod files {
    pub fn list_file_pairs() -> Vec<(String, String, String)> {
        // TODO: Faking it
        vec!(
            (
                String::from("src/shaders/purple_vert.glsl"),
                String::from("src/shaders/purple_frag.glsl"), 
                String::from("src/shaders/purple.rs"),
            ),
        )
    }
}

mod compiler {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Error;
    use glsl::parser::translation_unit;
    use glsl::syntax::ExternalDeclaration;
    use nom::IResult;

    pub enum CompilerError {
        IoError(Error),
        ParseError(String)
    }

    impl From<Error> for CompilerError {
        fn from(err: Error) -> CompilerError { CompilerError::IoError(err) }
    }

    pub fn parse_file(glsl_filename: &str) -> Result<(String, Vec<ExternalDeclaration>), CompilerError> {
        let mut glsl_file = File::open(glsl_filename)?;
        let mut contents = String::new();
        glsl_file.read_to_string(&mut contents)?;
        
        let decls = {
            let result = translation_unit(contents.as_bytes());
            match result {
                IResult::Done(_, tu) => {
                    println!("Success parsing {}", glsl_filename);
                    Ok(tu)
                },
                _ => Err(CompilerError::ParseError(format!("TODO better errors")))
            }?
        };
        Ok((contents, decls))
    }

    pub fn process_file(vert_filename: String, frag_filename: String, rust_filename: String) -> Result<(), CompilerError> {
        let (vert_source, vert_declarations) = parse_file(vert_filename.as_str())?;
        let (frag_source, frag_declarations) = parse_file(frag_filename.as_str())?;

        Ok(())
    }
}

fn main() {
    for (vert_filename, frag_filename, rust_filename) in files::list_file_pairs() {
        println!("Compiling {} & {} to {}", vert_filename, frag_filename, rust_filename);
        let _err = compiler::process_file(vert_filename, frag_filename, rust_filename);
    };

    println!("All good! Faking a bad exit code for debugging.");
    process::exit(1)
}