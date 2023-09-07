use std::fs::read;
use std::ops::Deref;



#[derive(Debug, Clone)]
struct ChildNode<T>{
    data: T,
    next: Option<Box<ChildNode<T>>>
}

#[derive(Debug, Clone)]
pub struct SimpleLinkedList<T: Clone>{
    node0: Option<Box<ChildNode<T>>>,
}

impl<T: PartialEq + Clone> ChildNode<T> {
    fn push_back(&mut self, value: T) {
        match self.next {
            Some(ref mut v) => {
                v.push_back(value);
            },
            None=> {
                self.next = Some(Box::new(ChildNode{
                    data: value,
                    next: None
                }));
            }
        }
    }

    ///
    fn search(&mut self, value: &T) -> bool {
        // Check the current node
        if self.data.eq(value) {
            return true;
        }

        // If there are more nodes, check the next one
        match self.next {
            Some(ref mut v) => {
                v.search(value)
            },
            None => {
                false
            }
        }
    }

    fn len(&self) -> usize {
        let mut count: usize = 0;

        let mut current = self;
        loop {
            match current.next {
                Some(ref next_node) => {
                    count += 1;
                    current = next_node;
                }
                None => {
                    break;
                }
            }
        }

        count
    }
}

impl<T: PartialEq + Clone> SimpleLinkedList<T> {
    ///Does not allocate until first item is inserted
    pub fn new() -> SimpleLinkedList<T>{
        SimpleLinkedList{
            node0: None
        }
    }

    /// Insert at the ned of the linked list
    pub fn insert(&mut self,value: T) {
        match &mut self.node0 {
            None => {
                self.node0 = Some(Box::new(ChildNode{
                    data: value,
                    next: None
                }))
            }
            Some(ref mut v) => {
                v.push_back(value);
            }
        }
    }

    /// Check every node and see if a value matches
    pub fn search(&mut self,value:&T) -> bool {
        match self.node0 {
            None => false,
            Some(ref mut v) =>{
                v.search(value)
            }
        }
    }

    pub fn len(&self) -> usize{
        let mut len: usize = 0;

        match self.node0 {
            Some(ref v) => {
                len+=1;
                len += v.len();
            },
            None => {
                return  0;
            }
        }

        len

    }
}

impl<T: PartialEq + Clone> Default for SimpleLinkedList<T> {
    fn default() -> Self {
        SimpleLinkedList::new()
    }
}

#[test]
fn ll_test_1(){
    let mut list = SimpleLinkedList::new();
    list.insert("First".to_string());
    list.insert("Second".to_string());
    list.insert("Third".to_string());

    let expected ="SimpleLinkedList { node0: Some(ChildNode { data: \"First\", next: Some(ChildNode { data: \"Second\", next: Some(ChildNode { data: \"Third\", next: None }) }) }) }"
        .to_string();
    assert_eq!(format!("{:?}", list), expected);
    assert!("aaa".to_string().eq(&"aaa".to_string()));
    assert!(list.search(&"First".to_string()));
    assert!(list.search(&"Second".to_string()));
    assert!(list.search(&"Third".to_string()));
    assert!(!list.search(&"Fourth".to_string()));
    std::mem::drop(list);

}





