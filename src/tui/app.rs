use std::sync::Arc;
use crate::telemetry::MinerMetrics;
use sysinfo::{System, CpuRefreshKind, MemoryRefreshKind};
use chrono::{DateTime, Local};
use rand::Rng;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CurrentScreen {
    Main,
    Miners,
    Logs,
    Settings,
}

pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub life: f64,
    pub max_life: f64,
    pub char: char,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x,
            y,
            vx: rng.gen_range(-1.0..1.0),
            vy: rng.gen_range(-2.0..-0.5),
            life: 1.0,
            max_life: rng.gen_range(0.5..2.0),
            char: ['*', '+', '.', 'o'][rng.gen_range(0..4)],
        }
    }
    
    pub fn update(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.life -= 0.02;
    }
    
    pub fn is_dead(&self) -> bool {
        self.life <= 0.0
    }
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub metrics: Arc<MinerMetrics>,
    pub system: System,
    pub should_quit: bool,
    pub cpu_usage_history: Vec<f64>,
    pub hashrate_history: Vec<f64>,
    pub particles: Vec<Particle>,
    pub last_block_time: Option<DateTime<Local>>,
    pub animation_frame: usize,
    pub start_time: Instant,
}

impl App {
    pub fn new(metrics: Arc<MinerMetrics>) -> App {
        let mut system = System::new_all();
        system.refresh_all();
        App {
            current_screen: CurrentScreen::Main,
            metrics,
            system,
            should_quit: false,
            cpu_usage_history: Vec::with_capacity(100),
            hashrate_history: Vec::with_capacity(100),
            particles: Vec::new(),
            last_block_time: None,
            animation_frame: 0,
            start_time: Instant::now(),
        }
    }

    pub async fn tick(&mut self) {
        // Refresh system information
        self.system.refresh_cpu_specifics(CpuRefreshKind::everything());
        self.system.refresh_memory_specifics(MemoryRefreshKind::everything());
        
        let avg_cpu = self.system.global_cpu_info().cpu_usage() as f64;
        self.cpu_usage_history.push(avg_cpu);
        if self.cpu_usage_history.len() > 100 {
            self.cpu_usage_history.remove(0);
        }

        let snapshot = self.metrics.snapshot().await;
        self.hashrate_history.push(snapshot.hashrate as f64);
        if self.hashrate_history.len() > 100 {
            self.hashrate_history.remove(0);
        }
        
        // Handle block found events for particle effects
        if snapshot.blocks_found > 0 {
            if self.last_block_time.is_none() || 
               self.last_block_time.as_ref().unwrap().timestamp() < 
               Local::now().timestamp() - 5 {  // New block in last 5 seconds
                self.create_mining_particles();
                self.last_block_time = Some(Local::now());
            }
        }
        
        // Update particles
        self.particles.retain_mut(|p| {
            p.update();
            !p.is_dead()
        });
        
        // Animation frame counter
        self.animation_frame = (self.animation_frame + 1) % 1000;
    }
    
    fn create_mining_particles(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..20 {
            let x = rng.gen_range(10.0..70.0);
            let y = rng.gen_range(5.0..15.0);
            self.particles.push(Particle::new(x, y));
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' | 'Q' => self.should_quit = true,
            '1' => self.current_screen = CurrentScreen::Main,
            '2' => self.current_screen = CurrentScreen::Miners,
            '3' => self.current_screen = CurrentScreen::Logs,
            '4' => self.current_screen = CurrentScreen::Settings,
            'r' | 'R' => {
                // Reset statistics
                self.cpu_usage_history.clear();
                self.hashrate_history.clear();
            },
            _ => {}
        }
    }
    
    pub fn uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
    
    pub fn formatted_uptime(&self) -> String {
        let secs = self.uptime_seconds();
        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}
