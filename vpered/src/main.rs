use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use std::collections::HashMap;

struct AppState {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads,
    clicks: Mutex<HashMap<String, usize>>
}


#[get("/click")]
async fn click(req: HttpRequest, app_state: web::Data<AppState>) -> impl Responder {
    let remote_addr = String::from(req.connection_info().realip_remote_addr().unwrap().split(":").next().unwrap());


    let mut counter = app_state.counter.lock().unwrap();
    *counter += 1;

    let mut response_text = format!("Vpereds: {}", counter);

    let mut clicks = app_state.clicks.lock().unwrap();
    let mut entry = clicks.entry(remote_addr).and_modify(|e| {*e += 1}).or_insert(1);
    for (address, n_clicks) in &(*clicks) {
        response_text = format!("{}\n{}: {}", response_text, address, n_clicks)
    }

    HttpResponse::Ok().body(response_text)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
        clicks: Mutex::new(HashMap::new()),
    });


    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(click)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
