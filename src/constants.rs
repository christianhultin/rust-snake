pub const GRID_SIZE: (i16, i16) = (60, 40);
pub const GRID_CELL_SIZE: (i16, i16) = (18, 18);
pub const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);
pub const UDPATES_PER_SECOND: f32 = 50.0;
pub const MILLIS_PER_UPDATE: u64 = (1000.0 / UDPATES_PER_SECOND) as u64;
