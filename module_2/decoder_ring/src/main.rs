/*
Attempts to statistically decode a Caesar cipher.
Here's an example of how to use it:

This is a shift 16 message: "Off to the bunker. Every person for themselves"
"Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc"

cargo run -- --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --guess
// Another message example:
cargo run -- --message "qjyx xjj nk dtz hfs ijhwduy ymnx. Mfmfmf" --guess

// Example where decode_message.txt is used as input:
cargo run -- --file decode_message.txt --guess

*/

use clap::Parser;
use decoder_ring::print_stats_analysis;
use std::fs;

/// CLI tool to reverse engineer a Caesar cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The message to decrypt
    #[arg(short, long)]
    message: Option<String>,

    #[arg(short, long)]
    file: Option<String>,

    //statistical information about the message
    #[arg(short, long)]
    stats: bool,

    //guess the shift
    #[arg(short, long)]
    guess: bool,
}

// Run main
fn main() {
    // Parse the CLI
    let args = Args::parse();

    // If file input, parse the file
    let message = if let Some(file_path) = args.file {
        fs::read_to_string(file_path).expect("Failed to read file")
    } else {
        args.message.expect("No message or file provided")
    };

    //stats
    if args.stats {
        print_stats_analysis(&message);
    }

    // Guess to decode the input message
    if args.guess {
        let (_depth, best_shift, decrypted, max_score) = decoder_ring::guess_shift(&message, 26);
        println!("Best shift: {}, score: {}", best_shift, max_score);
        println!("Decrypted message: {}", decrypted);
    }
}
