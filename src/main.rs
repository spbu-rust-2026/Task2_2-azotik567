use std::io;
fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(text: String) -> String {
    let first = text.split_whitespace().next().unwrap_or("").to_string();

    let mut message = text;
    message.push_str(" rust");
    let shown = format!("{}", &mut message);
    let read = format!("{}", &mut message);

    return format!("{}\n{}\n{}\n", first, shown, read);
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
