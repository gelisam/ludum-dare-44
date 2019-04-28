use ggez::{GameResult, Context};
use ggez::graphics::Image;
use rand::Rng;

use center;
use hex::*;


#[derive(Debug)]
pub struct Assets {
    vert_branch_images: Vec<Vec<Image>>,
    diag_branch_images: Vec<Vec<Image>>,
    anti_diag_branch_images: Vec<Vec<Image>>,
    leaf_images: Vec<Image>,
    flower_images: Vec<Image>,
    berry_images: Vec<Image>,
    nut_images: Vec<Image>,
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(Assets {
        vert_branch_images: vec!(
            vec!(
                Image::new(ctx, "/twig vert 1.png")?,
                Image::new(ctx, "/twig vert 2.png")?,
                Image::new(ctx, "/twig vert 3.png")?,
            ),
            vec!(
                Image::new(ctx, "/branch vert 1.png")?,
                Image::new(ctx, "/branch vert 2.png")?,
                Image::new(ctx, "/branch vert 3.png")?,
            ),
            vec!(
                Image::new(ctx, "/trunk vert 1.png")?,
                Image::new(ctx, "/trunk vert 2.png")?,
            ),
        ),
        diag_branch_images: vec!(
            vec!(
                Image::new(ctx, "/twig dia 1.png")?,
                Image::new(ctx, "/twig dia 2.png")?,
                Image::new(ctx, "/twig dia 3.png")?,
            ),
            vec!(
                Image::new(ctx, "/branch dia 1.png")?,
                Image::new(ctx, "/branch dia 2.png")?,
            ),
            vec!(
                Image::new(ctx, "/trunk dia 1.png")?,
                Image::new(ctx, "/trunk dia 2.png")?,
            ),
        ),
        anti_diag_branch_images: vec!(
            vec!(
                Image::new(ctx, "/twig anti-dia 1.png")?,
                Image::new(ctx, "/twig anti-dia 2.png")?,
                Image::new(ctx, "/twig anti-dia 3.png")?,
            ),
            vec!(
                Image::new(ctx, "/branch anti-dia 1.png")?,
                Image::new(ctx, "/branch anti-dia 2.png")?,
            ),
            vec!(
                Image::new(ctx, "/trunk anti-dia 1.png")?,
                Image::new(ctx, "/trunk anti-dia 2.png")?,
            ),
        ),
        leaf_images: vec!(
            Image::new(ctx, "/leaves 1.png")?,
            Image::new(ctx, "/leaves 2.png")?,
        ),
        flower_images: vec!(
            Image::new(ctx, "/flower 1.png")?,
            Image::new(ctx, "/flower 2.png")?,
        ),
        berry_images: vec!(
            Image::new(ctx, "/flowers 3.png")?, // TODO: this is a flower, not a berry!
        ),
        nut_images: vec!(
            Image::new(ctx, "/beehive.png")?, // TODO: this is a beehive, not a nut!
        ),
    })
}

impl Assets {
    fn branch_images(&self, orientation: Orientation) -> &Vec<Vec<Image>> {
        match orientation {
            Orientation::Vert     => &self.vert_branch_images,
            Orientation::Diag     => &self.diag_branch_images,
            Orientation::AntiDiag => &self.anti_diag_branch_images,
        }
    }

    fn gift_images(&self, gift: Gift) -> &Vec<Image> {
        match gift {
            Gift::Leaves  => &self.leaf_images,
            Gift::Flowers => &self.flower_images,
            Gift::Berries => &self.berry_images,
            Gift::Nuts    => &self.nut_images,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IntOrInfinite {
    Infinite,
    Finite(u32)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Gift {
    Leaves,
    Flowers,
    Berries,
	Nuts,
}

#[derive(Debug)]
pub struct GiftCell {
	gift: Option<Gift>,
    image_variant: usize,
    parent: Option<BranchPoint>,
}

#[derive(Debug)]
pub struct BranchCell {
	pub branch_strain_current: usize,
    pub branch_upgrade: usize,
    image_variant: usize,
    pub parent: Option<GiftPoint>,
}

impl BranchCell {
    pub fn new() -> BranchCell {
        BranchCell {
            branch_strain_current: 0,
            branch_upgrade: 0,
            image_variant: 0,
            parent: None,
        }
    }

    fn branch_strain_maximum(&self) -> IntOrInfinite {
        match self.branch_upgrade {
            0 => IntOrInfinite::Finite(5),
            1 => IntOrInfinite::Finite(25),
            2 => IntOrInfinite::Finite(125),
            3 => IntOrInfinite::Infinite,
            _ => unreachable!(),
        }
    }

    // will crash if branch_upgrade is 3 or more
    pub fn upgrade(&mut self, assets: &Assets, rng: &mut rand::ThreadRng, branch_point: BranchPoint, branch_upgrade: usize) {
        self.branch_upgrade = branch_upgrade;
        let n = assets.branch_images(branch_point.orientation())[self.branch_upgrade].len();
        self.image_variant = rng.gen_range(0, n);
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, branch_point: BranchPoint) -> GameResult<()> {
        let image = &assets.branch_images(branch_point.orientation())[self.branch_upgrade][self.image_variant];
        center::draw_centered_image(ctx, image, branch_point.to_point(), 0.0)
    }
}

impl GiftCell {
    pub fn new() -> GiftCell {
        GiftCell {
            gift: Some(Gift::Flowers),
            image_variant: 0,
            parent: None,
        }
    }

    pub fn next(&mut self, assets: &Assets) {
        if let Some(gift) = self.gift {
            let n = assets.gift_images(gift).len();
            if self.image_variant+1 < n {
                self.image_variant += 1;
            } else {
                self.image_variant = 0;
                self.gift = Some(
                    match gift {
                        Gift::Leaves  => Gift::Flowers,
                        Gift::Flowers => Gift::Berries,
                        Gift::Berries => Gift::Nuts,
                        Gift::Nuts    => Gift::Leaves,
                    }
                );
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, gift_point: GiftPoint) -> GameResult<()> {
        if let Some(gift) = self.gift {
            let image = match gift {
                Gift::Leaves => {
                    &assets.leaf_images[self.image_variant]
                },
                Gift::Flowers => {
                    &assets.flower_images[self.image_variant]
                },
                Gift::Berries => {
                    &assets.berry_images[self.image_variant]
                },
                Gift::Nuts => {
                    &assets.nut_images[self.image_variant]
                },
            };
            center::draw_centered_image(ctx, image, gift_point.to_point(), 0.0)?;
        }

        Ok(())
    }
}
