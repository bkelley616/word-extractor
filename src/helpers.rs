use chrono::SubsecRound;
use epub::doc::EpubDoc;
use html2text::from_read;
use pdf_extract::extract_text;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::path::PathBuf;
use std::{fs::*, io::Write};

pub fn create_matching_word_set(file: File) -> HashSet<String> {
    println!("running non-english word extractor");
    println!("loading set...");

    let mut reader = BufReader::new(file);
    let mut set = HashSet::new();

    let mut buf = String::new();
    while let Ok(n_bytes) = reader.read_line(&mut buf) {
        if n_bytes == 0 {
            break;
        }

        set.insert(buf.trim().to_lowercase().to_string());
        buf.clear();
    }

    println!("finished loading {} items into set", set.len());
    return set;
}

pub fn process_files(known_word_set: HashSet<String>) {
    println!("creating output file...");

    let date_time = chrono::offset::Local::now().round_subsecs(0).to_string();
    let date_time_parts: Vec<&str> = date_time.split_whitespace().collect();

    let mut new_file_name = String::from("./output/");
    new_file_name.push_str(date_time_parts[0]);
    new_file_name.push_str("_");
    new_file_name.push_str(&date_time_parts[1].replace(":", "_"));
    new_file_name.push_str(".txt");

    let output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(new_file_name)
        .expect("failed to create output file");

    let re_remove_specials = Regex::new(r"[^0-9a-zA-Z]+").unwrap();
    println!("reading input files...");

    let input_path = "./input/";
    let paths = fs::read_dir(input_path).expect("failed to read input directory");

    let mut unique_set = HashSet::new();

    for path in paths {
        let path = path.unwrap().path();
        let path_string = path.display().to_string();
        let filename = &path_string[input_path.len()..path_string.len()];
        println!("processing: {}", filename);

        let file_ext = Path::new(&filename).extension().unwrap().to_str().unwrap();
        match file_ext {
            "epub" => process_epub_file(
                path,
                &output_file,
                &known_word_set,
                &re_remove_specials,
                &mut unique_set,
            ),
            "pdf" => process_pdf_file(
                path,
                &output_file,
                &known_word_set,
                &re_remove_specials,
                &mut unique_set,
            ),
            "txt" => process_text_file(
                path,
                &output_file,
                &known_word_set,
                &re_remove_specials,
                &mut unique_set,
            ),
            _ => process_unsupported_file(file_ext, filename),
        };
    }
}

fn process_epub_file(
    input_path: PathBuf,
    mut output_file: &File,
    known_word_set: &HashSet<String>,
    sanitize_re: &regex::Regex,
    unique_set: &mut HashSet<String>,
) {
    let epub_file = EpubDoc::new(input_path);
    let mut epub_file = epub_file.unwrap();

    let mut index = 0;
    let last_page = epub_file.get_num_pages();
    while index < last_page - 1 {
        let html = epub_file.get_current().expect("failed to read epub page");
        let text = from_read(&html[..], u16::MAX as usize);

        for word in clean_text(&text).split_whitespace() {
            let sanitized_word = sanitize_re.replace_all(word, "").to_lowercase();
            if should_write_and_save_value(sanitized_word.to_string(), unique_set, known_word_set) {
                unique_set.insert(sanitized_word.to_string());
                let mut formatted_word = sanitized_word.to_string();
                formatted_word.push_str("\n");
                match output_file.write_all(formatted_word.as_bytes()) {
                    Ok(_val) => (),
                    Err(_err) => println!("error writing file!"),
                }
            }
        }
        epub_file.go_next().expect("failed to get next epub page");
        index += 1;
    }
}

fn process_pdf_file(
    input_path: PathBuf,
    mut output_file: &File,
    known_word_set: &HashSet<String>,
    sanitize_re: &regex::Regex,
    unique_set: &mut HashSet<String>,
) {
    let pdf_text = extract_text(input_path).expect("failed to open pdf");
    for word in clean_text(&pdf_text).split_whitespace() {
        let sanitized_word = sanitize_re.replace_all(word, "").to_lowercase();
        if should_write_and_save_value(sanitized_word.to_string(), unique_set, known_word_set) {
            unique_set.insert(sanitized_word.to_string());
            let mut formatted_word = sanitized_word.to_string();
            formatted_word.push_str("\n");
            match output_file.write_all(formatted_word.as_bytes()) {
                Ok(_val) => (),
                Err(_err) => println!("error writing file!"),
            }
        }
    }
}

fn process_text_file(
    input_path: PathBuf,
    mut output_file: &File,
    known_word_set: &HashSet<String>,
    sanitize_re: &regex::Regex,
    unique_set: &mut HashSet<String>,
) {
    let input_file = File::open(input_path);

    match input_file {
        Ok(input_file) => {
            let mut reader = BufReader::new(input_file);

            let mut buf = String::new();
            while let Ok(n_bytes) = reader.read_line(&mut buf) {
                if n_bytes == 0 {
                    break;
                }

                for word in clean_text(&buf).split_whitespace() {
                    let sanitized_word = sanitize_re.replace_all(word, "").to_lowercase();
                    if should_write_and_save_value(
                        sanitized_word.to_string(),
                        unique_set,
                        known_word_set,
                    ) {
                        unique_set.insert(sanitized_word.to_string());
                        let mut formatted_word = sanitized_word.to_string();
                        formatted_word.push_str("\n");
                        match output_file.write_all(formatted_word.as_bytes()) {
                            Ok(_val) => (),
                            Err(_err) => println!("error writing file!"),
                        }
                    }
                }

                buf.clear();
            }
        }
        _ => {
            println!("an error occurred, skipping file")
        }
    }
}

fn process_unsupported_file(ext: &str, filename: &str) {
    println!(
        "unsupported file type: .{}\nsupported file types are .txt and .pdf\nskipping {}",
        ext, filename
    );
}

pub fn print_elapsed_time(start_time: std::time::Instant) {
    let time_in_ms = start_time.elapsed().as_millis();
    match time_in_ms < 1000 {
        true => {
            println!("completed successfully! finished in {} ms", time_in_ms);
        }
        false => {
            println!(
                "completed successfully! finished in {} s",
                time_in_ms / 1000
            );
        }
    }
}

fn should_write_and_save_value(
    text: String,
    unique_set: &mut HashSet<String>,
    known_word_set: &HashSet<String>,
) -> bool {
    let mut is_alphabetic = false;
    for c in text.chars() {
        if c.is_alphabetic() {
            is_alphabetic = true;
        }
    }
    let text_not_empty = text != "";
    let text_not_epub_artifacts = !text.contains("htmlfilepos") && !text.contains("http");
    let text_not_already_seen = !unique_set.contains(&text);
    let text_not_not_a_known_word = !known_word_set.contains(&text);
    return is_alphabetic
        && text_not_empty
        && text_not_epub_artifacts
        && text_not_already_seen
        && text_not_not_a_known_word;
}

fn clean_text(text: &str) -> String {
    return text
        .replace("--", " ")
        .replace("-", " ")
        .replace("—", " ")
        .replace("[", " ");
}
