type GearMap = Vec<Vec<Option<GearCell>>>;

#[derive(Clone)]
struct GearCell {
    gear: i32,
    can_u: bool,
    can_d: bool,
    can_r: bool,
    can_l: bool,
    can_l_special: bool,
}

pub struct GearstickState {
    map: GearMap,
    pub mouse_pos: (i32, i32),
    gear_pos: (usize, usize),
    dist: i32,
    pub special: bool,
}

impl GearstickState {
    pub fn new_6_speed(dist: i32) -> Self {
        Self {
            map: Self::gear_map_6_speed(),
            dist,
            gear_pos: (2, 1),
            mouse_pos: (0, 0),
            special: false,
        }
    }

    pub fn new_5_speed(dist: i32) -> Self {
        Self {
            map: Self::gear_map_5_speed(),
            dist,
            gear_pos: (2, 1),
            mouse_pos: (0, 0),
            special: false,
        }
    }

    pub fn get_gear(&self) -> i32 {
        self.map[self.gear_pos.1][self.gear_pos.0]
            .as_ref()
            .unwrap()
            .gear
    }

    pub fn update(&mut self, delta: (i32, i32)) {
        let cell = self.map[self.gear_pos.1][self.gear_pos.0].as_ref().unwrap();
        self.mouse_pos.0 += delta.0;
        self.mouse_pos.1 += delta.1;
        if !cell.can_u && self.mouse_pos.1 < 0 || !cell.can_d && self.mouse_pos.1 > 0 {
            self.mouse_pos.1 = 0;
        }
        if !(cell.can_l || cell.can_l_special && self.special) && self.mouse_pos.0 < 0
            || !cell.can_r && self.mouse_pos.0 > 0
        {
            self.mouse_pos.0 = 0;
        }
        if self.mouse_pos.0 > self.dist {
            self.gear_pos.0 += 1;
            self.mouse_pos.0 -= 2 * self.dist;
        }
        if self.mouse_pos.0 < -self.dist {
            self.gear_pos.0 -= 1;
            self.mouse_pos.0 += 2 * self.dist;
        }
        if self.mouse_pos.1 > self.dist {
            self.gear_pos.1 += 1;
            self.mouse_pos.1 -= 2 * self.dist;
        }
        if self.mouse_pos.1 < -self.dist {
            self.gear_pos.1 -= 1;
            self.mouse_pos.1 += 2 * self.dist;
        }
    }

    fn gear_map_6_speed() -> GearMap {
        let width = 4;
        let height = 3;
        let mut map: GearMap = vec![vec![None; width]; height];
        map[1][0] = Some(GearCell {
            gear: 0,
            can_u: true,
            can_d: false,
            can_r: true,
            can_l: false,
            can_l_special: false,
        });
        map[1][1] = Some(GearCell {
            gear: 0,
            can_u: true,
            can_d: true,
            can_r: true,
            can_l: false,
            can_l_special: true,
        });
        map[1][2] = Some(GearCell {
            gear: 0,
            can_u: true,
            can_d: true,
            can_r: true,
            can_l: true,
            can_l_special: false,
        });
        map[1][3] = Some(GearCell {
            gear: 0,
            can_u: true,
            can_d: true,
            can_r: false,
            can_l: true,
            can_l_special: false,
        });
        map[0][0] = Some(GearCell {
            gear: -1,
            can_u: false,
            can_d: true,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[0][1] = Some(GearCell {
            gear: 1,
            can_u: false,
            can_d: true,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[0][2] = Some(GearCell {
            gear: 3,
            can_u: false,
            can_d: true,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[0][3] = Some(GearCell {
            gear: 5,
            can_u: false,
            can_d: true,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[2][1] = Some(GearCell {
            gear: 2,
            can_u: true,
            can_d: false,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[2][2] = Some(GearCell {
            gear: 4,
            can_u: true,
            can_d: false,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map[2][3] = Some(GearCell {
            gear: 6,
            can_u: true,
            can_d: false,
            can_r: false,
            can_l: false,
            can_l_special: false,
        });
        map
    }

    fn gear_map_5_speed() -> GearMap {
        let mut map = Self::gear_map_6_speed();
        map[1][3] = Some(GearCell {
            gear: 0,
            can_u: true,
            can_d: false,
            can_r: false,
            can_l: true,
            can_l_special: false,
        });
        map[2][3] = None;
        map
    }

    pub fn print_gear_map(&self) {
        let s = |b| if b { "1" } else { "0" };
        for row in self.map.iter() {
            for cell in row {
                if let Some(cell) = cell {
                    print!(
                        "{}{}{}{}{} ",
                        cell.gear,
                        s(cell.can_u),
                        s(cell.can_r),
                        s(cell.can_d),
                        if cell.can_l_special {
                            "s"
                        } else {
                            s(cell.can_l)
                        }
                    )
                } else {
                    print!("      ")
                }
            }
            println!()
        }
    }
}
