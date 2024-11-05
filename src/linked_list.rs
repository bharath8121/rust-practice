// use std::cell::RefCell;
// use std::io::Error;
//
// pub struct LinkedList {
//     head: Option<Box<LinkedListNode>>,
// }
//
// pub struct LinkedListNode {
//     val: i16,
//     next: Option<Box<LinkedListNode>>,
// }
//
// impl LinkedList {
//
//     pub fn new() -> Self {
//         LinkedList {
//             head: None,
//         }
//     }
//
//     pub fn print(&self) {
//         if self.head.is_none() {
//             print!("no items to print");
//         }
//
//         let mut curr_node = &self.head;
//         while curr_node.is_some() {
//             println!("{}", curr_node.as_ref().unwrap().val);
//             curr_node = &curr_node.as_ref().unwrap().next;
//         }
//     }
//
//     pub fn delete_at_head(&mut self) -> Result<bool, String> {
//         if let Some(mut node) = self.head.take() {
//             self.head = node.next.take();
//             return Ok(true)
//         }
//         Err(String::from("no head found"))
//     }
//
//     pub fn delete_at(&mut self, index: i16) -> Result<bool, String> {
//         let mut curr_index = 0;
//         let mut curr_node = &self.head;
//         while curr_index < index-1 {
//             if let Some(mut node) = curr_node {
//                 curr_node = &mut node.next;
//             } else {
//                 return Err(String::from("out of bounds"));
//             }
//             curr_index += 1;
//         }
//
//         if let Some(mut node) = curr_node {
//             curr_node = &node.next.take();
//         }
//
//         Ok(true)
//     }
//
//     pub fn insert_at(&mut self, index: i16, val: i16) -> Result<bool, String> {
//         if self.head.is_none() {
//            return Err(String::from("empty linked list. please insert at head first"));
//         }
//
//         let mut curr_node = &mut self.head;
//         let mut curr_index = 0;
//         while curr_index < index {
//             curr_index += 1;
//             if curr_node.is_none() {
//                 return Err(String::from(format!("index out of bound. length of linked list: {}", curr_index-1)));
//             }
//             curr_node = &mut curr_node.as_mut().unwrap().next;
//         }
//
//         let new_node = Box::new(LinkedListNode {val, next: curr_node.as_mut().unwrap().next.take() });
//         curr_node.as_mut().unwrap().next = Some(new_node);
//
//         Ok(true)
//     }
//
//     pub fn insert_at_head(&mut self, val: i16) {
//         let curr_node = LinkedListNode{val, next: self.head.take()};
//         self.head = Some(Box::new(curr_node));
//     }
//
// }