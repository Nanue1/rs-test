use std::time::Duration;

use garcon::{Delay, Waiter};

fn main() {
    println!("Hello, world!");
    can_send_between_threads()
}

#[allow(dead_code)]
fn can_send_between_threads() {
    let mut waiter = Delay::count_timeout(5);
    let (tx, rx) = std::sync::mpsc::channel();
    let (tx_end, rx_end) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        waiter.start();

        while let Some(x) = rx.recv().unwrap_or(None) {
            println!("X: {}",x); 
            for _i in 1..x {
                println!("{}", _i);
                waiter.wait().unwrap(); // 大于 5 次 ,就会超时
            }
        }

        tx_end.send(1000).unwrap();
    });

    tx.send(Some(6)).unwrap();
    tx.send(None).unwrap();

    let y = rx_end.recv().unwrap();
    println!("rx_end channel recv:{}",y)

}

#[allow(dead_code)]
async fn works_as_async() {
    let mut waiter = Delay::timeout(Duration::from_millis(50));
    let (tx, mut rx) = tokio::sync::mpsc::channel(5);
    let (tx_end, mut rx_end) = tokio::sync::mpsc::channel(1);

    tokio::task::spawn(async move {
        waiter.start();

        while let Some(x) = rx.recv().await.unwrap_or(None) {
            for _i in 1..x {
                waiter.async_wait().await.unwrap();
            }
        }

        tx_end.send(()).await.unwrap();
    });

    tx.send(Some(4)).await.unwrap();
    tx.send(Some(1)).await.unwrap();
    tx.send(None).await.unwrap();

    rx_end.recv().await.unwrap();
}
