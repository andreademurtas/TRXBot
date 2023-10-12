use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(track_edits, slash_command)]
pub async fn shellcode(ctx: Context<'_>, 
    syscall: String, #[description = "x86, arm etc"] arch: String, max_results: Option<u32>) -> Result<(), Error> {
    if max_results > Some(10) {
        ctx.say("Max results is 10").await?;
        return Ok(());
    }
    let mut msg = String::new();
    msg.push_str("http://shell-storm.org/api/?s=");
    msg.push_str(&syscall);
    msg.push_str("*");
    msg.push_str(&arch);
    let resp = reqwest::get(&msg).await?.text().await?;
    let lines = resp.lines();
    let mut results = Vec::new();
    for line in lines {
        let mut parts = line.split("::::");
        let author = parts.next().unwrap();
        let platform = parts.next().unwrap();
        let title = parts.next().unwrap();
        let id = parts.next().unwrap();
        let url = parts.next().unwrap();
        results.push((author, platform, title, id, url));
    }
    let mut msg = String::new();
    for (i, (author, platform, title, id, url)) in results.iter().enumerate() {
        msg.push_str(&format!("{}: {} - {} - {} - {} - {}\n", i + 1, author, platform, title, id, url));
    }
    ctx.send(|m| m.content(msg).reply(true)).await?;
    Ok(())
}
