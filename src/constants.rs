use bevy::prelude::*;

// Configuration constants
pub const ANIMATE_ONLY_WHEN_MOVING: bool = true; // true: animate only when moving, false: always animate
pub const FLIP_SPRITE_ON_DIRECTION: bool = true; // true: flip sprite based on direction, false: keep original

// Sprite settings
pub const SPRITE_COLUMNS: usize = 4;
pub const SPRITE_ROWS: usize = 1;
pub const FRAME_WIDTH: f32 = 108.0;
pub const FRAME_HEIGHT: f32 = 96.0;

// Gameplay settings
pub const ANIMATION_SPEED: f32 = 0.1; // Frame interval (seconds)
pub const MOVE_SPEED: f32 = 150.0; // Movement speed (pixels/second)

// Window settings
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const SPRITE_SCALE: f32 = 1.0;

// UI colors
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
