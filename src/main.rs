use async_ringbuf::traits::Split;
use futures::{AsyncReadExt, AsyncWriteExt};

const BUF_SIZE: usize = 8 * 1024;
const MSG_SIZE: usize = 64;
const MSG_COUNT: usize = 1024 * 1024;

#[tokio::main]
async fn main() {
    let (mut prd, mut cons) = async_ringbuf::AsyncHeapRb::<u8>::new(BUF_SIZE).split();

    let start_time = std::time::Instant::now();

    // prd task
    let prd_task = tokio::spawn(async move {
        let start_time = std::time::Instant::now();
        let msg = [0u8; MSG_SIZE];
        for _ in 0..MSG_COUNT {
            prd.write_all(&msg).await.unwrap();
        }
        let elapsed = start_time.elapsed();
        println!(
            "prd: {} milliseconds, {} msg, {} bytes, {} msg/s, {} MB/s",
            elapsed.as_millis(),
            MSG_COUNT,
            MSG_COUNT * MSG_SIZE,
            MSG_COUNT as f64 / elapsed.as_secs_f64(),
            MSG_COUNT as f64 * MSG_SIZE as f64 / 1024.0 / 1024.0 / elapsed.as_secs_f64()
        );
    });

    // cons task
    let cons_task = tokio::spawn(async move {
        let start_time = std::time::Instant::now();
        let mut buf = [0u8; MSG_SIZE];
        for _ in 0..MSG_COUNT {
            cons.read_exact(&mut buf).await.unwrap();
        }
        let elapsed = start_time.elapsed();
        println!(
            "cons: {} milliseconds, {} msg, {} bytes, {} msg/s, {} MB/s",
            elapsed.as_millis(),
            MSG_COUNT,
            MSG_COUNT * MSG_SIZE,
            MSG_COUNT as f64 / elapsed.as_secs_f64(),
            MSG_COUNT as f64 * MSG_SIZE as f64 / 1024.0 / 1024.0 / elapsed.as_secs_f64()
        );
    });

    prd_task.await.unwrap();
    cons_task.await.unwrap();
    let total_elapsed = start_time.elapsed();
    println!(
        "total time: {} milliseconds, total bytes: {} GB",
        total_elapsed.as_millis(),
        MSG_COUNT * MSG_SIZE / 1024 / 1024 / 1024
    );
}
