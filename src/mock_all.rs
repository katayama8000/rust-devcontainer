use mockall::predicate::*;
use mockall::*;
use serde::de;
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
    use super::*;

    #[test]
    fn test_todo() {
        let mut todo = MockTodoTrait::new();
        todo.expect_delete().times(0).returning(|id| id);
        todo.expect_load().times(0).returning(|id| Todo {
            id,
            name: "clean".to_string(),
            is_done: false,
        });
        todo.expect_store().times(1).returning(|todo| todo.id);
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
