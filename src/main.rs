// TODO: Implement HashMap solution
// Usable TUI using tui-rs
#[derive(Debug, Clone)]
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

// TODO: Can lookup time be optimised using binary search over lexicographically sorted entries
struct LinearLookup<'a> {
    entries: Vec<&'a [bool]>,
}

trait LookUpTable<'table> {
    fn insert(&mut self, value: &'table [bool]);
    fn search<'a>(&'a self, value: &'a [bool]) -> &[bool];
    fn delete(&mut self, _: &'table [bool]) {
        todo!()
    }
}

impl LookUpTable<'_> for BinaryTrie {
    fn insert(&mut self, value: &[bool]) {
        match value.split_first() {
            Some((first, rest)) => {
                let take_left = !*first;
                let target_node = if take_left {
                    &mut self.left
                } else {
                    &mut self.right
                };
                match target_node {
                    &mut Some(ref mut child) => child.insert(rest),
                    &mut None => {
                        let mut new_child = Self::new();
                        new_child.insert(rest);
                        *target_node = Some(Box::new(new_child));
                    }
                };
            }
            None => {
                self.filled = true;
            }
        }
    }
    fn search<'a>(&'a self, value: &'a [bool]) -> &[bool] {
        let mut node_to_search = self;
        let mut last_match_depth = 0;
        for (depth, bit) in value.iter().enumerate() {
            let take_left = !*bit;
            if node_to_search.filled {
                last_match_depth = depth;
            }
            let target_node = if take_left {
                &node_to_search.left
            } else {
                &node_to_search.right
            };
            match target_node {
                Some(ref x) => node_to_search = x,
                None => break,
            }
        }
        &value[..last_match_depth]
    }
}

impl<'table> LookUpTable<'table> for LinearLookup<'table> {
    fn insert(&mut self, value: &'table [bool]) {
        if !self.entries.contains(&value) {
            self.entries.push(value);
        }
    }
    fn search<'a>(&'a self, value: &'a [bool]) -> &[bool] {
        let mut best_match_index = 0;
        for entry in self.entries.iter() {
            let mut matched_till = 0;
            for (i, bit) in entry.iter().enumerate() {
                match value.get(i) {
                    Some(test_bit) if test_bit == bit => (),
                    _ => break,
                }
                matched_till = i;
            }
            if matched_till > best_match_index {
                best_match_index = matched_till;
            }
        }
        &value[..=best_match_index]
    }
}

fn bool_vec_from_str(value: &str) -> Vec<bool> {
    let mut result = Vec::with_capacity(value.len());
    value.chars().for_each(|char| {
        result.push(match char {
            '0' => false,
            '1' => true,
            _ => panic!("Invalid symbol"),
        })
    });
    result
}

fn str_from_bool_slice(value: &[bool]) -> String {
    let mut result = String::with_capacity(value.len());
    for bit in value.iter().rev() {
        result.push(if *bit { '1' } else { '0' });
    }
    result
}

fn print_formatted_table(table: &[Vec<bool>], longest_sequence_length: usize) {
    println!("Longest: {longest_sequence_length}");
    print!("┌");
    (0..longest_sequence_length).for_each(|_| print!("─"));
    println!("┐");
    for row in table {
        print!("│");
        for bit in row {
            print!("{}", if *bit { 1 } else { 0 });
        }
        (0..(longest_sequence_length - row.len())).for_each(|_| print!(" "));
        println!("│");
    }
    print!("└");
    (0..longest_sequence_length).for_each(|_| print!("─"));
    println!("┘");
}

use std::io::{self, Write};
fn main() {
    let mut table = Vec::new();
    let mut trie = BinaryTrie::new();
    let mut longest_sequence_length = 0;
    loop {
        print!("Enter Command: ");
        if let Err(e) = io::stdout().flush() {
            println!("[Failed to write to STDOUT] {e:?}");
            continue;
        };
        let mut buffer = String::new();
        if let Err(e) = io::stdin().read_line(&mut buffer) {
            println!("[Failed to read from STDIN] {e:?}");
            continue;
        };
        let parsed: Vec<_> = buffer.split_whitespace().collect();
        let command = parsed[0].to_lowercase();
        let value = parsed[1];
        let value = bool_vec_from_str(value);
        match command.as_str() {
            "insert" | "i" => {
                longest_sequence_length = longest_sequence_length.max(value.len());
                trie.insert(&value);
                table.push(value);
                print_formatted_table(&table, longest_sequence_length);
            }
            "search" | "s" => {
                let result = trie.search(&value);
                println!("Longest matching prefix: {}", str_from_bool_slice(result));
            }
            _ => println!("[Invalid Command] '{command}' is invalid"),
        }
    }
}

#[test]
fn table_fmt() {
    let table = [
        vec![true, false, true, true, false],
        vec![true, false, true, true, false],
        vec![true, false, true, true, false],
        vec![false, true, true, false],
    ];
    print_formatted_table(&table, 5);
    panic!();
}

#[test]
fn trie_searching() {
    let mut trie = BinaryTrie::new();
    let entries = [
        &bool_vec_from_str("0"),
        &bool_vec_from_str("1"),
        &bool_vec_from_str("01"),
        &bool_vec_from_str("00"),
        &bool_vec_from_str("10"),
        &bool_vec_from_str("110"),
        &bool_vec_from_str("00001001"),
    ];
    for entry in entries {
        trie.insert(entry);
    }
    let test = |test_case, longest_matching_prefix| {
        assert_eq!(
            trie.search(&bool_vec_from_str(test_case)),
            bool_vec_from_str(longest_matching_prefix)
        )
    };
    test("11", "1");
    test("000", "00");
    test("1101", "110");
    test("1101010", "110");
    test("00001001001", "00001001");
}
