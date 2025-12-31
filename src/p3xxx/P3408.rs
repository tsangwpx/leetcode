// Problem 3408
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Default)]
struct TaskManager {
    tasks: HashMap<i32, (i32, i32)>,
    btree: BTreeSet<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut task_manager = Self::default();

        for task in tasks.iter() {
            let &[user_id, task_id, priority] = task.as_slice() else {
                panic!();
            };

            task_manager.add(user_id, task_id, priority);
        }

        task_manager
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.tasks.insert(task_id, (user_id, priority));
        self.btree.insert((priority, task_id));
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let value = self.tasks.get_mut(&task_id).unwrap();

        let (user_id, old_priority) = *value;
        *value = (user_id, new_priority);

        self.btree.remove(&(old_priority, task_id));
        self.btree.insert((new_priority, task_id));
    }

    fn rmv(&mut self, task_id: i32) {
        let (_user_id, priority) = self.tasks.remove(&task_id).unwrap();
        self.btree.remove(&(priority, task_id));
    }

    fn exec_top(&mut self) -> i32 {
        if let Some((_, task_id)) = self.btree.pop_last() {
            let (user_id, _priority) = self.tasks.remove(&task_id).unwrap();
            user_id
        } else {
            -1
        }
    }
}

/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */
fn f() {}
