use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("==== Start async log collector ====");

    std::fs::write(
        "agent.log",
        "INFO 192.168.0.1 - Access Granted\nERROR 500 - Internal Server Error\nINFO 192.168.0.2 - Path /index.html\nERROR 403 - Forbidden\n"
    ).unwrap();

    let log_streamer = tokio::spawn(async {
        let log_file = File::open("agent.log").await.unwrap();
        let mut reader = BufReader::new(log_file);
        
        let mut line = String::new();
        let mut error_count = 0;

        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line).await.unwrap();
            if bytes_read == 0 {
                break
            }

            if line.contains("ERROR") {
                error_count += 1;
                println!("🚨 [Stream Task] 에러 로그 발견: {}", line.trim());
            }

            sleep(Duration::from_millis(10)).await;
        }
        error_count
    });

    let heart_beat = tokio::spawn(async {
        for i in 0..=3 {
            println!("[Heartbeat Task] Sending heartbeat signals to the agent: {}/3", i+1);
            sleep(Duration::from_millis(150)).await;
        }
    });

    let error_count = log_streamer.await.unwrap();
    let _ = heart_beat.await.unwrap();

    println!("=== Complete ===");
    println!("Total errors found: {}", error_count);
}