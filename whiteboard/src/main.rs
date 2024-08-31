use std::thread;

use api_design::subtle_breaking_changes::{/*HolderType, Lifetime,*/  SyncType};

fn main() {
    // let a = HolderType::new();
    // a.process(12);

    // let mut b = Lifetime::new();

    // {
    //     let word: String = String::from("bird");
    //     b.set_word(&word);
    // }

    // b.print_word();

    let sync_type = SyncType::new();
    let mut sync_type2 = sync_type.clone();
    let thr: thread::JoinHandle<()> = thread::spawn(move || {
        sync_type2.add(2);
        ()
    });

    let sync_type3 = sync_type.clone();
    let thr2 = thread::spawn(move || {
        let mut typ2 = sync_type3.clone();
        typ2.add(2);
        ()
    });

    thr.join().unwrap();
    thr2.join().unwrap();
}
