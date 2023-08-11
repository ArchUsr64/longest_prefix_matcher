#[derive(Debug)]
struct BinaryTrie {
    filled: bool,
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
}

impl BinaryTrie {
    fn new() -> Self {
        Self {
            filled: false,
            left: None,
            right: None,
        }
    }
}

fn main() {
    let root = BinaryTrie::new();
    println!("{root:#?}");
}
