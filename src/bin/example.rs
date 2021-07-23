use trie::TRIE;

pub fn example() {
    let s: &'static str = "/home/kuangjux/trie";
    let mut guard = unsafe {
        TRIE.lock()
    };
    guard.push(s, 0x8000000 as *mut u8);
    // println!("child: {:?}", guard.root().child);
    let handler = guard.call(s).unwrap();
    println!("0x{:x}", handler as usize);
    drop(guard);
}

fn main() {
    example();
}