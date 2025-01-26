use sea_orm::{ActiveModelTrait, Set};
#[cfg(test)]
mod test {
    use sea_orm::Set;
    use crate::entities::task;
    use crate::entities::task::ActiveModel;
    use crate::task_project::task_storage_db_async::TaskStorageDBAsync;

    #[tokio::test]
    async fn test_create_task_async() {
        let t = TaskStorageDBAsync{};
        let res = t.create_task(task::ActiveModel{
            id: Set("something100".to_string()),
            description: Set("hello world 123".to_string()),
            completed: Set(Option::from(false)),
        }).await;
        assert!(&res.is_ok());
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_update_task_async() {
        let t = TaskStorageDBAsync{};
        let res = t.update_task(ActiveModel{
            id: Set("something100".to_string()),
            description: Set("hello world 456".to_string()),
            completed: Set(Option::from(true)),
        }).await;
        assert!(&res.is_ok());
        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_get_task_async() {
        let t = TaskStorageDBAsync{};
        let res = t.get_task_for_id("something100".to_string()).await;
        assert!(&res.is_ok());
        println!("{:?}", res);
    }

}