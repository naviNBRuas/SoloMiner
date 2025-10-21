use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default, Debug, Clone)]
pub struct MinerMetrics {
    pub status: String,
    pub hashrate: f64,
    pub total_hashes: u64,
    pub blocks_found: u64,
}

#[get("/metrics")]
async fn get_metrics(metrics: web::Data<Arc<Mutex<MinerMetrics>>>) -> impl Responder {
    let metrics = metrics.lock().await;
    HttpResponse::Ok().json(&*metrics)
}

#[get("/")]
async fn index(metrics: web::Data<Arc<Mutex<MinerMetrics>>>) -> impl Responder {
    let metrics = metrics.lock().await;
    HttpResponse::Ok().body(format!(
        "<h1>SoloMiner Dashboard</h1>
        <p>Status: {}</p>
        <p>Hashrate: {:.2} hashes/s</p>
        <p>Total Hashes: {}</p>
        <p>Blocks Found: {}</p>",
        metrics.status, metrics.hashrate, metrics.total_hashes, metrics.blocks_found
    ))
}

pub async fn start_dashboard(metrics: Arc<Mutex<MinerMetrics>>) -> std::io::Result<()> {
    println!("Starting web dashboard on http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(metrics.clone()))
            .service(index)
            .service(get_metrics)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
