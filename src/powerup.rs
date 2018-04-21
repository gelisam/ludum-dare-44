use ggez::{GameResult, Context};
use ggez::graphics::{Font, Text};

use hex::HexPoint;
use text::draw_centered_text;


#[derive(Debug)]
pub struct Assets {
    bomb:         Text,
    kick:         Text,
    zip:          Text,
    spike_shield: Text,
}

pub fn load_assets(ctx: &mut Context, font: &Font) -> GameResult<Assets> {
    Ok(
        Assets {
            bomb:         Text::new(ctx, "Bomb", &font)?,
            kick:         Text::new(ctx, "Kick", &font)?,
            zip:          Text::new(ctx, "Zip", &font)?,
            spike_shield: Text::new(ctx, "SpikeShield", &font)?,
        }
    )
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
enum Powerup {
    Bomb,
    Kick,
    Zip,
    SpikeShield,
}

#[allow(dead_code)]
impl Powerup {
    pub fn draw(self, ctx: &mut Context, assets: &Assets, dest: HexPoint) -> GameResult<()> {
        let text: &Text = match self {
            Powerup::Bomb        => &assets.bomb,
            Powerup::Kick        => &assets.kick,
            Powerup::Zip         => &assets.zip,
            Powerup::SpikeShield => &assets.spike_shield,
        };
        draw_centered_text(ctx, text, dest.to_point(), 0.0)
    }

    pub fn first() -> Powerup {
        Powerup::Bomb
    }

    pub fn next(self) -> Option<Powerup> {
        match self {
            Powerup::Bomb        => Some(Powerup::Kick),
            Powerup::Kick        => Some(Powerup::Zip),
            Powerup::Zip         => Some(Powerup::SpikeShield),
            Powerup::SpikeShield => None,
        }
    }
}
