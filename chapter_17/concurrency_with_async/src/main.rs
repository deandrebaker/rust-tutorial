use std::time::Duration;

fn main() {
    {
        println!("\nPart 1\n");

        trpl::run(async {
            trpl::spawn_task(async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
    }

    {
        println!("\nPart 2\n");
        trpl::run(async {
            let handle = trpl::spawn_task(async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }

            handle.await.unwrap();
        });
    }

    {
        println!("\nPart 3\n");
        trpl::run(async {
            let fut1 = async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let fut2 = async {
                for i in 1..5 {
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            trpl::join(fut1, fut2).await;
        });
    }

    {
        println!("\nPart 4\n");

        trpl::run(async {
            let (tx, mut rx) = trpl::channel();

            let val = String::from("hi");
            tx.send(val).unwrap();

            let received = rx.recv().await.unwrap();
            println!("Got: {received}");
        });
    }

    if false {
        println!("\nPart 5\n");

        trpl::run(async {
            let (tx, mut rx) = trpl::channel();

            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }

            while let Some(value) = rx.recv().await {
                println!("Got: {value}");
            }
        });
    }

    if false {
        println!("\nPart 6\n");

        trpl::run(async {
            let (tx, mut rx) = trpl::channel();

            let tx_fut = async {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let rx_fut = async {
                while let Some(value) = rx.recv().await {
                    println!("Got: {value}");
                }
            };

            trpl::join(tx_fut, rx_fut).await;
        });
    }

    {
        println!("\nPart 7\n");

        trpl::run(async {
            let (tx, mut rx) = trpl::channel();

            let tx_fut = async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let rx_fut = async {
                while let Some(value) = rx.recv().await {
                    println!("Got: {value}");
                }
            };

            trpl::join(tx_fut, rx_fut).await;
        });
    }

    {
        println!("\nPart 8\n");

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

            trpl::join3(tx1_fut, tx_fut, rx_fut).await;
        });
    }
}
