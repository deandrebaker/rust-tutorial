use std::pin::pin;
use std::thread;
use std::time::Duration;
use trpl::{self, ReceiverStream, Stream, StreamExt};

fn main() {
    {
        println!("\nPart 4\n");

        trpl::run(async {
            let messages = get_message().timeout(Duration::from_millis(200));
            let intervals = get_intervals()
                .map(|count| format!("Interval: {count}"))
                .throttle(Duration::from_millis(100))
                .timeout(Duration::from_secs(10));
            let merged = messages.merge(intervals).take(20);
            let mut stream = pin!(merged);

            while let Some(message) = stream.next().await {
                match message {
                    Ok(message) => println!("{message}"),
                    Err(reason) => eprintln!("Problem: {reason:?}"),
                };
            }
        });
    }

    {
        let (tx, mut rx) = trpl::channel();

        thread::spawn(move || {
            for i in 1..11 {
                tx.send(i).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        trpl::run(async {
            while let Some(message) = rx.recv().await {
                println!("{message}");
            }
        })
    }
}

fn get_message() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{}'", message)) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    thread::spawn(move || {
        let mut count = 0;
        loop {
            thread::sleep(Duration::from_millis(1));
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Cannot send interval '{count}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}
