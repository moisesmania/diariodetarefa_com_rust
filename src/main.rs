#[macro_use]
extern crate rocket;

use rocket::{get, post, delete, routes};
use rocket_dyn_templates::{Template, context};
use mysql::*;
use mysql::prelude::*;

fn conectar_banco() -> PooledConn {
    let url = "mysql://root:@localhost/diario_tarefas";
    let pool = Pool::new(url).expect("Falha ao conectar ao banco de dados");
    pool.get_conn().expect("Falha ao obter conexão")
}

#[get("/")]
fn index() -> Template {
    let mut conn = conectar_banco();
    let tarefas: Vec<(i32, String, String)> = conn
        .query("SELECT id, titulo, descricao FROM tarefas")
        .unwrap();

    let contexto = context! {
        tarefas: tarefas.iter().map(|(id, titulo, descricao)| {
            context! { id: id, titulo: titulo, descricao: descricao }
        }).collect::<Vec<_>>()
    };

    Template::render("index", contexto)
}

#[post("/adicionar", data = "<form>")]
fn adicionar_tarefa(form: rocket::form::Form<Tarefa>) -> rocket::response::Redirect {
    let mut conn = conectar_banco();
    conn.exec_drop(
        "INSERT INTO tarefas (titulo, descricao) VALUES (:titulo, :descricao)",
        params! { "titulo" => &form.titulo, "descricao" => &form.descricao }
    ).unwrap();

    rocket::response::Redirect::to("/")
}

#[post("/editar/<id>", data = "<form>")]
fn editar_tarefa(id: i32, form: rocket::form::Form<Tarefa>) -> rocket::response::Redirect {
    let mut conn = conectar_banco();
    conn.exec_drop(
        "UPDATE tarefas SET titulo = :titulo, descricao = :descricao WHERE id = :id",
        params! { "titulo" => &form.titulo, "descricao" => &form.descricao, "id" => id }
    ).unwrap();

    rocket::response::Redirect::to("/")
}

#[delete("/excluir/<id>")]
fn excluir_tarefa(id: i32) -> rocket::response::Redirect {
    let mut conn = conectar_banco();
    conn.exec_drop("DELETE FROM tarefas WHERE id = :id", params! { "id" => id }).unwrap();

    rocket::response::Redirect::to("/")
}

#[derive(rocket::form::FromForm)]
struct Tarefa {
    titulo: String,
    descricao: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, adicionar_tarefa, editar_tarefa, excluir_tarefa])
}
