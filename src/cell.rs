use ggez::{GameResult, Context};
use ggez::graphics::Image;

use center;
use globals::PI;
use hex::*;


#[derive(Debug)]
pub struct Assets {
    branch_images: Vec<Image>,
    leaf_images: Vec<Image>,
    flower_images: Vec<Image>,
    berry_images: Vec<Image>,
    nut_images: Vec<Image>,
    beehive_images: Vec<Image>,
    birdnest_images: Vec<Image>,
    squirrel_images: Vec<Image>,
    moss_images: Vec<Image>,
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(Assets {
        branch_images: vec!(
            Image::new(ctx, "/twig.png")?,
            Image::new(ctx, "/small branch.png")?,
            Image::new(ctx, "/big branch.png")?,
            Image::new(ctx, "/trunk.png")?,
        ),
        leaf_images: vec!(
            Image::new(ctx, "/leaves.png")?,
        ),
        flower_images: vec!(
            Image::new(ctx, "/flowers.png")?,
            //Image::new(ctx, "/flower1.png")?,
            //Image::new(ctx, "/flower2.png")?,
            //Image::new(ctx, "/flower3.png")?,
        ),
        berry_images: vec!(
            Image::new(ctx, "/berry bunch.png")?, // TODO: this is a flower, not a berry!
        ),
        nut_images: vec!(
            Image::new(ctx, "/nut bunch.png")?,
        ),
        beehive_images: vec!(
            Image::new(ctx, "/beehive.png")?,
        ),
        birdnest_images: vec!(
            Image::new(ctx, "/nest.png")?,
        ),
        squirrel_images: vec!(
            Image::new(ctx, "/squirrel.png")?,
        ),
        moss_images: vec!(
            Image::new(ctx, "/moss.png")?,
        ),
    })
}

impl Assets {
    fn gift_images(&self, gift: Gift) -> &Vec<Image> {
        match gift {
            Gift::Leaves   => &self.leaf_images,
            Gift::Flowers  => &self.flower_images,
            Gift::Berries  => &self.berry_images,
            Gift::Nuts     => &self.nut_images,
            Gift::Beehive  => &self.beehive_images,
            Gift::Birdnest => &self.birdnest_images,
            Gift::Squirrel => &self.squirrel_images,
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
    Beehive,
    Birdnest,
    Squirrel,
}

#[derive(Clone, Copy, Debug)]
pub struct GiftCell {
	pub gift: Option<Gift>,
    image_variant: usize,
    pub parent: BranchPoint,
}

#[derive(Clone, Copy, Debug)]
pub struct BranchCell {
	pub branch_strain_current: usize,
    pub branch_upgrade: usize,
    pub parent: Option<GiftPoint>,
}

impl Gift {
    pub fn singular(self) -> &'static str {
        match self {
            Gift::Leaves   => "leaf",
            Gift::Flowers  => "flower",
            Gift::Berries  => "berry",
            Gift::Nuts     => "nut",
            Gift::Beehive  => "beehive",
            Gift::Birdnest => "bird nest",
            Gift::Squirrel => "squirrel",
        }
    }

    pub fn plural(self) -> &'static str {
        match self {
            Gift::Leaves   => "leaves",
            Gift::Flowers  => "flowers",
            Gift::Berries  => "berries",
            Gift::Nuts     => "nuts",
            Gift::Beehive  => "beehives",
            Gift::Birdnest => "bird nests",
            Gift::Squirrel => "squirrels",
        }
    }
}

impl BranchCell {
    pub fn new(parent: Option<GiftPoint>) -> BranchCell {
        BranchCell {
            branch_strain_current: 0,
            branch_upgrade: 0,
            parent,
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

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, branch_point: BranchPoint) -> GameResult<()> {
        let image = &assets.branch_images[self.branch_upgrade];
        let angle = match branch_point.orientation() {
            Orientation::Vert     => 0.0,
            Orientation::Diag     => 60.0 * PI / 180.0,
            Orientation::AntiDiag => 120.0 * PI / 180.0,
        };
        center::draw_centered_image(ctx, image, branch_point.to_point(), angle)
    }
}

impl GiftCell {
    pub fn new(parent: BranchPoint) -> GiftCell {
        GiftCell {
            gift: None,
            image_variant: 0,
            parent,
        }
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, gift_point: GiftPoint) -> GameResult<()> {
        if let Some(gift) = self.gift {
            let image = &assets.gift_images(gift)[self.image_variant];
            center::draw_centered_image(ctx, image, gift_point.to_point(), 0.0)?;
        }

        Ok(())
    }
}
