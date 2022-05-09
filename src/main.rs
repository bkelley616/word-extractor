use std::fs::File;
use std::io::{self};
use std::time::Instant;

mod helpers;

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let matching_set_text_file = File::open("./english-words-master/words.txt")?;
    let set = helpers::create_matching_word_set(matching_set_text_file);

    helpers::process_files(set);
    helpers::print_elapsed_time(start_time);
    println!("check the output directory for the generated files");
    Ok(())
}
