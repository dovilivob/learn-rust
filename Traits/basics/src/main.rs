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

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// a trait is like an interface in other languages
pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;
    // use default implementation
    fn summarize_default(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// impl Trait syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize_default());
}

// Trait Bound Syntax
pub fn notify_1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize_default());
}

// in this case, item1 and item2 can be different types
pub fn notify_2(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item1.summarize_default());
    println!("Breaking news! {}", item2.summarize_default());
}

// Trait Bound Syntax with where clause, which can be more readable, and set item1 and item2 to the same type
pub fn notify_3<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize_default());
    println!("Breaking news! {}", item2.summarize_default());
}

// if we want to impl multiple traits, we can use + syntax,
// like &(impl Summary + Display) or <T: Summary + Display>

// where clause example:
pub trait Display {}
pub trait Debug {}

fn _some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    // function body would be here
    1
}

// same as above, but with where clause
fn _some_function_1<T, U>(_t: &T, _u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // function body would be here
    1
}

// return traits
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let news_article_1 = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    let tweet_1 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let tweet_2 = Tweet {
        username: String::from("2 horse_ebooks"),
        content: String::from("2 of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet_1.summarize());
    println!("New article available! {}", news_article_1.summarize());

    println!("1 new tweet: {}", tweet_1.summarize_default());
    println!(
        "New article available! {}",
        news_article_1.summarize_default()
    );

    notify(&tweet_1);
    notify_1(&tweet_1);
    notify_2(&tweet_1, &news_article_1);
    notify_3(&tweet_1, &tweet_2);

    // calling return_summarizable()
    println!("1 new tweet: {}", _returns_summarizable().summarize());
}
