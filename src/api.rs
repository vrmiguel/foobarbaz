use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use sea_orm::DatabaseConnection;

mod forms;
mod routes;

pub fn spawn_server(db_conn: DatabaseConnection) -> crate::Result<Server> {
    let server = HttpServer::new(move || {
        App::new().service(
            web::scope("/tasks")
                .service(routes::list_tasks)
                .service(routes::create_task)
                .service(routes::delete_task)
                .service(routes::get_task)
                // *
                .app_data(Data::new(db_conn.clone())),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(server)
}

// *: I didn't find a way to not clone the connection here.
// I couldn't move it into the `app_data` since Rust says this
// variable belongs to the closure and I couldn't pass
// in the conn. to `spawn_server` as a reference
// since then the compiler would want the connection
// to live for 'static
