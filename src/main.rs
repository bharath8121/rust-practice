use std::fmt::Debug;
use std::io;

mod linked_list;
mod task_project;

use task_project::task_manager::TaskManager;
use task_project::task::Task;
use crate::task_project::task_manager::new_task_manager;

fn insert_at_linked_list_head(mut ll: linked_list::LinkedList) -> linked_list::LinkedList {

    println!("enter the integer to be inserted into the linked list");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let input: i16 = input.trim().parse().expect("failed to convert value to integer");

    ll.insert_at_head(input);
    ll
}

fn insert_at_index(mut ll: linked_list::LinkedList) -> linked_list::LinkedList {

    println!("enter the integer to insert into the linked list");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let val: i16 = input.trim().parse().expect("failed to convert value to integer");

    input = String::new();
    println!("enter index to insert the item:");
    io::stdin().read_line(&mut input).expect("failed to read line");
    let index: i16 = input.trim().parse().expect("failed to convert value to integer");

    let res = ll.insert_at(index, val);
    if res.is_err() {
        println!("failed to insert into linked list: {}", res.err().unwrap());
    }

    ll
}

fn start_linked_list_program() {

    let mut ll = linked_list::LinkedList::new();

    loop {
        println!("Select one of the following options: ");
        println!("1. insert at head");
        println!("2. insert anywhere");
        println!("enter X to exit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        if input.trim_end_matches("\n").trim() == "X" {
            break;
        }

        let input: i16 = input.trim().parse().expect("failed to convert value to integer");

        match input {
            1 => ll = insert_at_linked_list_head(ll),
            2 => ll = insert_at_index(ll),

            _ => {}
        }
    }

    println!("\n\n printing items in the linked list:");
    ll.print();
}

fn main() {
    let task_manager = new_task_manager();
    let res = task_manager.add_task(String::from("test task"), chrono::Utc::now());
    match res {
        Ok(value) => println!("\ntask added successfully. {:?}", value),
        Err(error) => println!("error occurred with task: {:?}", error)
    }

}