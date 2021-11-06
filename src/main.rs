#[macro_use] extern crate rocket;

#[get("/")]
fn root() -> &'static str {
    "Hello, world! I am comming from Rocket Rust"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!",name)
}

// #[post("/todo", data: <todo>)]
// fn hello(name: String) -> String {
//     format!("Hello, {}!",name)
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root,hello])
}