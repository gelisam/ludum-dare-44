extern crate counter;

use std::collections::HashMap;
use cell::Gift::*;
use counter::Counter;
use cell;
use hex;

macro_rules !get {
    ($map:expr, $value:expr) => (*$map.get(&Some($value)).unwrap_or(&0));
    ($map:expr) => (*$map.get(&None).unwrap_or(&0));
}

pub fn life_cycle(gifts: &mut HashMap<hex::GiftPoint, cell::GiftCell>,
                  branches: &HashM1ap<hex::BranchPoint, cell::BranchCell>) {
    let gifts_old = gifts.clone(); // deep copy of old state
    for (gift_point, _) in gifts_old.iter() {
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
                    if get![counts, Flowers] >=2 {
                        Some(cell::Gift::Birdnest)
                    }
                    else if get![counts, Flowers] >=2 {
                        Some(cell::Gift::Beehive)
                    }
                    else if get![counts] >=2 {
                        Some(cell::Gift::Leaves)
                    }
                    else {
                        None
                    }
                }
                Some(cell::Gift::Leaves) => {
                    if get![counts] == 0 {
                        None
                    }
                    else if get![counts, Flowers] >=2 {
                        Some(cell::Gift::Beehive)
                    }
                    else if get![counts, Leaves]>=2 {
                        Some(cell::Gift::Flowers)
                    }
                    else {
                        Some(cell::Gift::Leaves)
                    }
                }
                Some(cell::Gift::Flowers) => {
                    if get![counts, Leaves] == 0 {
                        None
                    }
                    else if (adjacent_branches_upgrade>0) & (get![counts, Flowers]>0) & (get![counts, Leaves]>0) {
                        Some(cell::Gift::Nuts)
                    }
                    else if (get![counts, Beehive]>0) & (get![counts, Leaves]>=2) {
                        Some(cell::Gift::Berries)
                    }
                    else {
                        Some(cell::Gift::Flowers)
                    }
                }
                Some(cell::Gift::Berries) => {
                    if (get![counts, Beehive]==0) | (get![counts, Flowers]==0) | (get![counts, Leaves]==0) {
                        Some(cell::Gift::Flowers)
                    }
                    else {
                        Some(cell::Gift::Berries)
                    }
                }
                Some(cell::Gift::Nuts) => {
                    if (adjacent_branches_upgrade==0) | (get![counts, Flowers]==0) | (get![counts, Leaves]==0) {
                        Some(cell::Gift::Flowers)
                    }
                    else {
                        Some(cell::Gift::Nuts)
                    }
                }
                Some(cell::Gift::Beehive) => {
                    if get![counts, Flowers] == 0 {
                        None
                    }
                    else {
                        Some(cell::Gift::Beehive)
                    }
                }
                Some(cell::Gift::Birdnest) => {
                    if get![counts, Berries]<2 {
                        None
                    }
                    else {
                        Some(cell::Gift::Birdnest)
                    }
                }
            }
        }
        else {
            println!("error: gift cell vanished during update"); // cryptic error bwahaha
        }
    }
}
