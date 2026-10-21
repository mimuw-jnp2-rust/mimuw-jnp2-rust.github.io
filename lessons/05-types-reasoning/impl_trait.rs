#![allow(dead_code)]

use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> impl Display;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}, by {} ({})",
            self.headline, self.author, self.location
        )?;
        f.write_str(&self.content)
    }
}

struct NewsArticleSummarizer<'a>(&'a NewsArticle);

impl Display for NewsArticleSummarizer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let article = self.0;
        write!(
            f,
            "{}, by {} ({})",
            article.headline, article.author, article.location
        )
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> impl Display {
        NewsArticleSummarizer(self)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> impl Display {
        struct TweetSummarizer<'a>(&'a Tweet);

        impl Display for TweetSummarizer<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let tweet = self.0;
                write!(f, "{}: {}", tweet.username, tweet.content)
            }
        }

        TweetSummarizer(self)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    println!("1 new tweet: {}", tweet.summarize());
}
