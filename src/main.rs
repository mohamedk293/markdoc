use std::fs;
use std::fs::File;
use std::io::{stdin, Write};

use configs::ParamChoices;
mod configs;

#[allow(dead_code)]
struct Symbol {
    keywords: Vec<String>,
    index: usize
}

impl Symbol {
    fn new(keywords: Vec<String>, index: usize) -> Self {
        Symbol {keywords: keywords, index: index}
    }
}

fn read_markdoc_contents(file_name: &String) -> String {
    let mut file_path = String::from("mkdcfile/");
    file_path.push_str(file_name);
    //file_path.pop(); // remove next line identifier (\n)
    file_path.push_str(".mkdc");

    // println!("{}", file_path);

    let file_contents = fs::read_to_string(&file_path).expect("File was not found!");

    file_contents
}

fn search_for_keywords(contents: &String) -> Vec<Symbol> {
    let mut keywords: Vec<Symbol> = Vec::new();
    let mut scanning = false;
    let mut current_symbol = Symbol::new(vec![], 0);
    let mut current_keyword = String::from("");
    let mut current_index: usize = 0;
    for character in contents.chars() {
        if !scanning {
            if character == '^' {
                // start scanning/adding keywords.
                scanning = true;
            }
        } else {
            if character == '^' {
                scanning = false;
                if current_keyword.len() > 0 {
                    current_symbol.keywords.push(current_keyword);
                }
                keywords.push(current_symbol);
                current_index += 1;
                current_symbol = Symbol::new(vec![], current_index);
                current_keyword = String::from("");
            } else {
                if character == ',' || character == ' ' {
                    if current_keyword.len() > 0 {
                        current_symbol.keywords.push(current_keyword);
                        current_keyword = String::from("");
                    }
                } else {
                    current_keyword.push(character);
                }
            }
        }
    }

    keywords
}

