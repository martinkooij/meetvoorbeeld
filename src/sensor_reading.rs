
use actix_web::{HttpResponse, Responder};

pub async fn read_temp () -> impl Responder {
    
    let temp = 21 ;


    println!("Temp reading is: {:?}", temp);
  
    
    HttpResponse::Ok().json(temp)

}


pub async fn read_humidity () -> impl Responder {
    
    let hum = 42 ;


    println!("Humidity reading is: {:?}", hum);
  
    
    HttpResponse::Ok().json(hum)

}