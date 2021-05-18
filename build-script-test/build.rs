use std::fs::File;
use std::io::Write;

fn main() {
    let language = std::env::var("GREET_LANG").unwrap_or("en".to_string());

    let greeting = match language.as_ref() {
        "en" => "Hello!",
        "es" => "¡Hola!",
        "el" => "γεια σας",
        "de" => "Hallo!",
        x => panic!("Unsupported language code {}", x),
    };

    let rust_code = format!(
        "fn greet() {{
        println!(\"{}\");
}}",
        greeting
    );

    let mut file = File::create("src/greet.rs").unwrap();
    file.write_all(rust_code.as_bytes()).unwrap();
}
