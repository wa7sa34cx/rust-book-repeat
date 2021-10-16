pub trait Info {
    fn author(&self) -> String;

    fn summary(&self) -> String {
        format!("Hello, {}", self.author())
    }
}

pub struct Article {
    author: String,
    title: String,
    date: u64,
}

impl Info for Article {
    fn author(&self) -> String {
        self.author.to_owned()
    }
}

fn print_summary<I>(article: &I)
where
    I: Info,
{
    println!("{}", article.summary());
}

fn main() {
    let article = Article {
        author: "Alex".to_string(),
        title: "Cold cold winter".to_string(),
        date: 42,
    };

    print_summary(&article);

    println!("{} - {}", article.title, article.date);
}
