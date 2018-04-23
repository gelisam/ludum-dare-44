use ggez::{GameResult, Context};
use ggez::graphics::{Font, Point2, Text, Vector2};

use globals::*;
use hex::{DirectionIndex, HexPoint};
use text::*;


#[derive(Debug)]
pub struct Assets {
    lap1_a: Text,
    lap1_b: Text,
    lap2:   Text,
    lap3:   Text,
    win_a:  Text,
    win_b:  Text,
    lose_a: Text,
    lose_b: Text,
}

pub fn load_assets(ctx: &mut Context, font: &Font) -> GameResult<Assets> {
    Ok(
        Assets {
            lap1_a: Text::new(ctx, "First to complete", &font)?,
            lap1_b: Text::new(ctx, "3 laps wins!", &font)?,
            lap2:   Text::new(ctx, "Second lap!", &font)?,
            lap3:   Text::new(ctx, "Final lap!", &font)?,
            win_a:  Text::new(ctx, "You win!", &font)?,
            win_b:  Text::new(ctx, "Press ESC to quit.", &font)?,
            lose_a: Text::new(ctx, "You lose :(", &font)?,
            lose_b: Text::new(ctx, "Press R to try again.", &font)?,
        }
    )
}

pub fn draw_lap_message(ctx: &mut Context, assets: &Assets, lap: Lap) -> GameResult<()> {
    let center = Point2::new(
        WINDOW_WIDTH as f32 / 2.0,
        WINDOW_HEIGHT as f32 / 2.0 - 13.0,
    );
    match lap {
        0 => {
            draw_centered_text(ctx, &assets.lap1_a, center, 0.0)?;
            draw_centered_text(ctx, &assets.lap1_b, center + Vector2::new(0.0, 20.0), 0.0)?;
            Ok(())
        },
        1 => draw_centered_text(ctx, &assets.lap2, center, 0.0),
        2 => draw_centered_text(ctx, &assets.lap3, center, 0.0),
        _ => {
            draw_centered_text(ctx, &assets.win_a, center - Vector2::new(0.0, 20.0), 0.0)?;
            draw_centered_text(ctx, &assets.win_b, center, 0.0)?;
            Ok(())
        },
    }
}

pub fn draw_loss_message(ctx: &mut Context, assets: &Assets) -> GameResult<()> {
    let center = Point2::new(
        WINDOW_WIDTH as f32 / 2.0,
        WINDOW_HEIGHT as f32 / 2.0 - 13.0,
    );
    draw_centered_text(ctx, &assets.lose_a, center - Vector2::new(0.0, 20.0), 0.0)?;
    draw_centered_text(ctx, &assets.lose_b, center, 0.0)?;
    Ok(())
}


// 0 is from right up to but not including the first checkpoint,
// 1 is from the first checkpoint up to but not including the second, etc.
// always in the range 0..6
pub type Section = i32;

pub fn point_to_section(hex_point: HexPoint) -> Section {
    if hex_point.q == 0 {
        if hex_point.r < 0 {
            2
        } else {
            5
        }
    } else if hex_point.q > 0 {
        if hex_point.r > 0 {
            5
        } else if -hex_point.r < hex_point.q {
            0
        } else {
            1
        }
    } else {
        if hex_point.r < 0 {
            2
        } else if -hex_point.q > hex_point.r {
            3
        } else {
            4
        }
    }
}

pub fn at_checkpoint(hex_point: HexPoint) -> bool {
    hex_point.q == 0 || hex_point.r == 0 || hex_point.q == -hex_point.r
}

pub fn forward(hex_point: HexPoint) -> DirectionIndex {
    let section = point_to_section(hex_point);
    (section + 2) % 6
}

#[allow(dead_code)]
pub fn backward(hex_point: HexPoint) -> DirectionIndex {
    let mut section = point_to_section(hex_point);
    if at_checkpoint(hex_point) {
        section = (section + 5) % 6;
    }
    (section + 5) % 6
}


// same as Section, but increases beyond 5 when completing laps.
pub type Checkpoint = i32;

pub fn checkpoint_to_section(checkpoint: Checkpoint) -> Section {
    if checkpoint >= 0 {
        checkpoint % 6
    } else {
        5 + (checkpoint + 1) % 6
    }
}

pub fn update_checkpoint(old_checkpoint: Checkpoint, new_hex_point: HexPoint) -> Checkpoint {
  let old_section = checkpoint_to_section(old_checkpoint);
  let new_section = point_to_section(new_hex_point);
  if new_section == (old_section + 1) % 6 {
      old_checkpoint + 1
  } else if new_section == (old_section + 2) % 6 {
      old_checkpoint + 2
  } else if new_section == (old_section + 5) % 6 {
      old_checkpoint - 1
  } else if new_section == (old_section + 4) % 6 {
      old_checkpoint - 2
  } else {
      old_checkpoint
  }
}


// number of _completed_ laps, so 0 during the first lap, 1 during the second, etc.
// negative if the racer drives the wrong way.
pub type Lap = i32;

#[allow(dead_code)]
pub fn checkpoint_to_lap(checkpoint: Checkpoint) -> Lap {
    if checkpoint >= 0 {
        checkpoint / 6
    } else {
        (checkpoint + 1) / 6 - 1
    }
}
