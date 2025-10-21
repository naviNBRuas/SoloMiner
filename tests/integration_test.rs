use tokio::process::Command;
use tokio::time::{sleep, Duration};
use reqwest;

#[tokio::test]
async fn test_sha256_miner_starts_and_stops() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").arg("start").arg("--algorithm").arg("sha256").arg("--mode").arg("conservative");
    
    let mut child = cmd.spawn().expect("Failed to spawn miner process");

    // Give the miner some time to start and run
    sleep(Duration::from_secs(5)).await;

    // Attempt to kill the process. If it panics, the test will fail.
    child.kill().await.expect("Failed to kill miner process");
    let status = child.wait().await.expect("Failed to wait for miner process");
    assert!(!status.success(), "Miner process should not exit successfully when killed");
}

#[tokio::test]
async fn test_randomx_miner_starts_and_stops() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").arg("start").arg("--algorithm").arg("random-x").arg("--mode").arg("conservative");
    
    let mut child = cmd.spawn().expect("Failed to spawn miner process");

    // Give the miner some time to start and run
    sleep(Duration::from_secs(5)).await;

    // Attempt to kill the process. If it panics, the test will fail.
    child.kill().await.expect("Failed to kill miner process");
    let status = child.wait().await.expect("Failed to wait for miner process");
    assert!(!status.success(), "Miner process should not exit successfully when killed");
}

#[tokio::test]
async fn test_dashboard_starts_and_responds() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").arg("dashboard");
    
    let mut child = cmd.spawn().expect("Failed to spawn dashboard process");

    // Give the dashboard some time to start
    sleep(Duration::from_secs(5)).await;

    // Implement a retry mechanism for connecting to the dashboard
    let client = reqwest::Client::new();
    let mut attempts = 0;
    let max_attempts = 20;
    let mut connected = false;

    while attempts < max_attempts {
        sleep(Duration::from_secs(1)).await;
        match client.get("http://127.00.1:8080/").send().await {
            Ok(res) if res.status().is_success() => {
                connected = true;
                break;
            }
            _ => {
                attempts += 1;
            }
        }
    }

    assert!(connected, "Dashboard did not become available after multiple attempts");

    // Kill the dashboard process
    child.kill().await.expect("Failed to kill dashboard process");
    let status = child.wait().await.expect("Failed to wait for dashboard process");
    assert!(!status.success(), "Dashboard process should not exit successfully when killed");
}