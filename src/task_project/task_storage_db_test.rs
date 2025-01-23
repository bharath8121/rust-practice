#[cfg(test)]
mod test {
    use crate::task_project::task::Task;
    use crate::task_project::task_storage_db::TaskStorageDB;

    #[test]
    fn test_insert_storage_db() {
        let storage = TaskStorageDB::new();
        let res = storage.create_task(&Task{
            id: "something".to_string(),
            description: "something test 1".to_string(),
            completed: false,
        });
        assert!(res.is_ok());
    }

    #[test]
    fn test_update_storage_db() {
        let storage = TaskStorageDB::new();
        let res = storage.update_task(&Task{
            id: "something".to_string(),
            description: "something test 2".to_string(),
            completed: false,
        });
        assert!(res.is_ok());
    }

    #[test]
    fn test_get_by_id() {
        let storage = TaskStorageDB::new();
        let res = storage.get_task_for_id(String::from("something"));
        assert!(res.is_ok());
        assert_eq!(res.unwrap().id, "something");
        println!("{:?}", res.unwrap());
    }

}