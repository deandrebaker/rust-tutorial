use std::pin::{Pin, pin};
use std::thread;
use std::time::{Duration, Instant};
use trpl::{self, Either};

fn main() {
    {
        println!("\nPart 1\n");

        trpl::run(async {
            let (tx, mut rx) = trpl::channel();

            let tx1 = tx.clone();
            let tx1_fut = async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let rx_fut = async {
                while let Some(value) = rx.recv().await {
                    println!("Got: {value}");
                }
            };

            let tx_fut = async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(1500)).await;
                }
            };

            trpl::join!(tx1_fut, tx_fut, rx_fut);
        });
    }

    {
        println!("\nPart 2\n");

        trpl::run(async {
            let (tx, mut rx) = trpl::channel();

            let tx1 = tx.clone();
            let tx1_fut = pin!(async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            let rx_fut = pin!(async {
                while let Some(value) = rx.recv().await {
                    println!("Got: {value}");
                }
            });

            let tx_fut = pin!(async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(1500)).await;
                }
            });

            let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

            trpl::join_all(futures).await;
        });
    }

    {
        println!("\nPart 3\n");

        trpl::run(async {
            let a = async { 1u32 };
            let b = async { "Hello!" };
            let c = async { true };

            let (a_result, b_result, c_result) = trpl::join!(a, b, c);
            println!("a: {a_result}, b: {b_result}, c: {c_result}");
        })
    }

    {
        println!("\nPart 4\n");

        trpl::run(async {
            let slow = async {
                println!("Starting slow task...");
                trpl::sleep(Duration::from_millis(100)).await;
                println!("Slow task done!");
            };

            let fast = async {
                println!("Starting fast task...");
                trpl::sleep(Duration::from_millis(50)).await;
                println!("fast task done!");
            };

            trpl::race(slow, fast).await;
        });
    }

    {
        println!("\nPart 5\n");

        trpl::run(async {
            let a = async {
                println!("'a' started.");
                slow("a", 30);
                slow("a", 10);
                slow("a", 20);
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'a' finished.");
            };

            let b = async {
                println!("'b' started.");
                slow("b", 75);
                slow("b", 10);
                slow("b", 15);
                slow("b", 350);
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'b' finished.");
            };

            trpl::race(a, b).await;
        });
    }

    {
        println!("\nPart 6\n");

        trpl::run(async {
            let a = async {
                println!("'a' started.");
                slow("a", 30);
                trpl::yield_now().await;
                slow("a", 10);
                trpl::yield_now().await;
                slow("a", 20);
                trpl::yield_now().await;
                println!("'a' finished.");
            };

            let b = async {
                println!("'b' started.");
                slow("b", 75);
                trpl::yield_now().await;
                slow("b", 10);
                trpl::yield_now().await;
                slow("b", 15);
                trpl::yield_now().await;
                slow("b", 350);
                trpl::yield_now().await;
                println!("'b' finished.");
            };

            trpl::race(a, b).await;
        });
    }

    {
        println!("\nPart 7\n");

        trpl::run(async {
            let one_ns = Duration::from_nanos(1);
            let start = Instant::now();
            async {
                for _ in 1..1000 {
                    trpl::sleep(one_ns).await;
                }
            }
            .await;

            let time = Instant::now() - start;
            println!(
                "'sleep' version finished in {} seconds.",
                time.as_secs_f32()
            );

            let start = Instant::now();
            async {
                for _ in 1..1000 {
                    trpl::yield_now().await;
                }
            }
            .await;

            let time = Instant::now() - start;
            println!(
                "'yield_now' version finished in {} seconds.",
                time.as_secs_f32()
            );
        });
    }

    {
        println! {"\nPart 8\n"};

        trpl::run(async {
            let slow = async {
                trpl::sleep(Duration::from_millis(1)).await;
                "I finished!"
            };

            match timeout(slow, Duration::from_millis(10)).await {
                Ok(message) => println!("Succeeded with '{message}"),
                Err(duration) => println!("Failed after {} seconds", duration.as_secs()),
            };
        });
    }
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{}' ran for {}ms!", name, ms);
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    let timer = trpl::sleep(max_time);
    let result = trpl::race(future_to_try, timer).await;
    match result {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
