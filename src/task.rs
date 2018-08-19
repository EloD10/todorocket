use diesel::{self, prelude::*};
use schema::tasks::{
    self,
    dsl::{tasks as all_tasks, completed as task_completed}
};

#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool
}

#[table_name="tasks"]
#[derive(FromForm, Insertable)]
pub struct Todo {
    pub description: String,
}

impl Task {
    pub fn all(conn: &PgConnection) -> Vec<Task> {
        all_tasks.order(tasks::id.desc()).load::<Task>(conn).unwrap()
    }

    pub fn insert(todo: Todo, conn: &PgConnection) -> bool {
        diesel::insert_into(all_tasks).values(todo).execute(conn).is_ok()
    }

    pub fn toggle_with_id(id: i32, conn: &PgConnection) -> bool {
        let task = all_tasks.find(id).get_result::<Task>(conn);
        if task.is_err() {
            return false;
        }

        let new_status = !task.unwrap().completed;
        let updated_task = diesel::update(all_tasks.find(id));
        updated_task.set(task_completed.eq(new_status)).execute(conn).is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &PgConnection) -> bool {
        diesel::delete(all_tasks.find(id)).execute(conn).is_ok()
    }
}
