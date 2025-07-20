use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &mut Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    use std::cmp::{max, min};
    for y in max(camera.top_y, 0)..min(camera.bottom_y, SCREEN_HEIGHT) {
        for x in max(camera.left_x, 0)..min(camera.right_x, SCREEN_WIDTH) {
            let idx = Map::map_idx(x, y);
            let point = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            let glyph = match map.tiles[idx] {
                TileType::Floor => to_cp437('.'),
                TileType::Wall => to_cp437('#'),
            };

            draw_batch.set(point - offset, ColorPair::new(WHITE, BLACK), glyph);
        }
    }

    draw_batch.submit(0).expect("Batching Error")
}
