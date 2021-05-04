// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

mod tokens;
mod read;

use {
    read::read::read_lines,

    tokens::*,

    std::{
        io::prelude::*,
        fs::{
            File,
            remove_file
        },

        process::{Command},

        path::Path
    }
};

fn extract_file_name(_path: &str) -> &str {
    let data = Path::new(_path).file_name().unwrap().to_str().unwrap();

    &data
}


fn main() {
    let arguments: Vec<String> = std::env::args().collect();

    if arguments.len() < 2 {
        std::process::exit(1);
    }

    let mut tokens  = InitTokens {
        generated_data: "".to_string(),
        print_data    : "".to_string(),
        put_data      : "".to_string(),

        tokens        : Default::default(),

        is_statement  : false,
        is_print      : false,
        is_put        : false
    };

    tokens.init_tokens();

    let mut __tokens: Vec<String> = Vec::new();

    let c_source_filename = format!("{}_solfege.c", extract_file_name(arguments.last().unwrap()));

    if let Ok(lines) = read_lines(arguments.last().unwrap()) {
        for line in lines {
            if let Ok(ip) = line {
                 let t_tokens: Vec<&str> = ip.split(' ').collect::<Vec<&str>>();

                for token in t_tokens {
                    tokens.codegen(*tokens.tokenize(&token.to_string()),
                                   token.to_string());
                }
            }
        }
    }

    tokens.generated_data.push('}');

    if Path::exists(c_source_filename.as_ref()) {
        remove_file(c_source_filename.clone());
    }

    let mut file =
        match File::create(Path::new(&c_source_filename.clone())) {
        Err(why) => panic!("Couldn't create: {}", why),
        Ok(file) => file,
    };

    match file.write_all(tokens.generated_data.as_bytes()) {
        Err(why) => {
            panic!("Couldn't write to {}: {}",
                   c_source_filename.clone(),  why)
        },
        Ok(_) => {
            println!("Successfully wrote to {}", c_source_filename.clone());

            let output = Command::new("cc")
                .arg(c_source_filename.clone())
                .arg("-o")
                .arg("solfege_data")
                .output().unwrap_or_else(|e| {
                    panic!("Failed to execute process: {}", e)
                });

            if !output.status.success() {
                let _s = String::from_utf8_lossy(&output.stderr);

                print!("Error : \n{}", _s);

                std::process::exit(1);
            }

            Command::new("./solfege_data")
                .status();
        },
    }
}
