#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template,context};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { title: "Rustyque System" })
}

#[get("/welcome")]
fn welcome() -> Template {
    Template::render("welcome", context! { title: "Rustyque System" })
}

#[get("/get_number")]
fn get_number() -> Template {
    Template::render("get_number", context! { title: "Rustyque System" })
}

#[get("/counter")]
fn counter() -> Template {
    Template::render("counter", context! { title: "Rustyque System" })
}

#[get("/call_next")]
fn call_next() -> Template {
    Template::render("call_next", context! { title: "Rustyque System" })
}

#[get("/display")]
fn display() -> Template {
    Template::render("display", context! { title: "Rustyque System" })
}

#[get("/update_display")]
fn update_display() -> Template {
    Template::render("update_display", context! { title: "Rustyque System" })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, welcome, get_number, counter, call_next, display, update_display])
        .attach(Template::fairing())
}