fn parse_symbols(symbols: &Vec<Symbol>, config: &configs::ConfigData) -> Vec<String> {
    let mut parsed_symbols: Vec<String> = vec![];
    let mut last_not_get_symbol: Option<&Symbol> = None;
    for (_symbol_index, symbol) in symbols.iter().enumerate() {
        let mut output_string = String::from("");
        let mut function_found = false;
        let mut param_vec: Vec<String> = vec![];
        let mut type_string = String::from("");
        let mut override_search = false;
        let mut header_str = String::from("");
        for (i, current_keyword) in symbol.keywords.iter().enumerate() {
            let next_keyword = symbol.keywords.get(i + 1);
            match next_keyword {
                None => {
                    // this will be done later, i am tired and need rest
                    // nvm, don't need to do anything, perfect!
                    // println!("Nothing found...");
                }
                Some(new_keyword) => {
                    if current_keyword != "get" {
                        last_not_get_symbol = Some(symbol);
                        if current_keyword == "function" {
                            output_string.push_str(new_keyword);
                            output_string.push('(');
                            function_found = true;
                        } else if current_keyword == "return" {
                            type_string.push_str(new_keyword);
                            // type_string.push_str(" "); // add a space :)
                        } else if current_keyword == "param" {
                            let return_type = symbol.keywords.get(i + 2);
                            match return_type {
                                Some(type_given) => {
                                    let mut total_result = String::from("");
                                    // format param based on config file
                                    match config.param_mode {
                                        ParamChoices::Colon => { // value: type
                                            total_result.push_str(new_keyword);
                                            total_result.push_str(": ");
                                            total_result.push_str(type_given);
                                        }
                                        ParamChoices::CStyle => { // type value
                                            total_result.push_str(type_given);
                                            total_result.push(' ');
                                            total_result.push_str(new_keyword);
                                        }
                                    }
                                    
                                    param_vec.push(total_result);
                                }
                                None => {
                                    panic!("parameter has no return type!");
                                }
                            }
                        } else if current_keyword == "variable" {
                            override_search = true;
                            let return_type = symbol.keywords.get(i + 2);
                            match return_type {
                                Some(type_given) => {
                                    let mut total_result = String::from("## `");
                                    total_result.push_str(type_given);
                                    total_result.push(' ');
                                    total_result.push_str(new_keyword);
                                    total_result.push('`');
                                    parsed_symbols.push(total_result);
                                }
                                None => {
                                    panic!("variable has no return type!");
                                }
                            }
                        }
                    } else {
                        // this is a final keyword aka once this is called, that's it
                        // let previous_symbol = symbols.get(symbol_index - 1);
                        match last_not_get_symbol {
                            Some(result_symbol) => {
                                if new_keyword == "param" {
                                    let param_index_indicator = symbol.keywords.get(i + 2);
                                    match param_index_indicator {
                                        None => {
                                            panic!("get function has no index argument!");
                                        }
                                        Some(index_as_string) => {
                                            let mut index = index_as_string.parse::<usize>().expect("get function index argument could not be parsed!");
                                            index -= 1; // we're starting at 1, rust starts at 0 :P
                                            let get_result = handle_get(result_symbol, new_keyword, Some(index));
                                            parsed_symbols.push(get_result);
                                        }
                                    }
                                } else if new_keyword == "variable" {
                                    let what_to_return = symbol.keywords.get(i + 2);
                                    match what_to_return {
                                        None => {
                                            let get_result = handle_get(result_symbol, new_keyword, None);
                                            parsed_symbols.push(get_result);
                                        }
                                        Some(value) => {
                                            if value == "return" {
                                                let get_result = handle_get(result_symbol, new_keyword, None);
                                                let target_value = &get_result[1..get_result.len()-1].to_string(); // trims the `` symbols from resultant.
                                                let final_get_result = handle_get(result_symbol, target_value, None);
                                                parsed_symbols.push(final_get_result);
                                            } else {
                                                let get_result = handle_get(result_symbol, new_keyword, None);
                                                parsed_symbols.push(get_result);
                                            }
                                        }
                                    }
                                } else {
                                    let get_result = handle_get(result_symbol, new_keyword, None);
                                    parsed_symbols.push(get_result);
                                }
                                override_search = true;
                                break;
                            }
                            None => {
                                panic!("get function won't work, nothing to get from!");
                            }
                        }
                    }
                }
            }
        }
        if !override_search {
            // format everything yippee
            for (i, param) in param_vec.iter().enumerate() {
                if i < param_vec.len() - 1 { // if we are not on the last param
                    let mut new_param_phrase = String::from(param);
                    new_param_phrase.push_str(", ");
                    output_string.push_str(&new_param_phrase);
                } else {
                    output_string.push_str(param);
                }
            }
            // close function
            if function_found {
                output_string.push(')');
                header_str.push_str("## `");
                if config.use_function_id {
                    match config.return_format {
                        configs::ReturnFormat::Colon => {
                            header_str.push_str(&config.function_id);
                            header_str.push(' ');
                            header_str.push_str(&output_string);
                            header_str.push_str(": ");
                            header_str.push_str(&type_string);
                        }
                        configs::ReturnFormat::Arrow => {
                            header_str.push_str(&config.function_id);
                            header_str.push(' ');
                            header_str.push_str(&output_string);
                            header_str.push_str(" -> ");
                            header_str.push_str(&type_string);
                        }
                        configs::ReturnFormat::Default => { // in case you didn't wanna ;)
                            header_str.push_str(&type_string);
                            header_str.push(' ');
                            header_str.push_str(&output_string);
                        }
                    }
                } else {
                    header_str.push_str(&type_string);
                    header_str.push(' ');
                    header_str.push_str(&output_string);
                }
            }
            header_str.push('`');
            parsed_symbols.push(header_str);
        }
    }
    parsed_symbols
}

