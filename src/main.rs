#![feature(decl_macro)]
#[macro_use] extern crate rocket;
use rocket::Request;
use rocket::response::content::Json;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use serde::Serialize;

// DATA STRUCTURES
#[derive(FromForm, Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String
}

// ROUTES
#[get("/")] // Index route for html
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
        first_name: String,
        last_name: String
    }
    let context = Context {
        first_name: String::from("Mark"),
        last_name: String::from("Tomlin")
    };
    Template::render("home", context)
}

#[get("/page2")] // Second html page
fn page2() -> Template {
    #[derive(Serialize)]
    struct Context {
        title: String,
        sub_title: String
    }
    let context = Context {
        title: String::from("Second page"),
        sub_title: String::from("Cool I guess 😎")
    };
    Template::render("page2", context)
}


#[catch(404)] // 404 error page
fn not_found(req: &Request) -> String { 
    print!("{}", req);
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[get("/hello")] // GET request to /hello route
fn hello() -> Json<&'static str> {
    Json("{
      'status': 'success',
      'message': 'Hello API!'
    }")
}

#[post("/book", data = "<book_form>")] // POST request for new Book on /book
fn new_book(book_form: Form<Book>) -> String {
    let book: Book = book_form.into_inner();
    let mut dummy_db: Vec<Book> = Vec::new();
    dummy_db.push(book);
    format!("Book added successfully: {:?}", dummy_db)
  }

// MAIN
fn main() {
    rocket::ignite()
      .register(catchers![not_found])
      .mount("/", routes![index, page2])
      .mount("/api", routes![hello, new_book])
      .attach(Template::fairing())
      .launch();
  }
