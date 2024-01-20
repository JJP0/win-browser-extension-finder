use std::io;
use std::path::Path;
use std::collections::HashMap;
use scraper::{Html, Selector};
use reqwest::blocking::Client;


fn main() {

    // Get username and PC name
    let username = input_loop("Username:".to_string());
    let pc_name = input_loop("PC Name:".to_string());

    // Pass username/PC name to filename func and create vars
    let (files_1, files_2, files_3) = get_filenames(&username, &pc_name);
    let (final_1, final_2, final_3) = google_search(&files_1, &files_2, &files_3);

    println!("\n[---] {} [---]\n{}\n","Chrome extensions", final_1.replace("<br>", "\n"));
    println!("[---] {} [---]\n{}\n","Edge extensions (1)", final_2.replace("<br>", "\n"));
    println!("[---] {} [---]\n{}\n","Edge extensions (2)", final_3.replace("<br>", "\n"));

}

 
// Get input until not empty, no other checks done
fn input_loop(input: String) -> String {

    let mut usr_input = String::new();

    println!("[=] {} {}", "Enter the", input);

    loop {
        io::stdin()
            .read_line(&mut usr_input)
            .expect("Invalid input");

        usr_input = usr_input.trim().to_string();

        if usr_input != "" {  
            break;
        } else {
            println!("[!] {} {}", input, "not accepted. Try again.\n")
        }                                                                                                                                            
    }
    return usr_input;
}

fn get_filenames(user: &str, pc: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
    let paths = [
        format!("\\\\{pc}\\c$\\Users\\{user}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Extensions\\"),
        format!("\\\\{pc}\\c$\\Users\\{user}\\AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Extensions\\"),
        format!("\\\\{pc}\\c$\\users\\{user}\\AppData\\Local\\Microsoft\\Edge\\User Data\\Webstore Downloads\\"),
    ];

    let mut chrome_files = Vec::new();
    let mut edge_one_files = Vec::new();
    let mut edge_two_files = Vec::new();

    for (i, path_str) in paths.iter().enumerate() {
        let path = Path::new(path_str);

        let files = if path.exists() {
            println!("[-] Looking for files in {}", path.display().to_string());

            path.read_dir()
                .expect("[!] Read dir failed")
                .filter_map(|file| {
                    let file = file.expect("[!!!] Failed to read file");

                    let file_name_str = file.path().file_name()?.to_str()?.to_owned();

                    if !should_ignore_file(&file_name_str) {
                        Some(file_name_str)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            println!("[!] No path found for {}\n", path.display().to_string());
            vec!["No path found".to_string()]
        };

        match i {
            0 => chrome_files = files,
            1 => edge_one_files = files,
            2 => edge_two_files = files,
            _ => println!("[?] How did this happen?"),
        }
    }

    (chrome_files, edge_one_files, edge_two_files)
}

fn should_ignore_file(file_name: &str) -> bool {
    let ignored_files = ["nmmhkkegccagdldgiimedpiccmgmieda", "ghbmnnjooekpmoecnnnilnnbdlolhkhi", "jmjflgjpcpepeafmmgdpfkogkghcpiha", "Temp"];
    ignored_files.contains(&file_name)
}


fn process_arrays<T, U>(original_array: &[T], googled_array: &[U]) -> HashMap<String, String>
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let mut result_dict = HashMap::new();

    for (original_filename, google_result) in original_array.iter().zip(googled_array.iter()) {
        let original_str = original_filename.as_ref();
        let google_str = google_result.as_ref();

        if original_str != "No path found" {
            result_dict.insert(original_str.to_string(), google_str.to_string());
        } else {
            result_dict.insert("No path found".to_string(), "No path found".to_string());
        }
    }

    result_dict
}

fn process_search(array: &Vec<String>) -> Vec<String> {

    array
        .into_iter()
        .map(|filename| {
            let filename_str = filename.as_ref();

            if filename_str != "No path found" {
                println!("[..] Identifying {}", filename_str);
                search(filename_str)
            } else {
                "No path found".to_string()
            }
        })
        .collect()
}

fn create_print_vector<T, U>(dictionary: &HashMap<T, U>) -> Vec<String>
where
    T: AsRef<str> + std::fmt::Display,
    U: AsRef<str> + std::fmt::Display,
{
    dictionary
        .iter()
        .map(|(original_filename, google_result)| {
            format!("{} - {}", original_filename, google_result)
        })
        .collect()
}

// This func needs to be refactored, lots of repeated code
fn google_search(array_1: &Vec<String>, array_2: &Vec<String>, array_3: &Vec<String>) -> (String, String, String) {


    let googled_array_1 = process_search(array_1);
    let googled_array_2 = process_search(array_2);
    let googled_array_3 = process_search(array_3);

    let dictionary_1 = process_arrays(&array_1, &googled_array_1);
    let dictionary_2 = process_arrays(&array_2, &googled_array_2);
    let dictionary_3 = process_arrays(&array_3, &googled_array_3);

    let print_vec_1 = create_print_vector(&dictionary_1);
    let print_vec_2 = create_print_vector(&dictionary_2);
    let print_vec_3 = create_print_vector(&dictionary_3);

    // Convert string vecs into single string to make it easier to output. Join lines with <br> which is html newline/break
    let final_str_1 = print_vec_1.join("<br>");
    let final_str_2 = print_vec_2.join("<br>");
    let final_str_3 = print_vec_3.join("<br>");

    (final_str_1, final_str_2, final_str_3)

}

fn search(extension_filename: &str) -> String {

    // Create search query, standard google search with .crx filename as search term.
    let search_query = format!("https://www.google.com/search?q={extension_filename}");

    // Call search query with reqwest
    let response = reqwest::blocking::get(search_query)
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    // Get 'h3' items which are search results
    let title_selector = scraper::Selector::parse("h3").unwrap();

    // Get text from h3 items
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    let mut count = 0;
    let mut result = String::new();

    // Used to get first search result, which is the name of the file extension 9/10 times
    for title in titles {
        if count == 1 {
            break;
        } else {

        // Convert search result to owned string
        let line = title.to_string();

        // Next 2 lines are used to extract just the search result and remove any html tags
        let start_bytes = line.find(r#"">"#).unwrap_or(0); //index where "pattern" starts                                            
                                                            // or beginning of line if
                                                                // "pattern" not found

        let end_bytes = line.find("</").unwrap_or(line.len()); //index where "<" is found
                                                                // or end of line

        result = line[start_bytes+2..end_bytes].to_string();
        count += 1;
        }
    }
    return result;
}
