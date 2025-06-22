fn main() {
    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probable already know, people"),
            reply: false,
            retween: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());
    }

    {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.default_summarize());
    }

    {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize_with_author());
    }

    {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        };

        notify(&article);
    }

    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probable already know, people"),
            reply: false,
            retween: false,
        };

        notify_2(&tweet);
    }

    {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        };

        notify_3(&article);
    }

    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probable already know, people"),
            reply: false,
            retween: false,
        };

        notify_4(&tweet);
    }

    {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        };

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probable already know, people"),
            reply: false,
            retween: false,
        };

        notify_5(&tweet, &article);
    }

    {
        let tweet = returns_summarizable();
        println!("Returned tweet: {}", tweet.summarize());
    }

    {
        let pair = Pair::new(5, 10);
        pair.cmp_display();

        let pair = Pair::new((), ());
        // pair.cmp_display(); // This will not compile because `()` does not implement `Display` or
        // `PartialOrd`.
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait DefaultSummary {
    fn default_summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait SummaryWithAuthor {
    fn get_author(&self) -> String;

    fn summarize_with_author(&self) -> String {
        format!("(Read more from {}...)", self.get_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl DefaultSummary for NewsArticle {}

impl SummaryWithAuthor for NewsArticle {
    fn get_author(&self) -> String {
        self.author.clone()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retween: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl DefaultSummary for Tweet {}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_3(item: &(impl Summary + DefaultSummary)) {
    println!("Breaking news! {}", item.summarize());
    println!("Default summary: {}", item.default_summarize());
}

pub fn notify_4<T: Summary + DefaultSummary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Default summary: {}", item.default_summarize());
}

pub fn notify_5<T, U>(item1: &T, item2: &U)
where
    T: Summary + DefaultSummary,
    U: Summary + SummaryWithAuthor,
{
    println!("Breaking news! {}", item1.summarize());
    println!("Default summary: {}", item1.default_summarize());
    println!("Author summary: {}", item2.summarize());
    println!("Author summary: {}", item2.summarize_with_author());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probable already know, people"),
        reply: false,
        retween: false,
    }
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
