#![allow(dead_code)]

use std::fmt::Display;

trait Summary<'a, Summarizer: Display> {
    fn summarize(&'a self) -> Summarizer;
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

impl<'a> Summary<'a, NewsArticleSummarizer<'a>> for NewsArticle {
    fn summarize(&'a self) -> NewsArticleSummarizer<'a> {
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

impl<'a> Summary<'a, TweetSummarizer<'a>> for Tweet {
    fn summarize(&'a self) -> TweetSummarizer<'a> {
        TweetSummarizer(self)
    }
}

impl<'a> Summary<'a, NewsArticleSummarizer<'a>> for Tweet {
    fn summarize(&'a self) -> NewsArticleSummarizer<'a> {
        unimplemented!("This is only to make code type-check and compile.");
    }
}

fn main() {
    let empty_article = NewsArticle {
        headline: "".into(),
        location: String::new(),
        author: String::default(),
        content: Default::default(),
    };
    println!("1 new article: {}", empty_article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    // Compile error: `type annotations needed; multiple `impl`s satisfying `Tweet: Summary<'_, _>` found`
    // println!("1 new tweet: {}", tweet.summarize());
    println!(
        "1 new tweet: {}",
        <Tweet as Summary<'_, TweetSummarizer>>::summarize(&tweet)
    );
    println!(
        "1 new tweet: {}",
        <Tweet as Summary<'_, NewsArticleSummarizer>>::summarize(&tweet)
    );
}
