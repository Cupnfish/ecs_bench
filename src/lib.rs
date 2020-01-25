pub mod pos_vel {

    /// Entities with velocity and position component.
    pub const N_POS_PER_VEL: usize = 10;

    /// Entities with position component only.
    pub const N_POS: usize = 10000;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Position {
        pub x: f32,
        pub y: f32,
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Velocity {
        pub dx: f32,
        pub dy: f32,
    }

}

pub mod parallel {

    pub const N: usize = 10000;

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct R {
        pub x: f32,
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct W1 {
        pub x: f32,
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct W2 {
        pub x: f32,
    }
}
