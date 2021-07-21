use todo_backend;

#[rocket::main]
async fn main() {
    todo_backend::rocket()
        .launch()
        .await;
}