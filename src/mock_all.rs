use mockall::predicate::*;
use mockall::*;
pub fn run() {
    println!("mock_all.rs");

    let mut mock = MockMyTrait::new();
    mock.expect_foo()
        .with(predicate::eq(4))
        .times(1)
        .returning(|x| x + 1);
    assert_eq!(5, call_with_four(&mock));
}

fn call_with_four(x: &dyn MyTrait) -> u32 {
    x.foo(4)
}

#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

// ---------------------------- mockall todo ----------------------------
#[derive(Debug, Clone, PartialEq, Eq)]
struct Todo {
    id: u32,
    name: String,
    is_done: bool,
}

#[automock]
trait TodoTrait {
    fn delete(&self, id: u32) -> u32;
    fn load(&self, id: u32) -> Todo;
    fn store(&self, todo: Todo) -> u32;
}

fn store_todo<T: TodoTrait>(todo: &T, val: Todo) -> u32 {
    todo.store(val)
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::mock_all::{store_todo, MockTodoTrait, Todo};

    #[test]
    fn test_todo_store() {
        let mut todo = MockTodoTrait::new();
        todo.expect_store()
            .with(eq(Todo {
                id: 1,
                name: "clean".to_string(),
                is_done: false,
            }))
            .times(1)
            .returning(|todo| todo.id);
        assert_eq!(
            1,
            store_todo(
                &todo,
                Todo {
                    id: 1,
                    name: "clean".to_string(),
                    is_done: false,
                }
            )
        );
    }
}
