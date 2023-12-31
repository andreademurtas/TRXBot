use crate::{Context, Error};
use factordb::Number;

#[poise::command(track_edits, slash_command)]
pub async fn factordb(
    ctx: Context<'_>,
    #[description = "Number to factor"] number: String,
) -> Result<(), Error> {
    let factors = Number::get_blocking(&number).unwrap();
    ctx.send(|c| {
        c.content(format!(
            "Factors of {}: {:?}",
            number,
            factors.factor_list()
        ))
    })
    .await?;
    Ok(())
}
