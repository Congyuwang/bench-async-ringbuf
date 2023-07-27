#![feature(test)]
extern crate test;
use test::Bencher;
use async_ringbuf::traits::Split;
use futures::{AsyncReadExt, AsyncWriteExt};

const BUF_SIZE: usize = 8 * 1024;
const MSG_SIZE: usize = 128;
const MSG_COUNT: usize = 1024 * 1024;

#[bench]
fn bytes_stream(b: &mut Bencher) {
    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
    b.iter(|| {
        rt.block_on(send_receive());
    });
}

async fn send_receive() {

    let (mut prd, mut cons) = async_ringbuf::AsyncHeapRb::<u8>::new(BUF_SIZE).split();

    // prd task
    let prd_task = tokio::spawn(async move {
        let msg = [0u8; MSG_SIZE];
        for _ in 0..MSG_COUNT {
            prd.write_all(&msg).await.unwrap();
        }
    });

    // cons task
    let cons_task = tokio::spawn(async move {
        let mut buf = [0u8; MSG_SIZE];
        for _ in 0..MSG_COUNT {
            cons.read_exact(&mut buf).await.unwrap();
        }
    });

    // await finish
    prd_task.await.unwrap();
    cons_task.await.unwrap();
}
