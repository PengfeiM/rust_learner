use traits::{NewsArticle, SocialPost, Summary};
// use aggregator::{SocialPost, Summary};

fn main() {
    println!("Hello, world!");
    println!("==========================================");

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    // default implementation
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.default_sum());
    println!("Try Override the default_sum: {}", post.default_sum());
}
