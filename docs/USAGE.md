# Lonely Solo Miner - Usage Guide

## 🚀 Quick Start

### Installation
```bash
# Clone the repository
git clone https://github.com/yourusername/lonely-solo-miner.git
cd lonely-solo-miner

# Build the project
cargo build --release

# Run the miner
./target/release/_lonely-solo-miner_ start
```

### Basic Usage

#### Start Mining with Beautiful TUI
```bash
_lonely-solo-miner_ start
```

#### Start Mining in Background Mode
```bash
_lonely-solo-miner_ start --no-tui
```

#### View Current Status
```bash
_lonely-solo-miner_ status
```

#### Start Web Dashboard
```bash
_lonely-solo-miner_ dashboard --port 8080
```

#### Stop All Mining Processes
```bash
_lonely-solo-miner_ stop
```

## ⚙️ Configuration

### Configuration File (`config.toml`)

```toml
[miner]
difficulty = "0000"     # Mining difficulty target
threads = 0            # Number of mining threads (0 = auto-detect)
algorithm = "sha256"   # Mining algorithm (sha256 or randomx)
batch_size = 10000     # Hashes per batch

[logging]
level = "info"         # Log level (trace, debug, info, warn, error)
format = "compact"     # Log format (compact, full, json)

[telemetry]
port = 8080            # Web dashboard port
enable_metrics = true   # Enable metrics collection
```

### Environment Variables

```bash
# Set wallet address
export WALLET_ADDRESS=your-wallet-address-here

# Configure mining parameters
export MINER_DIFFICULTY=00000
export MINER_THREADS=8
export MINER_ALGORITHM=randomx

# Configure telemetry
export TELEMETRY_PORT=9090
```

## 🎨 Terminal Interface Features

### Main Dashboard Screen
- **Animated Header**: Switching between different titles
- **Hashrate Gauge**: Real-time hashrate with dynamic coloring
- **CPU Usage Monitor**: System resource utilization
- **Loneliness Meter**: Emotional state indicator
- **Hashrate Trend Chart**: Historical performance visualization
- **System Information**: Memory usage, uptime, and temperature
- **Particle Effects**: Visual feedback when blocks are found

### Navigation
- `1` - Main Dashboard
- `2` - Mining Instances
- `3` - Logs
- `4` - Settings
- `q`/`Q` - Quit
- `r`/`R` - Reset Statistics

### Mining Instances Screen
View detailed information about all active mining threads:
- CPU SHA-256 miners
- CPU RandomX miners  
- GPU SHA-256 miners (simulated)
- GPU RandomX miners (simulated)
- Performance metrics for each instance

### Logs Screen
Real-time mining activity log with color-coded severity levels:
- INFO: General operational messages
- WARN: Warning conditions
- DEBUG: Detailed debugging information
- ERROR: Error conditions

### Settings Screen
Configure mining parameters on-the-fly:
- Difficulty adjustment
- Mining mode selection
- Wallet address management
- Theme customization

## 🌐 Web Dashboard

Access the web interface at `http://localhost:8080`

### Features
- **Real-time Statistics**: Live hashrate, blocks found, uptime
- **Performance Charts**: Interactive graphs and metrics
- **Responsive Design**: Works on desktop and mobile browsers
- **API Endpoints**: 
  - `/api/metrics` - JSON metrics data
  - `/api/stats` - Aggregated statistics
  - `/api/control/start` - Start mining (POST)
  - `/api/control/stop` - Stop mining (POST)

## 🧪 Testing and Benchmarking

### Run Unit Tests
```bash
cargo test
```

### Run Integration Tests
```bash
cargo test --test integration_test
```

### Run Performance Benchmarks
```bash
cargo bench
```

### Test Specific Components
```bash
# Test configuration loading
cargo test config_tests

# Test mining algorithms
cargo test miner_tests

# Test telemetry system
cargo test telemetry_tests
```

## 🛠️ Advanced Usage

### Custom Mining Modes
```bash
# Performance mode (uses all CPU cores)
_lonely-solo-miner_ start --mode performance

# Conservative mode (uses half CPU cores)
_lonely-solo-miner_ start --mode conservative
```

### Algorithm Selection
```bash
# SHA-256 algorithm (default)
_lonely-solo-miner_ start --algorithm sha256

# RandomX algorithm
_lonely-solo-miner_ start --algorithm randomx
```

### Custom Difficulty
```bash
# Easy difficulty for testing
_lonely-solo-miner_ start --difficulty 000

# Hard difficulty for serious mining
_lonely-solo-miner_ start --difficulty 0000000000000000
```

## 📊 Monitoring and Metrics

### Key Metrics Tracked
- **Hashrate**: Current and average hashing performance
- **Efficiency**: Blocks found per billion hashes attempted
- **Uptime**: Total mining duration
- **Resource Usage**: CPU and memory consumption
- **Block Discovery**: Successful mining events

### Performance Optimization Tips
1. Use `--mode performance` for maximum throughput
2. Adjust `threads` in config.toml based on your CPU
3. Monitor CPU temperature to prevent thermal throttling
4. Use SSD storage for better I/O performance
5. Close unnecessary applications to free up resources

## 🔧 Troubleshooting

### Common Issues

**High CPU Usage**
```bash
# Reduce mining intensity
_lonely-solo-miner_ start --mode conservative
```

**Low Hashrate**
```bash
# Check system resources
_lonely-solo-miner_ status

# Verify configuration
cat config.toml
```

**Web Dashboard Not Accessible**
```bash
# Check if port is available
netstat -tlnp | grep 8080

# Try different port
_lonely-solo-miner_ dashboard --port 9090
```

### Logging and Debugging
```bash
# Enable verbose logging
export RUST_LOG=debug
_lonely-solo-miner_ start

# Save logs to file
_lonely-solo-miner_ start 2>&1 | tee miner.log
```

## 🎯 Best Practices

1. **Start Small**: Begin with easy difficulty settings for testing
2. **Monitor Resources**: Keep an eye on CPU temperature and system stability
3. **Regular Updates**: Pull the latest version for performance improvements
4. **Backup Config**: Keep copies of your working configuration
5. **Community Engagement**: Share your experiences and optimizations

Remember: Mining cryptocurrencies consumes significant electricity. Always consider the environmental impact and costs before starting serious mining operations!