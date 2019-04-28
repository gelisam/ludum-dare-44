extern crate counter;

use std::collections::HashMap;
use cell::Gift::*;
use counter::Counter;
use cell;
use hex;

pub const base: f32 = 0.25;

macro_rules !get {
    ($map:expr, $value:expr) => (*$map.get(&Some($value)).unwrap_or(&0));
    ($map:expr) => (*$map.get(&None).unwrap_or(&0));
}

pub fn life_cycle(gifts: &mut HashMap<hex::GiftPoint, cell::GiftCell>,
                  branches: &HashMap<hex::BranchPoint, cell::BranchCell>,
                  forbidden: &HashMap<hex::GiftPoint, bool>,) {
    let gifts_old = gifts.clone(); // deep copy of old state
    for (gift_point, _) in gifts_old.iter() {
        // Should filter!
        if *forbidden.get(&gift_point).unwrap_or(&false) {
            continue
        }
        let mut counts = gift_point.gift_neighbours()
            .iter()
            .map(|adj_point| match gifts_old.get(&adj_point){
                Some(gp) => gp.gift,
                _ => None})
            .collect::<Counter<_, u8>>();
        // println!("{:?}", counts);
        // println!("{:?}", *counts.get(&Some(cell::Gift::Berries)).unwrap_or(&0) >= 2);
        // println!("{:?}", get![counts]);
        let mut adjacent_branches_upgrade = 0u8;

        for adjacent_point in gift_point.branch_neighbours() {
            if let Some(adjacent_cell) = branches.get(&adjacent_point)
            {
                if adjacent_cell.branch_upgrade > 0 {
                    adjacent_branches_upgrade += 1
                }
            }
        };
        // Should be gifts_old?
        if let Some(gift_cell) = gifts.get_mut(&gift_point)
        {
            gift_cell.gift = match gift_cell.gift {
                None => {
                    if get![counts, Berries] >=2 {
                        Some(Birdnest)
                    }
                    else if get![counts, Flowers] >=2 {
                        Some(Beehive)
                    }
                    else if get![counts] >=2 {
                        Some(Leaves)
                    }
                    else {
                        None
                    }
                }
                Some(Leaves) => {
                    if get![counts] == 0 {
                        None
                    }
                    // More GoL fun but more unpredictable.
                    //else if get![counts, Flowers] >=2 {
                    //    Some(Beehive)
                    //}
                    else if get![counts, Leaves]>=2 {
                        Some(Flowers)
                    }
                    else {
                        Some(Leaves)
                    }
                }
                Some(Flowers) => {
                    if get![counts, Leaves] == 0 {
                        None
                    }
                    else if (adjacent_branches_upgrade>0) & (get![counts, Flowers]>0) & (get![counts, Leaves]>0) {
                        Some(Nuts)
                    }
                    else if (get![counts, Beehive]>0) & (get![counts, Leaves]>=2) {
                        Some(Berries)
                    }
                    else {
                        Some(Flowers)
                    }
                }
                Some(Berries) => {
                    if (get![counts, Beehive]==0) | (get![counts, Flowers]==0) | (get![counts, Leaves]==0) {
                        Some(Flowers)
                    }
                    else {
                        Some(Berries)
                    }
                }
                Some(Nuts) => {
                    if (adjacent_branches_upgrade==0) | (get![counts, Flowers]==0) | (get![counts, Leaves]==0) {
                        Some(Flowers)
                    }
                    else {
                        Some(Nuts)
                    }
                }
                Some(Beehive) => {
                    if get![counts, Flowers] == 0 {
                        None
                    }
                    else {
                        Some(Beehive)
                    }
                }
                Some(Birdnest) => {
                    if get![counts, Berries]<2 {
                        None
                    }
                    else {
                        Some(Birdnest)
                    }
                }
            }
        }
        else {
            println!("error: gift cell vanished during update"); // cryptic error bwahaha
        }
    }
}

pub fn life_production(gifts: &HashMap<hex::GiftPoint, cell::GiftCell>) -> f32{
    let total: f32 = gifts.iter()
        .map(|(_, gift)| match gift.gift {
            Some(Leaves) => 1f32,
            Some(Flowers) => 1f32,
            Some(Beehive) => 4f32,
            Some(Birdnest) => 0f32,
            Some(Squirrel) => 8f32,
            _ => 0f32,
        })
        .sum();
    let multiplier: f32 = gifts.iter()
        .map(|(_, gift)| match gift.gift {
            Some(Birdnest) => 0.5f32,
            _ => 0.0f32})
        .sum();
    return base * (1f32 + (1f32 + multiplier) * total);
}
