use trpl::{Either, Html};

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

async fn page_url_and_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

fn main() {
    {
        let args: Vec<String> = std::env::args().collect();
        trpl::run(async {
            let url = &args[1];
            match page_title(url).await {
                Some(title) => println!("The title for {url} was {title}"),
                None => println!("{url} had no title"),
            };
        });
    }

    {
        let args: Vec<String> = std::env::args().collect();
        trpl::run(async {
            let title_fut_1 = page_url_and_title(&args[1]);
            let title_fut_2 = page_url_and_title(&args[2]);

            let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

            println!("{url} returned first");
            match maybe_title {
                Some(title) => println!("The title for {url} was {title}"),
                None => println!("{url} had no title"),
            };
        });
    }
}
