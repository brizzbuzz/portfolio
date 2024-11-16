#[derive(Clone)]
pub struct ForestSpirit {
    pub cx: i32,
    pub cy: i32,
    pub radius: i32,
    pub glow_radius: i32,
    pub animation_delay: i32,
}

impl ForestSpirit {
    pub fn new(cx: i32, cy: i32, radius: i32, glow_radius: i32, animation_delay: i32) -> Self {
        Self {
            cx,
            cy,
            radius,
            glow_radius,
            animation_delay,
        }
    }

    pub fn default_spirits() -> Vec<ForestSpirit> {
        vec![
            ForestSpirit::new(150, 150, 12, 8, 0),
            ForestSpirit::new(850, 250, 15, 11, 2000),
            ForestSpirit::new(450, 750, 10, 6, 1000),
            ForestSpirit::new(750, 550, 14, 10, 3000),
            ForestSpirit::new(250, 450, 11, 7, 2000),
            ForestSpirit::new(650, 350, 13, 9, 1000),
        ]
    }
}
