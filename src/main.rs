use clap::Parser;
use sha256::try_digest;
use std::path::Path;
use walkdir::WalkDir;
mod cli;

fn remove_nuls(input: &str) -> String {
    input.replace('\0', "")
}

fn is_ascii(text: &str) -> bool {
    text.chars().all(|c| c as u32 <= 127)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    let read_this_file = cli.list_file;
    let mut exclude_dirs_check_list: Vec<String> = vec![String::new()];
    let mut exclude_files_check_list: Vec<String> = vec![String::new()];
    let mut exclude_dirs_create_list: Vec<String> = vec![String::new()];
    let mut exclude_files_create_list: Vec<String> = vec![String::new()];
    let mut is_some_exclude_dirs = false;
    if let Some(exclude_dirs) = cli.exclude_dirs {
        exclude_dirs_check_list = exclude_dirs.clone();
        exclude_dirs_create_list = exclude_dirs.clone();
        is_some_exclude_dirs = true;
    }
    let mut is_some_exclude_files = false;
    if let Some(exclude_files) = cli.exclude_files {
        exclude_files_check_list = exclude_files.clone();
        exclude_files_create_list = exclude_files.clone();
        is_some_exclude_files = true
    }
    if read_this_file.is_some() {
        let read_this_file = read_this_file.unwrap();
        // let list_file = std::fs::read(&read_this_file);

        // Read the file contents into a byte vector
        let content = std::fs::read(&read_this_file);

        // Print the raw bytes (may not be human-readable)
        // println!("Raw bytes:\n{:?}", content);

        // If you need to decode as UTF-8, you can try this:

        // list_file.expect("can't read list file").
        match content {
            Ok(list) => {
                // Try decoding as UTF-8, handling potential errors
                let decoded_text = String::from_utf8_lossy(&list)
                    // .expect("Invalid UTF-8 encoding")
                    .trim()
                    .to_string();
                let decoded_text = remove_nuls(&decoded_text);
                let rows: Vec<String> = decoded_text
                    .split("\n")
                    .map(|s| s.trim().to_owned())
                    .collect();
                let mut new_rows: Vec<String> = vec![];
                for row in rows {
                    let mut columns = row.split_whitespace().map(|s| s.trim().to_owned());
                    let filepath_from_list: String;
                    if let Some(column) = &columns.next() {
                        filepath_from_list = column.clone().trim().to_string();
                    } else {
                        filepath_from_list = row.clone().trim().to_string();
                    }
                    if filepath_from_list.is_empty() {
                        continue;
                    }
                    let filepath: &Path;
                    let clean_input: String;
                    let shorter: String;
                    if cfg!(target_os = "windows") {
                        clean_input = remove_nuls(&filepath_from_list.as_str());
                        if is_ascii(clean_input.as_str()) {
                            filepath = Path::new(clean_input.as_str());
                        } else {
                            // when redirection windows --create-list output, some
                            // Unicode artifacts get's attached
                            // this code removes them
                            shorter = clean_input[6..].to_string();
                            filepath = Path::new(shorter.as_str());
                        }
                    } else {
                        let clean_input = &filepath_from_list;
                        filepath = Path::new(clean_input.trim());
                    }
                    // let clean_panic = format!("{}", filepath.display());
                    // println!("{}", clean_panic.as_str().to_ascii_lowercase());
                    if is_some_exclude_dirs || is_some_exclude_files {
                        let excluded_dirs = &exclude_dirs_check_list;
                        let excluded_files = &exclude_files_check_list;
                        let entry = &filepath;

                        let mut hit = false;
                        for dir in excluded_dirs {
                            if dir.is_empty() {
                                continue;
                            }
                            if entry.is_dir() && &format!("{}", entry.display()) == dir {
                                hit = true;
                                break;
                            }
                            let formatted_dir = format!("{}/", dir);
                            if format!("{}", entry.display()).starts_with(formatted_dir.as_str()) {
                                hit = true;
                                break;
                            }
                        }

                        for file in excluded_files {
                            if file.is_empty() {
                                continue;
                            }
                            if format!("{}", entry.display()) == file.as_str() {
                                hit = true;
                                break;
                            }
                        }

                        if hit && format!("{}", filepath.display()) == filepath_from_list {
                            let old_hash: String;
                            // let check_count_more_than_one: bool;
                            if let Some(column) = &columns.next() {
                                old_hash = column.clone();
                                let mut new_line = vec![format!(
                                    "{}  {}  {}",
                                    filepath_from_list, old_hash, "SKIPPED"
                                )];
                                new_rows.append(&mut new_line);
                            } else {
                                let mut new_line =
                                    vec![format!("{} {}", filepath_from_list, "SKIPPED")];
                                new_rows.append(&mut new_line);
                            }
                            if cli.verbose {
                                println!("{} {}", "SKIPPED", filepath_from_list)
                            }
                        } else if hit {
                            continue;
                        } else if entry.is_dir() {
                            continue;
                        }
                    }
                    let old_hash: String;
                    let check_count_more_than_one: bool;
                    if let Some(column) = &columns.next() {
                        old_hash = column.clone();
                        if old_hash == "SKIPPED" {
                            check_count_more_than_one = false;
                        } else {
                            check_count_more_than_one = true;
                        }
                    } else {
                        old_hash = "".to_string();
                        check_count_more_than_one = false
                    }
                    if check_count_more_than_one {
                        let new_hash = try_digest(&filepath).unwrap();
                        if new_hash != old_hash {
                            let mut new_line = vec![format!(
                                "{}  {}  {}",
                                filepath_from_list, new_hash, "CHANGED"
                            )];
                            new_rows.append(&mut new_line);
                            if cli.verbose {
                                println!("{} {}", "CHANGED", filepath_from_list)
                            }
                        } else {
                            let mut new_line =
                                vec![format!("{}  {}  {}", filepath_from_list, new_hash, "OLD")];
                            new_rows.append(&mut new_line);
                            if cli.verbose {
                                println!("{} {}", "OLD", filepath_from_list)
                            }
                        }
                    } else {
                        println!("{}", filepath.display());
                        let new_hash = try_digest(&filepath).unwrap();
                        let mut new_line =
                            vec![format!("{}  {}  {}", filepath_from_list, new_hash, "NEW")];
                        new_rows.append(&mut new_line);
                        if cli.verbose {
                            println!("{} {}", "NEW", filepath_from_list)
                        }
                    }
                }
                let mut new_list_file_content = String::new();
                for row in new_rows {
                    new_list_file_content = format!("{}{}\n", new_list_file_content, row);
                }
                let new_file_name = format!("{}{}", &read_this_file, ".new");
                let writing_new_file_result =
                    std::fs::write(&new_file_name, &new_list_file_content);
                if writing_new_file_result.is_ok() {
                    let _ = std::fs::rename(&new_file_name, &read_this_file);
                } else {
                    panic!(
                        "Can't write new list file... {}",
                        writing_new_file_result.expect_err("error: ")
                    )
                }
                return Ok(());
            }
            Err(error) => {
                panic!("{}", error)
            }
        }
    }
    let create_list = cli.create_list;
    if create_list.is_some() {
        let dir = create_list.unwrap();
        if is_some_exclude_dirs || is_some_exclude_files {
            let excluded_dirs = &exclude_dirs_create_list;
            let excluded_files = &exclude_files_create_list;
            let root_dir = Path::new(&dir);
            for entry in WalkDir::new(root_dir) {
                let entry = entry?;

                let mut hit = false;
                for dir in excluded_dirs {
                    if dir.is_empty() {
                        continue;
                    }
                    if entry.path().is_dir() && &format!("{}", entry.path().display()) == dir {
                        hit = true;
                        break;
                    }
                    let formatted_dir = format!("{}/", dir);
                    if format!("{}", entry.path().display()).starts_with(formatted_dir.as_str()) {
                        hit = true;
                        break;
                    }
                }

                for file in excluded_files {
                    if file.is_empty() {
                        continue;
                    }
                    if format!("{}", entry.path().display()) == file.as_str() {
                        hit = true;
                        break;
                    }
                }

                if hit || entry.path().is_dir() {
                    continue;
                } else {
                    let print_clean = format!("{}", entry.path().display()).trim().to_string();
                    println!("{}", remove_nuls(print_clean.as_str()));
                }
            }
        } else {
            let root_dir = Path::new(&dir);
            for entry in WalkDir::new(root_dir) {
                let entry = entry?;
                if !entry.path().is_dir() {
                    let print_clean = format!("{}", entry.path().display()).trim().to_string();
                    println!("{}", remove_nuls(print_clean.as_str()));
                }
            }
        }
    }

    Ok(())
}
