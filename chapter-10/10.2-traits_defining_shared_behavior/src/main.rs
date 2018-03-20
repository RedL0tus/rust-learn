// Example 1: Tweet & article
pub trait Summarizable { // Defining trait
    fn author_summary(&self) -> String;

    fn summary(&self) -> String { // Default implementation
        return format!("(Read more from {}...)", self.author_summary());
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle { // Implement a trait on a type
    fn summary(&self) -> String { // Overwriting a default implementation
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }

    fn author_summary(&self) -> String {
        return format!("{}", self.author);
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet { // Same as above
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}

fn notify<T: Summarizable>(item: T) { // Trait bounds
    println!("Breaking news! {}", item.summary());
}

// Example 2: Largest
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn main() {
    // Example 1
    let article = NewsArticle {
        headline: String::from("Today will have a rain of salted fish"),
        location: String::from("Cupertino, CA, USA"),
        author: String::from("Salted Fish"),
        content: String::from("Today we will have a rain of huge quantity of salty salted fish."),
    };

    let tweet = Tweet {
        username: String::from("SaltedFish"),
        content: String::from("Lorem ipsum dolor.... dolor set... sit.. amit..."),
        reply: false,
        retweet: false,
    };

    notify(article);
    notify(tweet);

    // Example 2
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
