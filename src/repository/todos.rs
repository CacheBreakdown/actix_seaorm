use crate::entity::{self, prelude::*, todos};
// use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait};

#[derive(Debug, Clone)]
pub struct TodosRepository {
    pub db_conn: DatabaseConnection,
}

impl TodosRepository {
    pub async fn get_todos(&self) -> Vec<todos::Model> {
        todos::Entity::find()
            .all(&self.db_conn)
            .await
            .expect("Error Find All")
    }

    pub async fn add(&self) {
        let add_todos = todos::ActiveModel {
            todo_id: ActiveValue::Set(10),
            todo_name: ActiveValue::Set("username4".to_owned()),
            todo_description: ActiveValue::Set(Some("test4".to_owned())),
            ..Default::default()
        };

        // Utc::now();
        let _res = Todos::insert(add_todos)
            .exec(&self.db_conn)
            .await
            .expect("add error");
    }

    pub async fn delete(&self, id: i32) {
        let delete_todos = todos::ActiveModel {
            todo_id: ActiveValue::Set(id.into()),
            ..Default::default()
        };
        let _res = delete_todos
            .delete(&self.db_conn)
            .await
            .expect("Error Delete");
    }

    pub async fn update(&self, id: i32) {
        let update_todos = todos::ActiveModel {
            todo_id: ActiveValue::Set(id.into()),
            todo_name: ActiveValue::Set("username update".to_owned()),
            todo_description: ActiveValue::Set(Some("update".to_owned())),
            ..Default::default()
        };
        let _res = update_todos
            .update(&self.db_conn)
            .await
            .expect("Error Update");
    }

    pub async fn find(&self, id: i32) -> std::option::Option<entity::todos::Model> {
        Todos::find_by_id(id.into())
            .one(&self.db_conn)
            .await
            .expect("Error Find")
    }
}
