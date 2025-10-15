use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Section {
    author: String,
    title: String,
    paragraphs: Vec<Paragraph>,
}


fn main() {
    let section = Section {
        author: String::from("John Doe"),
        title: String::from("Introduction to Rust"),
        paragraphs: vec![
            Paragraph {
                text: String::from("Rust is a systems programming language."),
            },
            Paragraph {
                text: String::from("It emphasizes safety and performance."),
            },
        ],
    };

    let json = serde_json::to_string_pretty(&section).unwrap();
    println!("{}", json);
}
