# Lonely Solo Miner API Documentation

## RESTful API Endpoints

### GET `/`
Returns the main web dashboard HTML page with real-time mining statistics.

**Response:**
```html
<!DOCTYPE html>
<html>
<head>
    <title>Lonely Solo Miner Dashboard</title>
    <!-- Embedded CSS and JavaScript -->
</head>
<body>
    <!-- Interactive dashboard with live metrics -->
</body>
</html>
```

### GET `/api/metrics`
Returns current mining metrics in JSON format.

**Response:**
```json
{
  "status": "Mining",
  "hashrate": 1250000,
  "total_hashes": 45872934,
  "blocks_found": 2,
  "uptime": 3600,
  "efficiency": 0.0436,
  "average_hashrate": 1180000.5,
  "peak_hashrate": 1450000,
  "timestamp": 1705737600
}
```

### GET `/api/stats`
Returns aggregated mining statistics.

**Response:**
```json
{
  "current_hashrate": 1250000,
  "average_hashrate": 1180000.5,
  "peak_hashrate": 1450000,
  "total_hashes": 45872934,
  "blocks_found": 2,
  "uptime_seconds": 3600,
  "efficiency": 0.0436
}
```

### POST `/api/control/start`
Starts the mining process.

**Request:**
```http
POST /api/control/start HTTP/1.1
Content-Type: application/json
```

**Response:**
```json
{
  "status": "started"
}
```

### POST `/api/control/stop`
Stops the mining process.

**Request:**
```http
POST /api/control/stop HTTP/1.1
Content-Type: application/json
```

**Response:**
```json
{
  "status": "stopped"
}
```

## WebSocket Support (Planned)

Future versions will include real-time WebSocket connections for live metric updates.

### WebSocket Endpoint: `/ws/metrics`

**Connection:**
```javascript
const ws = new WebSocket('ws://localhost:8080/ws/metrics');

ws.onmessage = function(event) {
    const metrics = JSON.parse(event.data);
    // Update UI in real-time
    console.log('New metrics:', metrics);
};
```

## Data Structures

### MetricsSnapshot
```rust
struct MetricsSnapshot {
    status: String,          // Current mining status
    hashrate: u64,          // Current hashrate (H/s)
    total_hashes: u64,      // Total hashes computed
    blocks_found: u64,      // Number of blocks discovered
    uptime: u64,            // Seconds since start
    efficiency: f64,        // Blocks per billion hashes
    average_hashrate: f64,  // Average hashrate
    peak_hashrate: u64,     // Highest recorded hashrate
    timestamp: u64,         // Unix timestamp
}
```

### MinerStats
```rust
struct MinerStats {
    current_hashrate: u64,
    average_hashrate: f64,
    peak_hashrate: u64,
    total_hashes: u64,
    blocks_found: u64,
    uptime_seconds: u64,
    efficiency: f64,
}
```

## Rate Limiting

The API implements reasonable rate limiting to prevent abuse:
- `/api/metrics`: 10 requests per second per IP
- `/api/stats`: 5 requests per second per IP
- Control endpoints: 1 request per second per IP

## Error Handling

### Standard Error Response
```json
{
  "error": "Descriptive error message",
  "code": 400,
  "timestamp": 1705737600
}
```

### Common HTTP Status Codes
- `200 OK`: Request successful
- `400 Bad Request`: Invalid parameters
- `404 Not Found`: Endpoint doesn't exist
- `429 Too Many Requests`: Rate limit exceeded
- `500 Internal Server Error`: Server-side error

## Authentication

Currently, the API does not require authentication. Future versions may include:
- API key authentication
- JWT token support
- OAuth2 integration

## Client Libraries

### JavaScript/Node.js Example
```javascript
class LonelySoloMinerClient {
    constructor(baseUrl = 'http://localhost:8080') {
        this.baseUrl = baseUrl;
    }

    async getMetrics() {
        const response = await fetch(`${this.baseUrl}/api/metrics`);
        return await response.json();
    }

    async getStats() {
        const response = await fetch(`${this.baseUrl}/api/stats`);
        return await response.json();
    }

    async startMining() {
        const response = await fetch(`${this.baseUrl}/api/control/start`, {
            method: 'POST'
        });
        return await response.json();
    }

    async stopMining() {
        const response = await fetch(`${this.baseUrl}/api/control/stop`, {
            method: 'POST'
        });
        return await response.json();
    }
}

// Usage
const client = new LonelySoloMinerClient();
const metrics = await client.getMetrics();
console.log(`Current hashrate: ${metrics.hashrate} H/s`);
```

### Python Example
```python
import requests
import json

class LonelySoloMinerClient:
    def __init__(self, base_url='http://localhost:8080'):
        self.base_url = base_url

    def get_metrics(self):
        response = requests.get(f'{self.base_url}/api/metrics')
        return response.json()

    def get_stats(self):
        response = requests.get(f'{self.base_url}/api/stats')
        return response.json()

    def start_mining(self):
        response = requests.post(f'{self.base_url}/api/control/start')
        return response.json()

    def stop_mining(self):
        response = requests.post(f'{self.base_url}/api/control/stop')
        return response.json()

# Usage
client = LonelySoloMinerClient()
metrics = client.get_metrics()
print(f"Current hashrate: {metrics['hashrate']} H/s")
```

### Rust Example
```rust
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct MetricsSnapshot {
    status: String,
    hashrate: u64,
    total_hashes: u64,
    blocks_found: u64,
    uptime: u64,
    efficiency: f64,
}

pub struct LonelySoloMinerClient {
    base_url: String,
    client: reqwest::Client,
}

impl LonelySoloMinerClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_metrics(&self) -> Result<MetricsSnapshot, reqwest::Error> {
        let url = format!("{}/api/metrics", self.base_url);
        let response = self.client.get(&url).send().await?;
        response.json().await
    }
}

// Usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = LonelySoloMinerClient::new("http://localhost:8080");
    let metrics = client.get_metrics().await?;
    println!("Current hashrate: {} H/s", metrics.hashrate);
    Ok(())
}
```

## Versioning

The API follows semantic versioning. Breaking changes will result in major version increments.

Current version: v1.0.0

## Changelog

### v1.0.0 (Initial Release)
- Basic metrics endpoints
- Control endpoints for start/stop
- Web dashboard
- JSON response format
- Rate limiting implementation

### Planned Features (v2.0.0)
- WebSocket support for real-time updates
- Authentication system
- Historical data endpoints
- Configuration management API
- Multi-user support