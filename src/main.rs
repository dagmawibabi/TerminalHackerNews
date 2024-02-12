#[allow(unused_imports)]
use std::{io::{stdin, stdout, Write}, time::Duration};
use crossterm::terminal::ClearType;
use reqwest;
use serde_json::from_str;
use serde::Deserialize;
use chrono::Local;

#[allow(unused_imports)]
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, },
    ExecutableCommand,
    event,
    terminal::Clear,
    cursor,
};

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct EachStory {
    by: String,
    descendants: u64,
    id: u64,
    kids: Vec<u64>,
    score: u64,
    time: u64,
    title: String,
    r#type: String,
    url: String,
}

#[allow(unused_must_use)]
fn terminal_output(current_story: EachStory) -> std::io::Result<()> {

    let duration: Duration = Duration::from_millis(current_story.time);
    let timestamp = Local::now() - duration; 
    timestamp.format("%Y-%m-%d").to_string();

    execute!(
        stdout(),
        
        Print("\n"),

        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("\tArticle: "),

        SetForegroundColor(Color::Yellow),
        SetBackgroundColor(Color::Black),
        Print(current_story.title),

        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("\n\t     By: "),

        SetForegroundColor(Color::Green),
        SetBackgroundColor(Color::Black),
        Print(current_story.by),
        
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("\n\t   Time: "),

        SetForegroundColor(Color::Magenta),
        SetBackgroundColor(Color::Black),
        Print(timestamp),

        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print("\n\t    URL: "),

        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Black),
        Print(current_story.url),

        Print("\n"),

        ResetColor
    );

    Ok(())
}

fn clear_screen() -> std::io::Result<()> {
    // Clear the entire screen and move the cursor to the top left corner
    execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    // Alternatively, clear only the current line:
    // execute!(stdout(), Clear(ClearType::CurrentLine))?;

    Ok(())
}

#[tokio::main]
async fn main() -> core::result::Result<(), reqwest::Error> {

    // Get Top Stories
    println!("Getting top news...");
    let top_stories_id: String = reqwest::get("https://hacker-news.firebaseio.com/v0/newstories.json?print=pretty")
    .await?
    .text()
    .await?;

    // JSON Top Stories
    let top_stories_json: Vec<u64> = from_str(&top_stories_id).unwrap();

    // Query User
    // println!("How many news would you like to see? (5/10/15...)");
    // stdout().flush().unwrap();
    // let mut input = String::new();
    // stdin().read_line(&mut input).unwrap();
    // let news_limit = input.trim().parse().unwrap();
    let news_limit = 10;

    // Loop Top Stories
    println!("Fetching each news...");

    let _clear_terminal = clear_screen();

    #[allow(unused_must_use)]
    let mut top_stories_vec: Vec<EachStory> = vec![];

    let mut index = 0;
    while top_stories_vec.len() < news_limit {
        // Get each news
        let current_story_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json?print=pretty", top_stories_json[index]);
        let current_story = reqwest::get(current_story_url)
        .await?
        .text()
        .await?;
        // Display Each News
        let current_story_obj: Result<EachStory, serde_json::Error> = from_str(&current_story);
        match current_story_obj {
            Ok(current_story_obj) => {
                let copied_story = current_story_obj.clone();
                top_stories_vec.push(current_story_obj);
                let _ = terminal_output(copied_story);
            }
            Err(_error) => {}
        }
        index += 1;
    }

    Ok(())
}
