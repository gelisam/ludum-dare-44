extern crate core;
extern crate ggez;
extern crate rand;
extern crate counter;

use core::time::Duration;
use ggez::{GameResult, Context};
use ggez::audio;
use ggez::event::*;
use ggez::graphics::*;
use ggez::timer;
use rand::seq::SliceRandom;
use std::collections::HashMap;

mod bg;
mod cell;
mod center;
mod channel;
mod globals;
mod hex;
mod sidebar;
mod text;
mod vector;
mod life;

use globals::*;
use life::Stats;

#[derive(Debug)]
struct Assets {
    bg: bg::Assets,
    cell: cell::Assets,
    dot: Mesh,
    font: Font,
    hex: hex::Assets,
    branch_place_sound: audio::Source,
    branch_upgrade_sound: audio::Source,
    branch_break_sounds: Vec<audio::Source>,
    gift_release_sound: audio::Source,
    moss: Image,
}

type CellCheckFn = fn( &HashMap<hex::BranchPoint, cell::BranchCell>, &Stats,) -> bool;

//#[derive(Debug)]
struct Achievement {
    pub achieved: bool,
    pub message: &'static str,
    //pub text: Text,
    pub functor: CellCheckFn,
}

fn any_branches( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    (stats.branch_lv1_count>0) | (stats.branch_lv2_count>0)
}

fn fewer_branches( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    (stats.branch_lv1_count+stats.branch_lv2_count) < stats.branches_max
}

fn any_branch_length3( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    stats.branch_length3_count > 0
}

fn any_branch_length5( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    stats.branch_length5_count > 0
}

fn two_leaves( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    stats.leaf_count >= 2
}

fn no_foliage( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    (stats.leaf_count == 0) & (stats.flower_count == 0)
}

fn any_foliage( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    (stats.leaf_count > 0) | (stats.flower_count>0)
}

fn any_flowers( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    stats.flower_count>0
}

fn any_beehives( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    stats.beehive_count>0
}

fn any_branch_lv2( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    stats.branch_lv2_count>0
}

fn any_berries( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    stats.berry_count>0
}

fn any_nuts( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    stats.nut_count>0
}

fn any_birds( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    stats.birdnest_count>0
}

fn any_bounty_lv3( _branches: &HashMap<hex::BranchPoint, cell::BranchCell>, stats: &Stats,) -> bool
{
    stats.bounty_max>=3
}


struct Alert {
    pub message: &'static str,
    pub until_time: Duration,
}

pub enum AlertMessage {
    NotEnoughBounty,
    BranchesTooStrained,
}


impl Assets {
    fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
        let font = Font::default_font()?;

        Ok(Assets {
            bg: bg::load_assets(ctx)?,
            cell: cell::load_assets(ctx)?,
            dot: Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 10.0, 3.0)?,
            font,
            hex: hex::load_assets(ctx)?,
            branch_place_sound: audio::Source::new(ctx, "/branch_place.ogg")?,
            branch_upgrade_sound: audio::Source::new(ctx, "/branch_upgrade.ogg")?,
            branch_break_sounds: vec!(
                audio::Source::new(ctx, "/branch_break.ogg")?,
                audio::Source::new(ctx, "/branch_break2.ogg")?,
                audio::Source::new(ctx, "/branch_break3.ogg")?,
                audio::Source::new(ctx, "/branch_break4.ogg")?,
            ),
            gift_release_sound: audio::Source::new(ctx, "/branch_item_remove.ogg")?,
            moss: Image::new(ctx, "/moss.png")?,
        })
    }
}

