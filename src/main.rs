pub mod sensor_reading;

use actix_web::{cookie, HttpServer, App, web, HttpResponse, Responder};
// use actix_files::Files;
// use actix_web::middleware::Logger;
// use actix_rt;
use std::sync::Mutex;

use tera::{Tera, Context};
use lazy_static::lazy_static;
use sensor_readings::{read_temp, read_humidity};


lazy_static!{
    pub static ref TEMPLATES: Tera = {
      let mut tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
          println!("Parsing error(s): {}", e);
          ::std::process::exit(1);
        }
      };
      tera.autoescape_on(vec![".html", ".sql"]);
      tera
    };
}


async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Control Panel");
    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}


#[tokio::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new ( move || {
        App::new()
            .service(Files::new("/assets", "./assets").prefer_utf8(true))
            .app_data(web::Data::new(TEMPLATES.clone()))
            .app_data( robot_address.clone()) 
            .route("/",web::get().to(index))
            .route("/gettemperature", web::get().to(read_temp))
            .route("/gethumidity",web::get().to(read_humidity))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await

}