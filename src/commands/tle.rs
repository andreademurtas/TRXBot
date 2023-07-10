use crate::{Context, Error};
use chrono::prelude::*;
use satellite;

#[poise::command(track_edits, slash_command)]
pub async fn moonlighter(ctx: Context<'_>) -> Result<(), Error> {
    let tle1 = "1 25544U 98067A   23187.34555919  .00007611  00000-0  14335-3 0  9996";
    let tle2 = "2 25544  51.6398 233.5611 0000373  12.3897  91.4664 15.49560249404764";
    let mut satrec = satellite::io::twoline2satrec(tle1, tle2).unwrap();
    let now = Utc::now();
    let result = satellite::propogation::propogate_datetime(&mut satrec, now).unwrap();
    let gmst = satellite::propogation::gstime::gstime_datetime(now);
    let sat_pos = satellite::transforms::eci_to_geodedic(&result.position, gmst);
    ctx.send(|c| {
        c.content(format!("Moonlighter is currently at lat: {}, lon: {} and alt: {}",
                          sat_pos.latitude * satellite::constants::RAD_TO_DEG,
                          sat_pos.longitude * satellite::constants::RAD_TO_DEG,
                          sat_pos.height * satellite::constants::KM_TO_MI
        ))
    }).await?;
    Ok(())
}