//#[derive(Debug)]
struct Globals {
    assets: Assets,
    achievements: Vec<Achievement>,
    alerts: Vec<Alert>,
    alert_current: Option<usize>,
    start_time: Duration,
    turn_time: Duration,
    turn_duration: Duration,
    guitar_channel: channel::Channel,
    clarinet_channel: channel::Channel,
    high_pithed_clarinet_channel: channel::Channel,
    dreamy_bells_channel: channel::Channel,
    bounty: sidebar::Sidebar,
    life: sidebar::Sidebar,
    bounty_amount: f32,
    life_amount: f32,
    hover: Option<hex::InBoundsPoint>,
    root_point: hex::BranchPoint,
    branches: HashMap<hex::BranchPoint, cell::BranchCell>,
    gifts: HashMap<hex::GiftPoint, cell::GiftCell>,
    stats: Stats,
    forbidden: HashMap<hex::GiftPoint, bool>,
    cost_multiplier: f32, // for debugging
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        let assets = Assets::load_assets(ctx)?;
        let bounty = sidebar::Sidebar::new(
            ctx,
            &assets.font,
            "Life", //"Bounty", // Design decision that Bounty should be called Life in UI
            Color::from_rgb(181, 208, 212),
            0.0
        )?;
        let life = sidebar::Sidebar::new(
            ctx,
            &assets.font,
            "Bounty", //"Life", // Design decision that Life should be called Bounty in UI
            Color::from_rgb(242, 240, 186),
            WINDOW_WIDTH as f32 - sidebar::SIDEBAR_WIDTH
        )?;

