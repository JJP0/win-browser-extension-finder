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

    // Create empty vectors for each file path - to store filenames if found
    let mut chrome_files: Vec<String> = Vec::new();
    let mut edge_one_files: Vec<String> = Vec::new();
    let mut edge_two_files: Vec<String> = Vec::new(); 

    // Default path for browser extensions, Chrome + Edge
    let chrome_path_string = format!("\\\\{pc}\\c$\\Users\\{user}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\Extensions\\");
    let edge_path_one_string = format!("\\\\{pc}\\c$\\Users\\{user}\\AppData\\Local\\Microsoft\\Edge\\User Data\\Default\\Extensions\\");
    let edge_path_two_string = format!("\\\\{pc}\\c$\\users\\{user}\\AppData\\Local\\Microsoft\\Edge\\User Data\\Webstore Downloads\\");

    // Convert paths from string format to actual paths
    let chrome_path = Path::new(&chrome_path_string);
    let edge_path_one = Path::new(&edge_path_one_string);
    let edge_path_two = Path::new(&edge_path_two_string);
 
    // Combine [Path]s into array
    let paths_arr: [&Path; 3] = [chrome_path, edge_path_one, edge_path_two];



    // Two counters used to maintain consistency if one path returns nothing
    let mut path_err_count = 1;
    let mut file_path_counter = 0;
 
    for path in paths_arr {

        // If path exists...
        if path.exists() {

            println!("[-] {} {}", "Looking for files in", path.display().to_string());

            // Read each file in path
            for file in path.read_dir().expect("[!] Read dir failed") {

                // Checks to ignore default extensions for Chrome and Edge, google results return malware related query which is irrelevant
                if file.as_ref().expect("[!!!] Failed to read file").path().file_name().unwrap().to_str().unwrap().to_owned() != "nmmhkkegccagdldgiimedpiccmgmieda"
                    && file.as_ref().expect("[!!!] Failed to read file").path().file_name().unwrap().to_str().unwrap().to_owned() != "ghbmnnjooekpmoecnnnilnnbdlolhkhi"
                    && file.as_ref().expect("[!!!] Failed to read file").path().file_name().unwrap().to_str().unwrap().to_owned() != "jmjflgjpcpepeafmmgdpfkogkghcpiha"
                    && file.as_ref().expect("[!!!] Failed to read file").path().file_name().unwrap().to_str().unwrap().to_owned() != "Temp"
                     {

                // If file can be read, move file to string vector to maintain a list of file names
                    if let Ok(file) = file {
                        match file_path_counter {
                            // Extract file name as string and push to string vector
                            0 => chrome_files.push(file.path().file_name().unwrap().to_str().unwrap().to_owned()),
                            1 => edge_one_files.push(file.path().file_name().unwrap().to_str().unwrap().to_owned()),
                            2 => edge_two_files.push(file.path().file_name().unwrap().to_str().unwrap().to_owned()),

                            _ => break,
                        }
                    }
                }
            }
            // Loop into next file path, same again
            file_path_counter += 1;

        // If path is empty, push single 'No path found' to string vector
        } else {
            println!("[!] No path found for {}\n", path.display().to_string());
            match path_err_count {
                1 => chrome_files.push("No path found".to_string()),
                2 => edge_one_files.push("No path found".to_string()),
                3 => edge_two_files.push("No path found".to_string()),

                _ => println!("[?] How did this happen?"),
            }
        }
        path_err_count += 1
    }  

    return (chrome_files, edge_one_files, edge_two_files);
}


// This func needs to be refactored, lots of repeated code
fn google_search(array_1: &Vec<String>, array_2: &Vec<String>, array_3: &Vec<String>) -> (String, String, String) {


    // Empty hash maps used to map .crx filename to actual extension name, Key:Value pairs
    let mut dictionary_1 = HashMap::new();
    let mut dictionary_2 = HashMap::new();
    let mut dictionary_3 = HashMap::new();

    let mut googled_array_1: Vec<String> = Vec::new();
    let mut googled_array_2: Vec<String> = Vec::new();
    let mut googled_array_3: Vec<String> = Vec::new();

    // For file in list of filenames (array_1 = Chrome extension filenames etc..)
    for filename in array_1 {
        // If file names found, run search() func on filename. Else, treat as nothing found
        if filename != "No path found" {
            println!("[..] Identifying {}", filename.to_string());
            googled_array_1.push(search(filename));

        } else {
            googled_array_1.push("No path found".to_string());
        }
    }

    // Repeat of above for Edge (1) filenames
    for filename in array_2 {
        if filename != "No path found" {
            println!("[..] Identifying {}", filename.to_string());
            googled_array_2.push(search(filename));

        } else {
            googled_array_2.push("No path found".to_string());
            }
        }

    // Repeat of above for Edge (2) filenames
    for filename in array_3 {
        if filename != "No path found" {
            println!("[..] Identifying {}", filename.to_string());
            googled_array_3.push(search(filename));

        } else {
            googled_array_3.push("No path found".to_string());
            }
        }

    // Loops through .crx filename and result of search() func, adds .crx filename and actual extension name to hashmap
    for (original_filename, google_result) in array_1.iter().zip(googled_array_1.iter()) {
        if original_filename != "No path found" {
            dictionary_1.insert(original_filename.to_string(), google_result.to_string());
        } else {
            dictionary_1.insert("No path found".to_string(), "No path found".to_string());
        }
    }

    // Same as above
    for (original_filename, google_result) in array_2.iter().zip(googled_array_2.iter()) {
        if original_filename != "No path found" {
            dictionary_2.insert(original_filename.to_string(), google_result.to_string());
        } else {
            dictionary_2.insert("No path found".to_string(), "No path found".to_string());
        }
    }

    // Same as above
    for (original_filename, google_result) in array_3.iter().zip(googled_array_3.iter()) {
        if original_filename != "No path found" {
            dictionary_3.insert(original_filename.to_string(), google_result.to_string());
        } else {
            dictionary_3.insert("No path found".to_string(), "No path found".to_string());
        }
    }

    // String vecs to convert hashamp Key:Value pairs into single ".crx filename - extensions name"
    let mut print_vec_1: Vec<String> = Vec::new();
    let mut print_vec_2: Vec<String> = Vec::new();
    let mut print_vec_3: Vec<String> = Vec::new();

    for (original_filename, google_result) in &dictionary_1 {
        let temp_var = format!("{} - {}", original_filename, google_result);
        print_vec_1.push(temp_var);
    }

    for (original_filename, google_result) in &dictionary_2 {
        let temp_var = format!("{} - {}", original_filename, google_result);
        print_vec_2.push(temp_var);
    }

    for (original_filename, google_result) in &dictionary_3 {
        let temp_var = format!("{} - {}", original_filename, google_result);
        print_vec_3.push(temp_var);
    }

    // Convert string vecs into single string to make it easier to output. Join lines with <br> which is html newline/break
    let final_str_1 = print_vec_1.join("<br>");
    let final_str_2 = print_vec_2.join("<br>");
    let final_str_3 = print_vec_3.join("<br>");

    return (final_str_1, final_str_2, final_str_3);

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
