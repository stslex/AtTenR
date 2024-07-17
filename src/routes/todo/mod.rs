mod request;
mod response;
mod todo;

pub trait ToDoRoute {
    fn mount_todo_route(self, base_url: &str) -> Self;
}