        let mut globals = Globals {
            assets,
            achievements: vec!(
                Achievement {
                    achieved: false,
                    message: "TIP: Click near the tree trunk to add a branch - click between two cells",
                    functor: any_branches,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Right-click a branch to prune - right-click between two cells",
                    functor: fewer_branches,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Try making a longer branch",
                    functor: any_branch_length3,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Leaves and Flowers grow on ends of branches - try getting two leaves",
                    functor: two_leaves,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Right-click leaves to replace with moss - try deleting all foliage",
                    functor: no_foliage,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Right-click moss to allow growth again - try deleting a moss",
                    functor: any_foliage,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Flowers reqiure two leaves nearby - they die if no leaves",
                    functor: any_flowers,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Click a branch to grow it thicker and allow a bigger tree",
                    functor: any_branch_lv2,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Try building a very long branch",
                    functor: any_branch_length5,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Beehives appear when two flowers are nearby - more Bounty than flowers",
                    functor: any_beehives,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Berries grow when a beehive and two leaves are nearby - More Bounty then Beehive",
                    functor: any_berries,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Nuts grow only on the ends of thick branches near flowers and leaves",
                    functor: any_nuts,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Birds appear when two berries are nearby - Large multiplier to Bounty",
                    functor: any_birds,
                },
                Achievement {
                    achieved: false,
                    message: "TIP: Leaves, flowers and other life build Bounty - try getting to Bounty 5",
                    functor: any_bounty_lv3,
                },
            ),
            alerts: vec!(
                Alert {
                    message: "NOTE: Not enough Life for this action - build Bounty for faster Life",
                    until_time: Duration::from_millis(0),
                },
                Alert {
                    message: "NOTE: The branch or supporting ones must be thicker - click to make thicker",
                    until_time: Duration::from_millis(0),
                },
            ),
            alert_current: None,
            start_time: get_current_time(ctx),
            turn_time: get_current_time(ctx),
            turn_duration: Duration::from_millis(2000),
            guitar_channel: channel::Channel::new(ctx, "/guitar.ogg")?,
            clarinet_channel: channel::Channel::new(ctx, "/clarinet.ogg")?,
            high_pithed_clarinet_channel: channel::Channel::new(ctx, "/high-pitched clarinet.ogg")?,
            dreamy_bells_channel: channel::Channel::new(ctx, "/dreamy-bells.ogg")?,
            bounty,
            life,
            bounty_amount: 0.0,
            life_amount: 0.0,
            hover: None,
            root_point: hex::BranchPoint::new(hex::HexPoint::new(0, 1)),
            branches: HashMap::with_capacity(100),
            gifts: HashMap::with_capacity(100),
            stats: Stats{
                leaf_count: 0,
                flower_count: 0,
                beehive_count: 0,
                berry_count: 0,
                nut_count: 0,
                birdnest_count: 0,
                squirrel_count: 0,
                moss_count: 0,
                branch_lv1_count: 0,
                branch_lv2_count: 0,
                branch_length3_count: 0,
                branch_length5_count: 0,
                branches_max: 0,
                bounty_max: 0,
            },
            forbidden: HashMap::with_capacity(100),
            cost_multiplier: 1.0,
        };
        globals.reset(ctx);
        Ok(globals)
    }

    fn reset(&mut self, ctx: &mut Context) {
        self.start_time = get_current_time(ctx);
        self.turn_time = get_current_time(ctx);
        self.bounty_amount = 5.0;
        self.life_amount = 0.0;

        self.branches.clear();
        self.root_point = hex::BranchPoint::new(hex::HexPoint::new(0, 1));
        let root_gift_point = hex::GiftPoint::new(hex::HexPoint::new(0, 0));
        let mut root_cell = cell::BranchCell::new(None);
        root_cell.branch_upgrade = 3;
        self.branches.insert(self.root_point, root_cell);

        self.forbidden.clear();
        self.forbidden.insert(root_gift_point, true);

        self.gifts.clear();
        let origin_point = hex::GiftPoint::new(hex::HexPoint::new(0, 0));
        let origin_cell = cell::GiftCell::new(self.root_point);
        self.gifts.insert(origin_point, origin_cell);
    }

    fn branch_parent_branch(&self, branch_point: hex::BranchPoint) -> Option<hex::BranchPoint> {
        let branch_cell = self.branches.get(&branch_point)?;
        let gift_point = branch_cell.parent?;
        let gift_cell = self.gifts.get(&gift_point)?;
        Some(gift_cell.parent)
    }

    fn branch_nth_parent_branch_cell(&self, branch_point: hex::BranchPoint, n: u8) -> Option<cell::BranchCell> {
        if n == 0 {
            self.branches.get(&branch_point).map(|b| *b)
        } else {
            let parent_point = self.branch_parent_branch(branch_point)?;
            self.branch_nth_parent_branch_cell(parent_point, n-1)
        }
    }

    fn branch_nth_parent_branch_cell_or_root(&self, branch_point: hex::BranchPoint, n: u8) -> cell::BranchCell {
        match self.branch_nth_parent_branch_cell(branch_point, n) {
            Some(branch_cell) => branch_cell,
            None => {
                *self.branches.get(&self.root_point).unwrap()
            },
        }
    }

    #[allow(dead_code)]
    fn gift_parent_gift(&self, gift_point: hex::GiftPoint) -> Option<hex::GiftPoint> {
        let gift_cell = self.gifts.get(&gift_point)?;
        let branch_point = gift_cell.parent;
        let branch_cell = self.branches.get(&branch_point)?;
        branch_cell.parent
    }

    fn branch_children(&self, branch_point: hex::BranchPoint) -> Vec<hex::GiftPoint> {
        branch_point.gift_neighbours()
            .iter()
            .map(|g| *g)
            .filter(|g|
                match self.gifts.get(g) {
                    None => false,
                    Some(gift_cell) => gift_cell.parent == branch_point,
                }
            )
            .collect()
    }

    fn gift_children(&self, gift_point: hex::GiftPoint) -> Vec<hex::BranchPoint> {
        gift_point.branch_neighbours()
            .iter()
            .map(|b| *b)
            .filter(|b|
                match self.branches.get(b) {
                    None => false,
                    Some(branch_cell) => branch_cell.parent == Some(gift_point),
                }
            )
            .collect()
    }

    fn prune_branch(&mut self, branch_point: hex::BranchPoint) {
        if let Some(_) = self.branches.get(&branch_point) {
            for gift_point in self.branch_children(branch_point) {
                self.prune_gift(gift_point);
            }

            if let Some(branch_cell) = self.branches.remove(&branch_point) {
                match branch_cell.branch_upgrade {
                    0 => self.stats.branch_lv1_count -= 1,
                    _ => self.stats.branch_lv2_count -= 1,
                };
            }
        }
    }

    fn prune_gift(&mut self, gift_point: hex::GiftPoint) {
        if let Some(_) = self.gifts.get(&gift_point) {
            for branch_point in self.gift_children(gift_point) {
                self.prune_branch(branch_point);
            }
            if let Some(gift_cell) = self.gifts.remove(&gift_point) {
                match gift_cell.gift {
                    Some(cell::Gift::Leaves)   => self.stats.leaf_count     -= 1,
                    Some(cell::Gift::Flowers)  => self.stats.flower_count   -= 1,
                    Some(cell::Gift::Beehive)  => self.stats.beehive_count  -= 1,
                    Some(cell::Gift::Berries)  => self.stats.berry_count    -= 1,
                    Some(cell::Gift::Nuts)     => self.stats.nut_count      -= 1,
                    Some(cell::Gift::Birdnest) => self.stats.birdnest_count -= 1,
                    Some(cell::Gift::Squirrel) => self.stats.squirrel_count -= 1,
                    _ => {},
                };
            }
        }
        if let Some(_) = self.forbidden.get(&gift_point) {
            self.forbidden.remove(&gift_point);
        }
    }

    fn remove_gift(&mut self, gift_point: hex::GiftPoint) {
        if let Some(gift_cell) = self.gifts.get(&gift_point) {
            match gift_cell.gift {
                Some(cell::Gift::Leaves)   => self.stats.leaf_count     -= 1,
                Some(cell::Gift::Flowers)  => self.stats.flower_count   -= 1,
                Some(cell::Gift::Beehive)  => self.stats.beehive_count  -= 1,
                Some(cell::Gift::Berries)  => self.stats.berry_count    -= 1,
                Some(cell::Gift::Nuts)     => self.stats.nut_count      -= 1,
                Some(cell::Gift::Birdnest) => self.stats.birdnest_count -= 1,
                Some(cell::Gift::Squirrel) => self.stats.squirrel_count -= 1,
                _ => {},
            };
        }

        self.gifts
            .entry(gift_point)
            .and_modify(|g| g.gift = None);
        if self.gift_children(gift_point).len() == 0 {
            self.forbidden
                .entry(gift_point)
                .and_modify(|b| *b ^= true)
                .or_insert(true);
        }
    }

    fn display_alert(&mut self, ctx: &mut Context, alert_message: AlertMessage )
    {
        let i: usize = match alert_message {
            AlertMessage::NotEnoughBounty => 0,
            AlertMessage::BranchesTooStrained => 1,
        };
        self.alert_current = Some(i);
        self.alerts[i].until_time = get_current_time(ctx) + Duration::from_millis(2000);
    }
}

