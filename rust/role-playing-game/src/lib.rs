// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health != 0{
            return None;
        }

        match self.level {
            level if level >= 10 => Some(Player { health: 100, mana: Some(100), level: self.level }),
            _ => Some(Player { health: 100, mana: None, level: self.level })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana.is_none() {
            self.health = if self.health < mana_cost { 0 } else { self.health - mana_cost };
            0
        } else if mana_cost > self.mana.unwrap() {
            0
        } else {
            self.mana = Some(self.mana.unwrap() - mana_cost);
            mana_cost * 2
        }
    }
}
