use chrono::{DateTime, FixedOffset, Utc};
use serde_derive::Deserialize;
use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

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
fn print_post(post: &Child) {
    println!("\nTitle : {:?} ", post.data.title);
    let domain = String::from("https://www.reddit.com");

    let epoch_post_timestamp: DateTime<Utc> =
        DateTime::from_timestamp(post.data.created as i64, 0).unwrap(); //2023-MM-DD T HH:MM:SS

    let timezone_offset = FixedOffset::east_opt(2 * 3600).unwrap();

    let dt = epoch_post_timestamp.with_timezone(&timezone_offset);
    let fmt_posting_data = dt.format("%d.%m.%Y at %R").to_string();

    println!("Creation date:{:?}", fmt_posting_data);

    let url_to_post = domain + &post.data.permalink;
    println!("Link to post : {:?} ", url_to_post);
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("\n=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-\n");
    println!("=-=-=-=-=-=-=  Redditor  =-=-=-=-=-=-=-=\n");

    let mut buffer = String::new();

    print!("Number of seconds for refresh rate:");
    std::io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Error while reading from stdin");
    let seconds: u64 = buffer.trim().parse()?;

    buffer = String::new();
    print!("Number of refreshes:");
    std::io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error while reading from stdin");
    let refreshes: u32 = buffer.trim().parse()?;

    let args = Args::parse();
    let subreddit_name = args.subreddit;
    let ordering = args.sort;

    let domain: String = String::from("https://www.reddit.com/r/");

    let mut sorting_order = String::from("/");
    sorting_order = sorting_order + &ordering + ".json";

    let url = domain + &subreddit_name + &sorting_order;

    let body: String = ureq::get(&url).call()?.into_string()?;
    let postlist: PostList = serde_json::from_str(&body)?;

    println!("\n=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-\n");
    println!(
        "Showing posts for subbredit:{:?} sorting order:{:?};",
        subreddit_name, ordering
    );

    let mut printed_posts_ids = HashSet::new();

    for post in postlist.data.children {
        print_post(&post);
        printed_posts_ids.insert(post.data.id.clone());
    }
    println!("\n=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-\n");

    let mut count = 1;
    println!("\nPrinting new posts:\n");
    loop {
        let body: String = ureq::get(&url).call()?.into_string()?;
        let postlist: PostList = serde_json::from_str(&body)?;

        let mut new_post = 0;
        for post in postlist.data.children {
            if !printed_posts_ids.contains(&post.data.id) {
                println!("\n=-=-=-=-=-=-=-=New post!=-=-=-=-=-=-=-=-=");
                print_post(&post);
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
