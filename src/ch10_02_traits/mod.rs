trait Summary {
    fn summarize(&self) -> String {
        String::from("Default text...")
    }
}

struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{} {}", self.username, self.content)
    }
}

fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// fn notify(item: &(impl Summary + Display)) {
// fn notify<T: Summary + Display>(item: &T) {

// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {

pub fn run() {
    let article = Article {
        title: String::from("Hello World!"),
        author: String::from("Alex Black"),
        content: String::from("Some content"),
    };

    let tweet = Tweet {
        username: String::from("wa7sa34cx"),
        content: String::from("Nice day for coding!"),
        reply: true,
        retweet: false,
    };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
}
