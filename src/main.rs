use tokio::select;
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;
#[tokio::main]
async fn main() {
    for i in 1..=3 {
        let (tx, mut rx) = mpsc::channel::<()>(1);

        tokio::spawn(async move {
            loop {
                select! {
                    _ = sleep(Duration::from_millis(1500)) => {
                        println!("Task {i} was did!");
                    }
                    _ = rx.recv() => {
                        println!("{i} task is ending a work!");
                        break;
                    }
                }
            }
            println!("End {i} task");
        });
    }
    println!("A tasks's starting do work!");
    tokio::signal::ctrl_c();
}
