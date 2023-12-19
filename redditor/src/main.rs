use chrono::{DateTime, Utc};
use serde_derive::Deserialize;

use std::thread::sleep;
use std::time::Duration;

use std::io;
use std::io::Write;

use std::collections::HashSet;

// use ureq::Error;

#[derive(Debug, Deserialize)]
struct PostList {
    data: Children,
}
#[derive(Debug, Deserialize)]

struct Children {
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
    id: String,
}

use clap::Parser;

#[derive(Parser)]
#[command(version, about = "args parsing example")]
struct Args {
    /// Subbreddit name
    #[arg(long)]
    subreddit: String,

    /// Sort option (hot,new,top)
    #[arg(long, default_value = "hot")]
    sort: String,
}

fn main() -> Result<(), ureq::Error> {
    println!("\n=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-\n");
    println!("=-=-=-=-=-=-=  Redditor  =-=-=-=-=-=-=-=\n");

    let mut buffer = String::new();
    print!("No. of seconds for refresh rate:");
    std::io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Error while reading from stdin");
    let seconds: u64 = buffer.trim().parse().expect("Not an u64 value");

    buffer = String::new();
    print!("Number of refreshes:");
    std::io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error while reading from stdin");
    let refreshes: u32 = buffer.trim().parse().expect("Not an u32 value");

    let args = Args::parse();
    let subreddit_name = args.subreddit;
    let ordering = args.sort;

    let domain: String = String::from("https://www.reddit.com/r/");

    let mut sorting_order = String::from("/");
    sorting_order = sorting_order + &ordering + ".json";

    let url = domain + &subreddit_name + &sorting_order;

    let body: String = ureq::get(&url).call()?.into_string()?;
    let postlist: PostList = serde_json::from_str(&body).expect("Error at deserializing");

    println!(
        "Showing posts for subbredit:{:?} sorting order:{:?};",
        subreddit_name, ordering
    );

    let mut printed_posts_ids = HashSet::new();

    for post in postlist.data.children {
        println!("\nTitle : {:?} ", post.data.title);
        let domain = String::from("https://www.reddit.com");

        let posting_data: DateTime<Utc> =
            DateTime::from_timestamp(post.data.created as i64, 0).unwrap(); //2023-MM-DD T HH:MM:SS
        let fmt_posting_data = posting_data.format("%d.%m.%Y at %R").to_string();
        println!("Creation date:{:?}", fmt_posting_data);

        let url_to_post = domain + &post.data.permalink;
        println!("Link to post : {:?} ", url_to_post);

        printed_posts_ids.insert(post.data.id.clone());
    }

    let mut count = 1;
    println!("\nPrinting new posts:\n");
    loop {
        let body: String = ureq::get(&url)
            .call()
            .expect("Error fetching data")
            .into_string()
            .expect("Error converting to string");
        let postlist: PostList = serde_json::from_str(&body).expect("Error at deserializing");

        let mut new_post = 0;
        for post in postlist.data.children {
            if !printed_posts_ids.contains(&post.data.id) {
                println!("\n=-=-=-=-=-=-=-=New post!=-=-=-=-=-=-=-=-=");
                println!("\nTitle : {:?} ", post.data.title);

                let domain = String::from("https://www.reddit.com");

                let posting_data: DateTime<Utc> =
                    DateTime::from_timestamp(post.data.created as i64, 0).unwrap();
                let fmt_posting_data = posting_data.format("%d.%m.%Y at %R").to_string();
                println!("Creation date:{:?}", fmt_posting_data);

                let url_to_post = domain + &post.data.permalink;
                println!("Link to post : {:?} \n", url_to_post);

                printed_posts_ids.insert(post.data.id.clone());
                new_post = 1;
            }
        }

        sleep(Duration::from_secs(seconds));

        if new_post == 0 {
            println!(
                "No new posts have been found. Next refresh in {} seconds. ({}/{})",
                seconds, count, refreshes
            );
            count += 1;
        }
        if count == refreshes + 1 {
            break;
        }
    }

    Ok(())
}
