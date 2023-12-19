use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use std::io;
use std::io::Write;
// use ureq::Error;

#[derive(Debug, Deserialize)]
struct PostList {
    data: Childrens,
}
#[derive(Debug, Deserialize)]

struct Childrens {
    children: Vec<Child>,
}
#[derive(Debug, Deserialize)]

struct Child {
    data: Post,
}
#[derive(Debug, Deserialize)]

struct Post {
    title: String,
    permalink: String,
    created: f64,
}

fn get_post_time(created:f64){

    let dt: DateTime<Utc> = DateTime::from_timestamp(created as i64, 0).unwrap(); //2023-MM-DD T HH:MM:SS
    
        let time_of_posting = Utc::now() - dt;

        if time_of_posting.num_weeks() < 4 && time_of_posting.num_weeks() > 0 {
            if time_of_posting.num_weeks() == 1 {
                println!("Posted 1 week ago");
            } else {
                println!("Posted {:?} weeks ago", time_of_posting.num_weeks());
            }
        } else if time_of_posting.num_days() < 7 && time_of_posting.num_days() > 0 {
            if time_of_posting.num_days() == 1 {
                println!("Posted 1 day ago");
            } else {
                println!("Posted {:?} days ago.", time_of_posting.num_days());
            }
        } else if time_of_posting.num_hours() < 24 && time_of_posting.num_hours() > 0 {
            if time_of_posting.num_hours() == 1 {
                println!("Posted 1 hour ago");
            } else {
                println!("Posted {:?} hours ago", time_of_posting.num_hours());
            }
        } else if time_of_posting.num_minutes() < 60 && time_of_posting.num_minutes() > 0 {
            if time_of_posting.num_minutes() == 1 {
                println!("Posted 1 minute ago");
            } else {
                println!("Posted {:?} minutes ago", time_of_posting.num_minutes());
            }
        } else if time_of_posting.num_seconds() < 60 {
            println!("Posted just now");
        }



}


fn main() -> Result<(), ureq::Error> {
    println!("Redditor\n");

    print!("Enter the name of a subreddit:");
    std::io::stdout().flush().unwrap();

    let mut subreddit_name = String::new();
    io::stdin()
        .read_line(&mut subreddit_name)
        .expect("Error at reading the subreddit name");
    subreddit_name.pop();

    print!("Enter the desired sorting order:");
    std::io::stdout().flush().unwrap();

    let mut ordering = String::new();
    io::stdin()
        .read_line(&mut ordering)
        .expect("Error at reading the sorting order");
    ordering.pop();
    let mut sorting_order = String::from("/");
    sorting_order = sorting_order + &ordering + ".json";

    let domain: String = String::from("https://www.reddit.com/r/");
    let url = domain + &subreddit_name + &sorting_order;

    let body: String = ureq::get(&url).call()?.into_string()?;
    let postlist: PostList = serde_json::from_str(&body).expect("Error at deserializing");

    for post in postlist.data.children {
        
        println!("\nTitle : {:?} ", post.data.title);
        let domain = String::from("https://www.reddit.com");

        get_post_time(post.data.created);

        let url_to_post = domain + &post.data.permalink;
        println!("Link to post : {:?} ", url_to_post);
    }

    Ok(())
}