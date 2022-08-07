use crate::{AppError, Context};
use rand::Rng;

///rolling dices is FUN
#[poise::command(slash_command)]
pub async fn dice(
    ctx: Context<'_>,
    #[description = "number of dice"]
    #[min = 1_u8]
    number_of_dice: u8,
    #[description = "number of side"]
    #[min = 1_u8]
    number_of_side: u8,
) -> Result<(), AppError> {
    let mut res = vec![];
    for _ in 0..number_of_dice {
        let mut rng = rand::thread_rng();
        res.push(rng.gen_range(1..=number_of_side));
    }
    let res = res
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    ctx.say(res).await?;

    Ok(())
}
