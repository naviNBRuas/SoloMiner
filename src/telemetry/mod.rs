use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use tokio::sync::RwLock;
use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerStats {
    pub current_hashrate: u64,
    pub average_hashrate: f64,
    pub peak_hashrate: u64,
    pub total_hashes: u64,
    pub blocks_found: u64,
    pub uptime_seconds: u64,
    pub efficiency: f64,
}

#[derive(Default, Debug)]
pub struct MinerMetrics {
    pub status: RwLock<String>,
    pub hashrate: AtomicU64,
    pub total_hashes: AtomicU64,
    pub blocks_found: AtomicU64,
    pub start_time: Option<Instant>,
    pub is_mining: AtomicBool,
    pub hash_history: RwLock<Vec<(Instant, u64)>>,
}

#[derive(serde::Serialize)]
pub struct MetricsSnapshot {
    pub status: String,
    pub hashrate: u64,
    pub total_hashes: u64,
    pub blocks_found: u64,
    pub uptime: u64,
    pub efficiency: f64,
    pub average_hashrate: f64,
    pub peak_hashrate: u64,
    pub timestamp: u64,
}

impl MinerMetrics {
    pub fn new() -> Self {
        Self {
            status: RwLock::new("Idle".to_string()),
            hashrate: AtomicU64::new(0),
            total_hashes: AtomicU64::new(0),
            blocks_found: AtomicU64::new(0),
            start_time: Some(Instant::now()),
            is_mining: AtomicBool::new(false),
            hash_history: RwLock::new(Vec::new()),
        }
    }
    
