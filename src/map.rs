use ggez::{GameResult, Context};
use ggez::graphics::{Font, Text};

use globals::*;
use hex::HexPoint;
use text;


#[derive(Debug)]
pub struct Assets {
    bonus_box:       Text,
    car:             Text,
    checkpoint_line: Text,
    finish_line:     Text,
    obstacle:        Text,
    wall:            Text,
}

pub fn load_assets(ctx: &mut Context, font: &Font) -> GameResult<Assets> {
    Ok(
        Assets {
            bonus_box:       Text::new(ctx, "?", &font)?,
            car:             Text::new(ctx, "V", &font)?,
            checkpoint_line: Text::new(ctx, ".", &font)?,
            finish_line:     Text::new(ctx, ":", &font)?,
            obstacle:        Text::new(ctx, "@", &font)?,
            wall:            Text::new(ctx, "#", &font)?,
        }
    )
}


#[derive(Clone, Copy, Debug)]
pub enum CellContents {
    BonusBox,
    Car,
    CheckpointLine,
    FinishLine,
    Obstacle,
    Wall,
}

impl CellContents {
    pub fn draw(self, ctx: &mut Context, assets: &Assets, dest: HexPoint) -> GameResult<()> {
        let text: &Text = match self {
            CellContents::BonusBox       => &assets.bonus_box,
            CellContents::Car            => &assets.car,
            CellContents::CheckpointLine => &assets.checkpoint_line,
            CellContents::FinishLine     => &assets.finish_line,
            CellContents::Obstacle       => &assets.obstacle,
            CellContents::Wall           => &assets.wall,
        };
        let rotation: f32 = match self {
            CellContents::Car => PI, // I want the pointy bit of the "V" to point upwards
            _   => 0.0,
        };
        text::draw_centered_text(ctx, text, dest.to_point(), rotation)
    }
}