fn handle_get(symbol: &Symbol, target_value: &String, optional_param_index: Option<usize>) -> String {
    for (i, keyword) in symbol.keywords.iter().enumerate() {
        if keyword == target_value {
            match optional_param_index {
                None => { // no param index = not looking for a parameter OR is a single param function
                    let target_keyword = symbol.keywords.get(i + 1);
                    match target_keyword {
                        Some(value) => {
                            let final_result = format!("`{}`", value.to_string());
                            return final_result;
                        }
                        None => {
                            panic!("target value has nothing!");
                        }
                    }
                }
                Some(param_index) => {
                    // loop through it again!
                    let result = get_from_param_index(symbol, param_index);
                    match result {
                        Some(value) => {
                            let final_result = format!("`{}`", value.to_string());
                            return final_result;
                        }
                        None => {
                            panic!("could not find the correct param value given param index!");
                        }
                    }
                }
            }
        }
    }
    println!("Given: {}", target_value);
    panic!("Did not find the element to get from!");
}

fn get_from_param_index(symbol: &Symbol, param_index: usize) -> Option<String> {
    let mut current_index: usize = 0;
    for (i, word) in symbol.keywords.iter().enumerate() {
        if word == "param" {
            let other_word = symbol.keywords.get(i + 1);
            match other_word {
                None => {
                    panic!("why does this param have no argument?");
                }
                Some(value) => {
                    if current_index == param_index {
                        return Some(value.to_string());
                    } else {
                        current_index += 1;
                    }
                }
            }
        }
    }
    // panic!("could not find the correct param using given index");
    return None;
}

fn return_formatted_md_file_contents(file_contents: &String, parsed_keys: &Vec<String>) -> String {
    let mut in_braces = false;
    let mut formatted_string = String::from("");
    let mut current_key_index: usize = 0;
    for character in file_contents.chars(){
        if !in_braces {
            if character == '^' {
                in_braces = true;
            } else {
                formatted_string.push(character);
            }
        } else {
            if character == '^' {
                if current_key_index < parsed_keys.len() {
                    formatted_string.push_str(&parsed_keys[current_key_index]);
                }
                current_key_index += 1;
                in_braces = false;
            }
        }
    }
    formatted_string
}

fn write_to_md_file(file_name: &String, contents: &String) {
    let file_extension = format!("{}.md", file_name);
    let test_file = File::open(&file_extension);
    match test_file {
        Ok(_) => {
            fs::remove_file(&file_extension).expect("Removal did not work");
        }
        Err(_) => {} // we already have no file!
    }

    let mut new_file = File::create(&file_extension).unwrap();
    new_file.write_all(contents.as_bytes()).expect("ouch");
}

fn main() {
    println!("Welcome to Markdoc! Please enter the file name of your .mkdc file.\nThe .mkdc file will be parsed into a .md file.");
    let mut file_input = String::new();
    stdin().read_line(&mut file_input).unwrap(); // read user input for file name
    file_input.pop();

    println!("Config file name? (exclude .txt)");
    let mut config_input = String::new();
    stdin().read_line(&mut config_input).unwrap();
    config_input.pop();
    let config_file_contents = configs::get_config_contents(&config_input);

    let resulting_config = configs::get_config_data(&config_file_contents);
    // println!("{}", resulting_config.use_function_id);

    let file_contents = read_markdoc_contents(&file_input);
    // println!("{}", file_contents);

    let keywords_found = search_for_keywords(&file_contents);
    // for symbol in keywords_found.iter() {
    //     println!("Symbol found: ");
    //     for word in symbol.keywords.iter() {
    //         println!("{}", word);
    //     }
    // }

    let parsed_keywords = parse_symbols(&keywords_found, &resulting_config);
    // for parsed_result in parsed_keywords.iter() {
    //     println!("Parsed symbol: {}", parsed_result);
    // }

    let formatted_contents = return_formatted_md_file_contents(&file_contents, &parsed_keywords);
    // println!("Result contents: \n{}", formatted_contents);
    write_to_md_file(&file_input, &formatted_contents);
    println!("File was created/updated under {}.md!\nThank you for using Markdoc.\n", file_input);
}
