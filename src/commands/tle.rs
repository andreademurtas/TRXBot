use crate::{Context, Error};
use chrono::prelude::*;
use satellite;
use staticmap::{
    tools::{Color, CircleBuilder},
    StaticMapBuilder,
};
use tokio::fs::{File, remove_file};
use poise::serenity_prelude as serenity;
use serenity::model::channel::AttachmentType;
use uuid::Uuid;

#[poise::command(track_edits, slash_command)]
pub async fn moonlighter(ctx: Context<'_>) -> Result<(), Error> {
    let tle1 = "1 25544U 98067A   23187.34555919  .00007611  00000-0  14335-3 0  9996";
    let tle2 = "2 25544  51.6398 233.5611 0000373  12.3897  91.4664 15.49560249404764";
    let mut satrec = satellite::io::twoline2satrec(tle1, tle2).unwrap();
    let now = Utc::now();
    let result = satellite::propogation::propogate_datetime(&mut satrec, now).unwrap();
    let gmst = satellite::propogation::gstime::gstime_datetime(now);
    let sat_pos = satellite::transforms::eci_to_geodedic(&result.position, gmst);
    let id = Uuid::new_v4();
    {
    let mut map = StaticMapBuilder::new()
        .width(300)
        .height(400)
        .padding((10, 0))
        .build()?;
    let red = Color::new(true, 255, 0, 0, 255);
    let circle = CircleBuilder::default()
        .lat_coordinate(sat_pos.latitude * satellite::constants::RAD_TO_DEG)
        .lon_coordinate(sat_pos.longitude * satellite::constants::RAD_TO_DEG)
        .radius(5.0)
        .color(red)
        .build()?;
    map.add_tool(circle);
    map.save_png(format!("{}.png", id))?;
    }
    let file = File::open(format!("{}.png", id)).await?;
    let attachement = AttachmentType::File {
        filename: "map.png".into(),
        file: &file
    }; 
    ctx.send(|c| {
        c.content(format!("Moonlighter is currently at lat: `{}`, lon: `{}` and alt: `{}`",
                          sat_pos.latitude * satellite::constants::RAD_TO_DEG,
                          sat_pos.longitude * satellite::constants::RAD_TO_DEG,
                          sat_pos.height * satellite::constants::KM_TO_MI
        ))
        .attachment(attachement)
    }).await?;
    remove_file(format!("{}.png", id)).await?;
    Ok(())
}
