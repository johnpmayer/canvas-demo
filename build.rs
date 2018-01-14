
extern crate glsl;
extern crate nom;

use std::process;
// use std::io::prelude::*;

mod files {
    pub fn list_file_pairs() -> Vec<(String, String)> {
        // TODO: Faking it
        vec!(
            (String::from("src/shaders/vert.glsl"), String::from("src/shaders/vert.rs")),
            (String::from("src/shaders/frag.glsl"), String::from("src/shaders/frag.rs")),
        )
    }
}

mod compiler {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Error;
    use glsl::parser::translation_unit;
    use nom::IResult;

    enum CompilerError {
        IoError(Error)
    }

    impl From<Error> for CompilerError {
        fn from(err: Error) -> CompilerError { CompilerError::IoError(err) }
    }

    pub fn process_file(glsl_filename: String, rust_filename: String) -> Result<(), Error> {
        let mut glsl_file = File::open(glsl_filename)?;
        let mut glsl_contents = String::new();
        glsl_file.read_to_string(&mut glsl_contents)?;

        let result = translation_unit(glsl_contents.as_bytes());

        match result {
            IResult::Done(_, tu) => {
                println!("Got something");
                for ext_directive in tu.iter() {
                    println!("{:?}", ext_directive);
                }
            },
            _ => println!("Something wrong")
        }

        let rust_file = File::create(rust_filename)?;


        Ok(())
    }
}

fn main() {
    for (glsl_filename, rust_filename) in files::list_file_pairs() {
        println!("Compiling {} to {}", glsl_filename, rust_filename);
        let _err = compiler::process_file(glsl_filename, rust_filename);
    };

    println!("All good! Faking a bad exit code for debugging.");
    process::exit(1)
}