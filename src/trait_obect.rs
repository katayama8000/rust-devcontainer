// https://zenn.dev/mosu/articles/87e4715c4bcbb8
trait Article {
    fn show(&self);
}

struct User<T: Article> {
    articles: Vec<T>,
}

impl<T: Article> User<T> {
    fn show_all(&self) {
        for article in self.articles.iter() {
            article.show();
        }
    }
}

struct TechArticle {
    title: String,
}

impl Article for TechArticle {
    fn show(&self) {
        println!("tech: {}", self.title);
    }
}

struct Tweet;
impl Article for Tweet {
    fn show(&self) {
        println!("Tweet");
    }
}

impl Tweet {
    fn tweet(&self) {
        println!("Tweet");
    }
}

struct Blog;
impl Article for Blog {
    fn show(&self) {
        println!("Blog");
    }
}

pub fn run() {
    println!(
        "{}{}{}trait_obect.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );

    let user: User<Tweet> = User {
        articles: vec![Tweet, Tweet],
    };
    user.show_all();
    user.articles[0].show();
    user.articles[0].tweet();
}
