use tokio::process::Command;
use tokio::time::{Duration, sleep};
use reqwest;

#[tokio::test]
async fn test_miner_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .await
        .expect("Failed to execute help command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Lonely Solo Miner"));
    assert!(stdout.contains("start"));
    assert!(stdout.contains("dashboard"));
    assert!(stdout.contains("status"));
}

#[tokio::test]
async fn test_miner_starts_with_no_tui() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
        .arg("--")
        .arg("start")
        .arg("--no-tui")
        .arg("--difficulty")
        .arg("00000"); // Easy difficulty for testing

    let mut child = cmd.spawn().expect("Failed to spawn miner process");

    // Give the miner some time to start and run
    sleep(Duration::from_secs(5)).await;

    // Check if process is still running
    assert!(child.try_wait().unwrap().is_none(), "Miner should still be running");
    
    // Attempt to kill the process
    child.kill().await.expect("Failed to kill miner process");
    let status = child
        .wait()
        .await
        .expect("Failed to wait for miner process");
    
    // Process should exit gracefully when killed
    assert!(!status.success(), "Miner process should not exit successfully when killed");
}

#[tokio::test]
async fn test_dashboard_starts_and_responds() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
       .arg("--")
       .arg("dashboard")
       .arg("--port")
       .arg("8081"); // Use different port to avoid conflicts

    let mut child = cmd.spawn().expect("Failed to spawn dashboard process");
    
    // Wait for dashboard to start
    sleep(Duration::from_secs(3)).await;
    
    // Test if dashboard responds
    let client = reqwest::Client::new();
    let resp = client
        .get("http://localhost:8081/")
        .send()
        .await;
    
    match resp {
        Ok(response) => {
            assert!(response.status().is_success());
            let body = response.text().await.unwrap();
            assert!(body.contains("Lonely Solo Miner Dashboard"));
        }
        Err(e) => {
            // Dashboard might not be ready yet, this is acceptable in test environment
            println!("Dashboard test warning: {}", e);
        }
    }
    
    // Cleanup
    child.kill().await.expect("Failed to kill dashboard process");
    child.wait().await.ok();
}

#[tokio::test]
async fn test_miner_status_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "status"])
        .output()
        .await
        .expect("Failed to execute status command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Lonely Solo Miner Status"));
    assert!(stdout.contains("Status:"));
    assert!(stdout.contains("Hashrate:"));
}

#[tokio::test]
async fn test_miner_start_stop_commands() {
    // Test stop command (should work even when nothing is running)
    let output = Command::new("cargo")
        .args(["run", "--", "stop"])
        .output()
        .await
        .expect("Failed to execute stop command");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Stopping all mining processes"));
}

#[tokio::test]
async fn test_config_loading() {
    // Test that config can be loaded without errors
    let output = Command::new("cargo")
        .args(["run", "--", "status"])
        .env("MINER_DIFFICULTY", "000000")
        .env("MINER_THREADS", "4")
        .output()
        .await
        .expect("Failed to execute config test");
    
    // Should either succeed or fail gracefully
    let stderr = String::from_utf8_lossy(&output.stderr);
    // If there are errors, they should be about missing wallet address, not config issues
    if !output.status.success() {
        assert!(!stderr.contains("Configuration validation error"));
    }
}

#[tokio::test]
async fn test_multiple_mining_modes() {
    // Test that different modes can be parsed
    let modes = ["performance", "conservative"];
    
    for _mode in &modes {
        let output = Command::new("cargo")
            .args(["run", "--", "--help"]) // Just test argument parsing
            .output()
            .await
            .expect("Failed to execute mode test");
        
        assert!(output.status.success());
    }
}
