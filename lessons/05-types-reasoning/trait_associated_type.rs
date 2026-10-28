#![allow(dead_code)]

use std::fmt::Display;

trait Summary<'a> {
    type Summarizer: Display;

    fn summarize(&'a self) -> Self::Summarizer;
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

impl<'a> Summary<'a> for NewsArticle {
    type Summarizer = NewsArticleSummarizer<'a>;
    fn summarize(&'a self) -> Self::Summarizer {
        NewsArticleSummarizer(self)
    }
}

struct Tweet {
    username: String,
    content: String,
}

struct TweetSummarizer<'a>(&'a Tweet);

impl Display for TweetSummarizer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tweet = self.0;
        write!(f, "{}: {}", tweet.username, tweet.content)
    }
}

impl<'a> Summary<'a> for Tweet {
    type Summarizer = TweetSummarizer<'a>;
    fn summarize(&'a self) -> Self::Summarizer {
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