impl EventHandler for Globals {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.guitar_channel.update(ctx);
        self.clarinet_channel.update(ctx);
        self.high_pithed_clarinet_channel.update(ctx);
        self.dreamy_bells_channel.update(ctx);
        self.bounty.update(ctx, self.bounty_amount, 0.0f32);
        self.life.update(ctx, 0.0f32, self.life_amount+1.0);

        let now = get_current_time(ctx);
        while (now - self.turn_time) > self.turn_duration { // while loop in case of large discrepancy
            // let basic_amount = 0.1f32; // get this amount even if no life
            // self.bounty_amount = (self.bounty_amount+self.life_amount+basic_amount).min(30.0);
            self.life_amount = life::life_production(&self.gifts);
            self.bounty_amount = (self.bounty_amount + self.life_amount).min(MAX_BOUNTY);
            self.turn_time = self.turn_time + self.turn_duration;

            life::life_cycle(
                &mut self.gifts, &self.branches, &self.forbidden, &mut self.stats
            );
        }

        self.stats.bounty_max = self.stats.bounty_max.max(self.bounty_amount.floor() as usize);
        self.stats.branches_max = self.stats.branches_max.max(self.stats.branch_lv1_count + self.stats.branch_lv2_count);


        // calculate the moss count
        // Need to skip non-tips. Check that children is [] when we get those!
        self.stats.moss_count = 0;
        for (&gift_point, &b) in self.forbidden.iter() {
            if b && self.gift_children(gift_point).len() == 0 {
                self.stats.moss_count += 1;
            }
        }

        for mut achievement in self.achievements.iter_mut() {
            if !achievement.achieved {
                if (achievement.functor)(&self.branches,&self.stats) {
                    achievement.achieved = true;
                }
                break // don't mark an achievment when its hint was never displayed yet
            }
        }

