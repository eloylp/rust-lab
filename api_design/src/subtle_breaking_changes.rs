use std::{cell::Cell, rc::Rc, sync::{Arc, Mutex}};

pub struct HolderType {
    subtype: u32,
}

impl HolderType {
    pub fn new() -> Self {
        HolderType { subtype: 12 }
    }

    // pub fn process(&self, number: u32) {
    //     println!("{}", number + self.subtype)
    // }
    /// Now accepts mut ... breaking change !
    pub fn process(& mut self, number: u32) {
        self.subtype += number;
        println!("{}", self.subtype);
    }
}

// pub struct Lifetime {
//     pub word: String,
// }

// impl Lifetime {
//     pub fn new() -> Self {
//         Lifetime {
//             word: "".to_string(),
//         }
//     }

//     pub fn set_word(&mut self, word: &str) {
//         self.word = word.to_string();
//     }

//     pub fn print_word(&self) {
//         println!("{}", self.word)
//     }
// }


// pub struct Lifetime<'a> {
//     pub word: &'a str,
// }

// impl <'a>Lifetime<'a> {
//     pub fn new() -> Self {
//         Lifetime {
//             word: "",
//         }
//     }

//     pub fn set_word(&mut self, word: &'a str) {
//         self.word = word;
//     }

//     pub fn print_word(&self) {
//         println!("{}", self.word)
//     }
// }

// #[derive(Clone)]
// pub struct SyncType {
//     number: u32,
// }

// impl SyncType {
//     pub fn new() -> Self {
//         SyncType { number: 1 }
//     }
// }

#[derive(Clone)]
pub struct SyncType {
    number: Arc<Mutex<u32>>
}

impl SyncType {
    pub fn new()-> Self {
        SyncType { number: Arc::new(Mutex::new(1)) }
    }

    pub fn add(&mut self, add: u32){
        let mut guard = self.number.lock().expect("poisoned!!");
        *guard += add
    }
}
