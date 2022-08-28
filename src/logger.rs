#[allow(dead_code)]
pub fn raw(text: &str) { print(text.to_string()); }

pub fn error(text: String) {
    let cthread = std::thread::current();
    let thread_name = match cthread.name() {
        Some(o) => o,
        None => "Unknown",
    };

    let text = format!("[ERROR]: [{0}]: {1}",thread_name, text);
    print(text);
}

pub fn debug(text: &str) {
    if crate::DEBUG {
        let cthread = std::thread::current();
        let thread_name = match cthread.name() {
            Some(o) => o,
            None => "Unknown",
        };

        let text = format!("[DEBUG]: [{0}]: {1}",thread_name, text);
        print(text);

    }
}

#[allow(dead_code)]
pub fn debug2(text: &str, object: &str) {
    if crate::DEBUG {

        let text = format!("[DEBUG]: [{0}]: {1}",object, text.to_uppercase());
        print(text);

    }
}

fn print(text: String) {
    println!("{}",text);
}