        self.guitar_channel.enable(ctx, self.stats.leaf_count > 0);
        self.clarinet_channel.enable(ctx, self.stats.birdnest_count > 0);
        self.high_pithed_clarinet_channel.enable(ctx, self.stats.beehive_count > 0);
        self.dreamy_bells_channel.enable(ctx, self.stats.squirrel_count > 0);

        ggez::timer::sleep(Duration::from_millis(50));
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::D     => self.cost_multiplier = 0.0,
            Keycode::Escape => ctx.quit().unwrap(),
            _               => (),
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::D     => self.cost_multiplier = 1.0,
            Keycode::R     => self.reset(ctx),
            _              => (),
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        let point = Point2::new(x as f32, y as f32);
        if let Some(in_bounds_point) = hex::HexPoint::from_point(point).is_in_bounds() {
            match button {
                MouseButton::Left => {
                    match in_bounds_point {
                        hex::InBoundsPoint::BranchPoint(branch_point) => {
                            let mut alert_option: Option<AlertMessage> = None;

                            match self.branches.get(&branch_point) {
                                None => {
                                    let gift_neighbours = branch_point.gift_neighbours();
                                    let empty_neighbours: Vec<hex::GiftPoint> = gift_neighbours
                                        .iter()
                                        .map(|g| *g)
                                        .filter(|g| self.gifts.get(g).is_none())
                                        .collect();
                                    let full_neighbours: Vec<hex::GiftPoint> = gift_neighbours
                                        .iter()
                                        .map(|g| *g)
                                        .filter(|g| self.gifts.get(g).is_some())
                                        .collect();
                                    if empty_neighbours.len() == 1 && full_neighbours.len() == 1 {
                                        let empty_neighbour = empty_neighbours[0];
                                        let full_gift_point = full_neighbours[0];
                                        let full_gift_cell = *self.gifts.get(&full_gift_point).unwrap();
                                        let grandparent_cell = self.branch_nth_parent_branch_cell_or_root(full_gift_cell.parent, 2);

                                        if grandparent_cell.branch_upgrade > 0 {
                                            let cost = self.cost_multiplier * life::BASE * 5.0;
                                            if self.bounty_amount >= cost {
                                                // place a new branch
                                                self.assets.branch_place_sound.play().unwrap_or(());
                                                self.bounty_amount -= cost;
                                                self.stats.branch_lv1_count += 1;
                                                let branch_cell = cell::BranchCell::new(Some(full_gift_point));
                                                let gift_cell = cell::GiftCell::new(branch_point);
                                                self.branches.insert(branch_point, branch_cell);
                                                self.gifts.insert(empty_neighbour, gift_cell);
                                                self.forbidden.insert(full_gift_point, true);
                                                if full_gift_cell.gift.is_some() {
                                                    self.remove_gift(full_gift_point);
                                                }

                                                if self.branch_nth_parent_branch_cell(full_gift_cell.parent, 2).is_some() {
                                                    self.stats.branch_length3_count += 1;
                                                }
                                                if self.branch_nth_parent_branch_cell(full_gift_cell.parent, 4).is_some() {
                                                    self.stats.branch_length5_count += 1;
                                                }
                                            } else {
                                                alert_option = Some(AlertMessage::NotEnoughBounty);
                                                //println!("not enough Bounty");
                                            }
                                        } else {
                                            alert_option = Some(AlertMessage::BranchesTooStrained);
                                            //println!("branches are too thin to hold more branches!");
                                        }
                                    } else if empty_neighbours.len() == 2 {
                                        println!("new branches must be attached to the tree");
                                    } else if full_neighbours.len() == 2 {
                                        println!("branches cannot form a cycle");
                                    }
                                },
                                Some(_) => {
                                    let parent_cell = self.branch_nth_parent_branch_cell_or_root(branch_point, 1);
                                    let grandparent_cell = self.branch_nth_parent_branch_cell_or_root(branch_point, 3);
                                    if let Some(branch_cell) = self.branches.get_mut(&branch_point) {
                                        let bounty_amount_ = &mut self.bounty_amount;
                                        if branch_cell.branch_upgrade < parent_cell.branch_upgrade {
                                            if branch_cell.branch_upgrade+1 < grandparent_cell.branch_upgrade {
                                                match branch_cell.branch_upgrade {
                                                    0 => {
                                                        let cost = self.cost_multiplier * life::BASE * 25.0;
                                                        if *bounty_amount_ >= cost {
                                                            // upgrade a branch to level 1
                                                            self.assets.branch_upgrade_sound.play().unwrap_or(());
                                                            *bounty_amount_ -= cost;
                                                            branch_cell.branch_upgrade = 1;
                                                            self.stats.branch_lv2_count += 1;
                                                        } else {
                                                            alert_option = Some(AlertMessage::NotEnoughBounty);
                                                            //println!("not enough Bounty");
                                                        }
                                                    },
                                                    1 => {
                                                        let cost = self.cost_multiplier * life::BASE * 125.0;
                                                        if *bounty_amount_ >= cost {
                                                            // upgrade a branch to level 2
                                                            self.assets.branch_upgrade_sound.play().unwrap_or(());
                                                            *bounty_amount_ -= cost;
                                                            branch_cell.branch_upgrade = 2;
                                                        } else {
                                                            alert_option = Some(AlertMessage::NotEnoughBounty);
                                                            //println!("not enough Bounty");
                                                        }
                                                    },
                                                    2 => {
                                                        let cost = self.cost_multiplier * life::BASE * 625.0;
                                                        if *bounty_amount_ >= cost {
                                                            // upgrade a branch to level 3
                                                            self.assets.branch_upgrade_sound.play().unwrap_or(());
                                                            *bounty_amount_ -= cost;
                                                            branch_cell.branch_upgrade = 3;
                                                        } else {
                                                            alert_option = Some(AlertMessage::NotEnoughBounty);
                                                            //println!("not enough Bounty");
                                                        }
                                                    },
                                                    _ => {
                                                        println!("this branch has already reached its maximum growth");
                                                    },
                                                }
                                            } else {
                                                alert_option = Some(AlertMessage::BranchesTooStrained);
                                                //println!("branches are too thin to hold more branches!");
                                            }
                                        } else {
                                            alert_option = Some(AlertMessage::BranchesTooStrained);
                                            //println!("you have to grow the parent branch first!");
                                        }
                                    }
                                },
                            }
                            if let Some(alert_message) = alert_option {
                                self.display_alert(ctx,alert_message);
                            }
                        },
                        hex::InBoundsPoint::GiftPoint(gift_point) => {
                            match self.gifts.get(&gift_point) {
                                None => {
                                    println!("you cannot place a branch on a cell, only in-between two cells");
                                },
                                Some(gift_cell) => {
                                    match gift_cell.gift {
                                        None => {
                                            println!("you cannot place leaves, you have to let them grow");
                                        },
                                        Some(gift) => {
                                            println!("right-click to release the {:}", gift.singular());
                                        },
                                    }
                                },
                            }
                        },
                    }
                },
                MouseButton::Right => {
                    match in_bounds_point {
                        hex::InBoundsPoint::BranchPoint(branch_point) => {
                            if self.branches.get(&branch_point).is_some() {
                                self.assets.branch_break_sounds.choose(&mut rand::thread_rng()).unwrap().play().unwrap_or(());
                                self.prune_branch(branch_point);
                            }
                        },
                        hex::InBoundsPoint::GiftPoint(gift_point) => {
                            self.assets.gift_release_sound.play().unwrap_or(());
                            self.remove_gift(gift_point);
                        },
                    }
                }
                _ => {}
            }
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: MouseState, x: i32, y: i32, _xrel: i32, _yrel: i32) {
        let hex_point = hex::HexPoint::from_point(Point2::new(x as f32, y as f32));
        self.hover = hex_point.is_in_bounds()
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // must use white for drawing images, otherwise they get tinted
        set_color(ctx, Color::from_rgb(255, 255, 255))?;

        bg::draw_bg(ctx, &self.assets.bg)?;
        hex::draw_hex_grid(ctx, &self.assets.hex)?;
        self.bounty.draw(ctx)?;
        self.life.draw(ctx)?;

        set_color(ctx, Color::from_rgb(0, 0, 0))?; // fix white artifacts around the branches
        for (&branch_point, branch_cell) in self.branches.iter() {
            branch_cell.draw(ctx, &self.assets.cell, branch_point)?;
        }
        set_color(ctx, Color::from_rgb(255, 255, 255))?;
        for (&gift_point, gift_cell) in self.gifts.iter() {
            gift_cell.draw(ctx, &self.assets.cell, gift_point)?;
        }
        set_color(ctx, Color::from_rgb(128, 255, 128))?;
        // Need to skip non-tips. Check that children is [] when we get those!
        for (&gift_point, &b) in self.forbidden.iter() {
            //println!("{:?}", self.gift_children(gift_point).len());
            if b && self.gift_children(gift_point).len() == 0 {
                let image = &self.assets.moss;
                center::draw_centered_image(ctx, image, gift_point.to_point(), 0.0)?;
                //self.assets.dot.draw(ctx, gift_point.to_point(), 0.0)?;
            }
        }
        if let Some(in_bounds_point) = self.hover {
            set_color(ctx, Color::from_rgb(255, 128, 128))?;
            self.assets.dot.draw(ctx, in_bounds_point.to_point(), 0.0)?;

            // neighbour-debugging code; uncomment me, it's fun!
            //set_color(ctx, Color::from_rgb(128, 128, 255))?;
            //match in_bounds_point {
            //    hex::InBoundsPoint::BranchPoint(branch_point) => {
            //        for n in branch_point.gift_neighbours() {
            //            self.assets.dot.draw(ctx, n.to_point(), 0.0)?;
            //        }
            //    },
            //    hex::InBoundsPoint::GiftPoint(gift_point) => {
            //        for n in gift_point.branch_neighbours() {
            //            self.assets.dot.draw(ctx, n.to_point(), 0.0)?;
            //        }
            //        for n in gift_point.gift_neighbours() {
            //            self.assets.dot.draw(ctx, n.to_point(), 0.0)?;
            //        }
            //    },
            //}
        }

        if let Some(alert_current) = self.alert_current {
            set_color(ctx, Color::from_rgb(255, 0, 0))?;
            let center = Point2::new(
                WINDOW_WIDTH as f32 / 2.0,
                WINDOW_HEIGHT as f32 - 20.0,
            );
            let text = Text::new(ctx,self.alerts[alert_current].message, &self.assets.font)?;
            text::draw_centered_text(ctx, &text, center, 0.0)?;
            if self.alerts[alert_current].until_time < get_current_time(ctx) {
                self.alert_current = None
            }
        }
        else {
            for achievement in self.achievements.iter() {
                if !achievement.achieved {
                    set_color(ctx, Color::from_rgb(255, 255, 255))?;
                    let center = Point2::new(
                        WINDOW_WIDTH as f32 / 2.0,
                        WINDOW_HEIGHT as f32 - 20.0,
                    );
                    let text = Text::new(ctx,achievement.message, &self.assets.font)?;
                    text::draw_centered_text(ctx, &text, center, 0.0)?;
                    break;
                }
            }
        }


        //if get_current_time(ctx) - self.start_time > Duration::from_millis(1000) {
        //    self.start_time = get_current_time(ctx);
        //    println!("FPS: {}", ggez::timer::get_fps(ctx));
        //}
        present(ctx);
        timer::yield_now();

        Ok(())
    }
}

pub fn main() {
    let ctx = &mut Context::load_from_conf(
        GAME_NAME,
        "Michaelson Britt, Samuel Gélineau, Dylan Khor, Zhentao Li, Kyla Squires, and Farren Wang",
        ggez::conf::Conf {
            window_mode: ggez::conf::WindowMode {
                width:  WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                .. Default::default()
            },
            window_setup: ggez::conf::WindowSetup {
                title: GAME_NAME.to_owned(),
                .. Default::default()
            },
            .. Default::default()
        },
    ).unwrap();

    let globals = &mut Globals::new(ctx).unwrap();
    globals.guitar_channel.source.play().unwrap_or(());
    globals.clarinet_channel.source.play().unwrap_or(());
    globals.high_pithed_clarinet_channel.source.play().unwrap_or(());
    globals.guitar_channel.source.play().unwrap_or(());

    run(ctx, globals).unwrap();
}
