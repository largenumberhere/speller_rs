
use crate::simple_linked_list::SimpleLinkedList;
pub const HASH_MAX: usize = 26;

#[derive(Clone)]
pub struct SimpleHashset<const N: usize, T: Clone + SimpleHashable>{
    buckets: [SimpleLinkedList<T>; N]
}
impl<const N: usize, T: PartialEq + Clone + SimpleHashable>  SimpleHashset<N, T>{
    pub fn new() -> SimpleHashset<N,T>{
        let arr: [SimpleLinkedList<T>; N] =  std::array::from_fn(|_| SimpleLinkedList::default());
        SimpleHashset{
            buckets: arr
        }
    }

    pub fn insert(&mut self, value: T) {
        let hash_value = value.hash();
        self.buckets[hash_value].insert(value);
    }

    pub fn check(&mut self, value: &T) -> bool {
        let hash_value = value.hash();
        self.buckets[hash_value].search(value)
    }

    pub fn len(&mut self) -> usize {
        let mut count = 0;
        for b in self.buckets.iter_mut(){
            count+= b.len();
        }

        count
    }
}

impl<const N: usize,T: PartialEq + Clone + SimpleHashable> Default for SimpleHashset<N,T> {
    fn default() -> Self {
        SimpleHashset::new()
    }
}

pub trait SimpleHashable {
    fn hash(&self) -> usize;
}

impl SimpleHashable for String {
    fn hash(&self) -> usize {
        let mut chars = self.chars();
        let c: char = match chars.next() {
            None => {
                return 26; // Default hash
            }
            Some(v) => {v}
        };
        alpha_char_to_usize(&c)
    }
}

impl SimpleHashable for &str {
    fn hash(&self) -> usize {
        let mut chars = self.chars();
        let c: char = match chars.next() {
            None => {
                return 26; // Default hash
            }
            Some(v) => {v}
        };
        alpha_char_to_usize(&c)
    }
}

/// Lossy conversion from a latin letter to a usize 0 - 26 (inclusive). Defaults to 26
fn alpha_char_to_usize(c: &char) -> usize{
    match c {
        'a' | 'A' => 0,
        'b' | 'B' => 1,
        'c' | 'C' => 2,
        'd' | 'D' => 3,
        'e' | 'E' => 4,
        'f' | 'F' => 5,
        'g' | 'G' => 6,
        'h' | 'H' => 7,
        'i' | 'I' => 8,
        'j' | 'J' => 9,
        'k' | 'K' => 10,
        'l' | 'L' => 11,
        'm' | 'M' => 12,
        'n' | 'N' => 13,
        'o' | 'O' => 14,
        'p' | 'P' => 15,
        'q' | 'Q' => 16,
        'r' | 'R' => 17,
        's' | 'S' => 18,
        't' | 'T' => 19,
        'u' | 'U' => 20,
        'v' | 'V' => 21,
        'w' | 'W' => 22,
        'x' | 'X' => 23,
        'y' | 'Y' => 24,
        'z' | 'Z' => 25,
        _=> 26
    }
}

#[test]
fn hashset_test(){
    let mut set:SimpleHashset<HASH_MAX,_> = SimpleHashset::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");

    assert!(set.check(&"a"));
    assert!(set.check(&"b"));
    assert!(set.check(&"c"));
    assert!(!set.check(&"d"));

    assert_eq!(set.len(), 3);
    assert_ne!(set.len(), 4);

    set.insert("d");
    assert_eq!(set.len(),4);
    set.insert("d");
    assert_eq!(set.len(), 5);
}
