use std::fs;

pub enum ParamChoices {
    Colon, CStyle
}

pub enum ReturnFormat {
    Default, Colon, Arrow
}

pub struct ConfigData {
    pub use_function_id: bool,
    pub function_id: String,
    pub param_mode: ParamChoices,
    pub return_format: ReturnFormat
}

impl ConfigData {
    pub fn new(use_fn_id: bool, fn_id: String, p_mode: ParamChoices, r_format: ReturnFormat) -> Self {
        // just return a new config data with the goods
        ConfigData {use_function_id: use_fn_id, function_id: fn_id, param_mode: p_mode, return_format: r_format}
    }
}

pub fn get_config_contents(file_name: &String) -> String {
    let mut file_path = String::from("configs/");
    file_path.push_str(file_name);
    //file_path.pop(); // remove next line identifier (\n)
    file_path.push_str(".txt");

    let file_contents = fs::read_to_string(&file_path).expect("Config file was not found!");

    file_contents
}

pub fn get_config_data(contents: &String) -> ConfigData {
    let mut token = String::from("");
    let mut value = String::from("");
    // default values for our config data struct that we'll change over the following loop
    let mut new_config_data = ConfigData::new(false, String::from(""), ParamChoices::Colon, ReturnFormat::Default);
    let mut adding_to_token = true; // basically an on off switch for what to add to;

    // fix for config files not being fully parsed: add a newline at end to fully parse!
    let mut new_contents = contents.clone();
    new_contents.push('\n');

    for character in new_contents.chars() {
        if character == ' ' {
            continue;
        } else if character == '\n' {
            // parse into config data
            // I could use match here but I like this better for strings :P
            if token == "use_function_identifier" {
                if value == "true" {
                    new_config_data.use_function_id = true;
                } else if value == "false" {
                    new_config_data.use_function_id = false;
                } else {
                    panic!("use_function_identifier in configs folder uses an unsupported value! (can only be true or false)");
                }
            } else if token == "function_identifier" {
                new_config_data.function_id = value.clone();
            } else if token == "param_mode" {
                if value == "colon" {
                    new_config_data.param_mode = ParamChoices::Colon;
                } else if value == "c-style" {
                    new_config_data.param_mode = ParamChoices::CStyle;
                } else {
                    panic!("param_mode in configs folder uses an unsupported value! (only colon or c-style)");
                }
            } else if token == "return_format" {
                if value == "default" {
                    new_config_data.return_format = ReturnFormat::Default;
                } else if value == "colon" {
                    new_config_data.return_format = ReturnFormat::Colon;
                } else if value == "arrow" {
                    new_config_data.return_format = ReturnFormat::Arrow;
                } else {
                    panic!("return_format in configs folder uses an unsupported value! (only default, colon, or arrow)");
                }
            } else {
                panic!("unidentified token found in config file! (token: {})", token);
            }
            // reset
            token.clear();
            value.clear();
            adding_to_token = true;
        } else if character == ':' {
            if adding_to_token {
                adding_to_token = false;
            } else {
                panic!("why have we already switched w/o touching a colon?");
            }
        } else {
            if adding_to_token {
                token.push(character);
            } else {
                value.push(character);
            }
        }
    }
    new_config_data
}