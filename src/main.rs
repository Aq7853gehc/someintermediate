// fn main() {
//  panic!("Crash and terminate the porgram")
// }

// fn main() {
//     let v: Vec<i32>= vec![1,2,4];
//     v[88];
//     println!("{}",v[99]);
// }

// use std::fs::File;

// fn main() {
//     let opne_file = File::open("hello.txt");
//     let _open_file_resutl = match opne_file {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}", error),
//     };
// }


use app::NewsArticle;
use app::Summary;
use app::Tweet;



fn main() {
    let artical = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    let tweet:Tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());
    println!("{}", artical.summarize());
    
}
