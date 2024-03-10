use std::io::stdin;

pub fn read_entry() -> String {
    let mut messages = String::new();
    let stdin_reader = stdin();
    let reader_res = stdin_reader.read_line(&mut messages);

    if reader_res.is_err() {
        println!("error! {:?}", reader_res.err())
    };

    messages.trim().to_string()
}