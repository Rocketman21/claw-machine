use bevy::prelude::*;

pub const COL_GROUP_ALL: u32 = u32::MAX;
pub const COL_GROUP_CLAW: u32 = 0b0001;
pub const COL_GROUP_GLASS: u32 = 0b0010;
pub const COL_GROUP_CLAW_STOPPER: u32 = 0b0100;
pub const COL_GROUP_TOY_EJECTION_SHELV: u32 = 0b1000;
pub const COL_GROUP_EJECTED_TOY: u32 = 0b10000;
pub const COL_GROUP_BOTTOM_GLASS: u32 = 0b100000;

pub const PURPLE_COLOR: Color = Color::rgb(
    114.0 / 255.0,
    0.0,
    163.0 / 255.0
);
