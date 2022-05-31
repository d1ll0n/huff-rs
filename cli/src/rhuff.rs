use std::path::Path;

use clap::Parser;
use utils::io::*;
// use lexer::Lexer;

/// Efficient Huff compiler.
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Rhuff {
    path: Option<String>,

    /// The source path to the contracts (default: "./src").
    #[clap(short = 's', long = "source-path", default_value = "./src")]
    source: String,

    /// The output file path.
    #[clap(short = 'o', long = "output")]
    output: Option<String>,

    /// The output directory (default: "./artifacts").
    #[clap(short = 'd', long = "output-directory")]
    outputdir: Option<String>,

    /// Optimize compilation.
    #[clap(short = 'z', long = "optimize")]
    optimize: bool,

    /// Generate and log bytecode (default: false).
    #[clap(short = 'b', long = "bytecode")]
    bytecode: bool,

    /// Print the output to the terminal.
    #[clap(short = 'p', long = "print")]
    print: bool,
}

fn main() {
    // Parse the command line arguments
    let cli = Rhuff::parse();

    // Gracefully derive file compilation
    let input_files: Vec<String> = match cli.path {
        Some(path) => {
            // If the file is huff, we can use it
            let ext = Path::new(&path).extension().unwrap_or_default();
            println!("Extension: {}", ext.to_str().unwrap());
            if ext.eq("huff") {
                vec![path]
            } else {
                // Otherwise, override the source files and use all files in the provided dir
                unpack_files(&path).unwrap_or_default()
            }
        }
        None => {
            // If there's no path, unpack source files
            let source: String = cli.source;
            unpack_files(&source).unwrap_or_default()
        }
    };

    // Validate files strictly (the extensions must be .huff)
    let compile_files: Vec<String> = input_files
        .iter()
        .filter(|&f| Path::new(&f).extension().unwrap().eq("huff"))
        .cloned()
        .collect();

    println!("Compiling files: {:?}", compile_files);

    // Perform Lexical Analysis
    // let lexer: Lexer = Lexer::new();
    // TODO: print compiled bytecode if flagged
    // TODO: print output to terminal if flagged

    // TODO: Unpack output (if only one file or contract specified)
    // TODO: Unpack output directory
}