    pub async fn snapshot(&self) -> MetricsSnapshot {
        let status = self.status.read().await.clone();
        let current_hashrate = self.hashrate.load(Ordering::Relaxed);
        let total_hashes = self.total_hashes.load(Ordering::Relaxed);
        let blocks_found = self.blocks_found.load(Ordering::Relaxed);
        let uptime = self.start_time.unwrap_or_else(Instant::now).elapsed().as_secs();
        
        // Calculate efficiency (blocks per billion hashes)
        let efficiency = if total_hashes > 0 {
            (blocks_found as f64 / total_hashes as f64) * 1_000_000_000.0
        } else {
            0.0
        };
        
        // Calculate average hashrate
        let history = self.hash_history.read().await;
        let average_hashrate = if !history.is_empty() {
            let sum: u64 = history.iter().map(|(_, rate)| rate).sum();
            sum as f64 / history.len() as f64
        } else {
            current_hashrate as f64
        };
        
        // Calculate peak hashrate
        let peak_hashrate = history.iter()
            .map(|(_, rate)| *rate)
            .max()
            .unwrap_or(current_hashrate);
        
        MetricsSnapshot {
            status,
            hashrate: current_hashrate,
            total_hashes,
            blocks_found,
            uptime,
            efficiency,
            average_hashrate,
            peak_hashrate,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    pub async fn record_hashrate(&self, rate: u64) {
        let now = Instant::now();
        let mut history = self.hash_history.write().await;
        
        // Keep only last 100 measurements
        if history.len() >= 100 {
            history.remove(0);
        }
        
        history.push((now, rate));
    }
    
    pub fn set_mining_status(&self, is_mining: bool) {
        self.is_mining.store(is_mining, Ordering::Relaxed);
    }
    
    pub async fn get_stats(&self) -> MinerStats {
        let snapshot = self.snapshot().await;
        MinerStats {
            current_hashrate: snapshot.hashrate,
            average_hashrate: snapshot.average_hashrate,
            peak_hashrate: snapshot.peak_hashrate,
            total_hashes: snapshot.total_hashes,
            blocks_found: snapshot.blocks_found,
            uptime_seconds: snapshot.uptime,
            efficiency: snapshot.efficiency,
        }
    }
}

#[get("/api/metrics")]
async fn get_metrics(metrics: web::Data<Arc<MinerMetrics>>) -> impl Responder {
    let snapshot = metrics.snapshot().await;
    HttpResponse::Ok().json(&snapshot)
}

#[get("/api/stats")]
async fn get_stats(metrics: web::Data<Arc<MinerMetrics>>) -> impl Responder {
    let stats = metrics.get_stats().await;
    HttpResponse::Ok().json(&stats)
}

#[post("/api/control/start")]
async fn start_mining(metrics: web::Data<Arc<MinerMetrics>>) -> impl Responder {
    *metrics.status.write().await = "Mining".to_string();
    metrics.set_mining_status(true);
    HttpResponse::Ok().json(serde_json::json!({"status": "started"}))
}

#[post("/api/control/stop")]
async fn stop_mining(metrics: web::Data<Arc<MinerMetrics>>) -> impl Responder {
    *metrics.status.write().await = "Stopped".to_string();
    metrics.set_mining_status(false);
    HttpResponse::Ok().json(serde_json::json!({"status": "stopped"}))
}

#[get("/")]
async fn index(metrics: web::Data<Arc<MinerMetrics>>) -> impl Responder {
    let snapshot = metrics.snapshot().await;
    let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>Lonely Solo Miner Dashboard</title>
    <meta charset="utf-8">
    <style>
        body {{ font-family: monospace; background: #1a1a1a; color: #00ff00; margin: 0; padding: 20px; }}
        .container {{ max-width: 800px; margin: 0 auto; }}
        .header {{ text-align: center; margin-bottom: 30px; }}
        .stats-grid {{ display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; }}
        .stat-card {{ background: #2a2a2a; border: 1px solid #00ff00; padding: 15px; border-radius: 5px; }}
        .stat-value {{ font-size: 2em; font-weight: bold; color: #ffff00; }}
        .stat-label {{ color: #888888; }}
        h1 {{ color: #00ff00; text-shadow: 0 0 10px #00ff00; }}
        .progress {{ width: 100%; height: 20px; background: #333; border-radius: 10px; overflow: hidden; margin: 10px 0; }}
        .progress-bar {{ height: 100%; background: linear-gradient(90deg, #00ff00, #ffff00); width: {:.1}%; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>⛏️ Lonely Solo Miner Dashboard</h1>
            <p>Real-time mining statistics and performance metrics</p>
        </div>
        
        <div class="stats-grid">
            <div class="stat-card">
                <div class="stat-label">STATUS</div>
                <div class="stat-value">{}</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-label">HASHRATE</div>
                <div class="stat-value">{{:,}} H/s</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-label">TOTAL HASHES</div>
                <div class="stat-value">{{:,}}</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-label">BLOCKS FOUND</div>
                <div class="stat-value">{}</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-label">UPTIME</div>
                <div class="stat-value">{}:{:02}:{:02}</div>
            </div>
            
            <div class="stat-card">
                <div class="stat-label">EFFICIENCY</div>
                <div class="stat-value">{:.2} BPB</div>
                <div class="progress">
                    <div class="progress-bar" style="width: {:.1}%"></div>
                </div>
            </div>
        </div>
        
        <div style="margin-top: 30px; text-align: center; color: #888;">
            <p>Last updated: Never</p>
            <p>Mining in splendid isolation since eternity</p>
        </div>
    </div>
</body>
</html>"#,
        (snapshot.efficiency / 10.0).min(100.0),
        snapshot.status,
        snapshot.hashrate,
        snapshot.total_hashes,
        snapshot.blocks_found,
        snapshot.uptime / 3600,
        (snapshot.uptime % 3600) / 60,
        snapshot.uptime % 60,


    );
    
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn start_dashboard(metrics: Arc<MinerMetrics>, port: u16) -> std::io::Result<()> {
    println!("🚀 Starting Lonely Solo Miner Dashboard on port {}", port);
    println!("🌐 Access dashboard at: http://localhost:{}", port);
    println!("📊 API endpoint: http://localhost:{}/api/metrics", port);
    println!("📈 Stats endpoint: http://localhost:{}/api/stats", port);
    println!("🔄 Press Ctrl+C to stop the dashboard\n");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(metrics.clone()))
            .service(index)
            .service(get_metrics)
            .service(get_stats)
            .service(start_mining)
            .service(stop_mining)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
