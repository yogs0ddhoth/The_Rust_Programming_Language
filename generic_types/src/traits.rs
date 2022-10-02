pub mod aggregator;

use aggregator::{
    NewsArticle,
    Summary, // Trait must be brought into scope to be called
    // Types imported
    Tweet,
};

/* Traits are similar to interfaces from other languages */
pub fn impl_tweet() {
    let tweet = returns_summarizable();
    println!("1 new tweet: {}", tweet.summarize());
}
pub fn summarize_vec() {
    let vec = vec!["Be ", "seeing ", "you ", "Chuck."];
    println!("{}\n", vec.summarize());
}

pub fn impl_article() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("John"),
        content: String::from("Hiya Chuck, it's John."),
    };
    println!("News article available! {}", article.summarize());
}

// A trait implementation is a valid type return 
// Only one type may be returned, though
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("it's_john"),
        content: String::from("Hiya Chuck."),
        reply: false,
        retweet: false,
    }
}