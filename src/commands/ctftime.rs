use crate::{Context, Error};
use chrono::Datelike;

#[poise::command(track_edits, slash_command, subcommands("trx"))]
pub async fn ctftime(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(track_edits, slash_command)]
pub async fn trx(ctx: Context<'_>) -> Result<(), Error> {
    let resp = reqwest::get("https://ctftime.org/api/v1/teams/46516/")
        .await?
        .text()
        .await?;
    let json: serde_json::Value = serde_json::from_str(&resp)?;
    let year = chrono::Utc::now().year();
    let rating_place = &json["rating"][&year.to_string().as_str()]["rating_place"];
    let organizer_points = &json["rating"][&year.to_string().as_str()]["organizer_points"];
    let rating_points = &json["rating"][&year.to_string().as_str()]["rating_points"];
    let country_place = &json["rating"][&year.to_string().as_str()]["country_place"];
    let message = format!(
        "How are we doing this year?\n```Rating place: {}\nOrganizer points: {}\nRating points: {}\nCountry place: {}```",
        rating_place,
        organizer_points,
        rating_points,
        country_place
    );
    ctx.send(|c| c.content(message).reply(true)).await?;
    Ok(())
}
