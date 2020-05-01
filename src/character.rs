use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::{Action, Direction, MAP_SIZE};
use crate::map::Map;
use crate::texture_handler::TextureHandler;

pub struct Character<'a> {
    pub action: Action,
    pub x: i32,
    pub y: i32,
    pub total_health: u32,
    pub health: u32,
    pub total_mana: u32,
    pub mana: u32,
    pub total_stamina: u32,
    pub stamina: u32,
    pub xp_to_next_level: u32,
    pub xp: u32,
    pub texture_handler: TextureHandler<'a>,
}

impl<'a> Character<'a> {
    pub fn move_result(&self, dir: Direction) -> ((i32, i32), (i32, i32)) {
        let (info, _) = &self.texture_handler.actions_moving[dir as usize];
        match dir {
            Direction::Front => ((0, 0), (info.height() as i32 / 2, 1)),
            Direction::Back => ((0, 0), (info.height() as i32 / -4, -1)),
            Direction::Left => ((info.width() as i32 / -2, -1), (0, 0)),
            Direction::Right => ((info.width() as i32 / 2, 1), (0, 0)),
        }
    }

    pub fn inner_apply_move(&mut self, map: &Map) -> bool {
        if self.action.movement.is_none() {
            return false;
        }
        let ((mut x, mut x_add), (mut y, mut y_add)) = self.move_result(self.action.direction);
        if let Some(second) = self.action.secondary {
            let ((tmp_x, tmp_x_add), (tmp_y, tmp_y_add)) = self.move_result(second);
            x += tmp_x;
            x_add += tmp_x_add;
            y += tmp_y;
            y_add += tmp_y_add;
        }
        if self.y + y >= map.y + MAP_SIZE as i32 * 8
            || self.y + y < map.y
            || self.x + x >= map.x + MAP_SIZE as i32 * 8
            || self.x + x < map.x
        {
            return false;
        }
        let map_pos = (self.y + y - map.y) / 8 * MAP_SIZE as i32 + (self.x + x - map.x) / 8;
        println!(
            "{}|{} => ({}, {})",
            map.data.len(),
            map_pos,
            self.x + x,
            self.y + y
        );
        if map_pos < 0 || map_pos as usize >= map.data.len() {
            return false;
        } else if map.data[map_pos as usize] != 0 {
            println!("/!\\ {:?}", map.data[map_pos as usize]);
            return false;
        }
        self.x += x_add;
        self.y += y_add;
        true
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, is_running: bool, screen: &Rect) {
        let (tile_x, tile_y, tile_width, tile_height) =
            self.action.get_current(is_running, &self.texture_handler);
        if (self.x + tile_width < screen.x || self.x > screen.x + screen.width() as i32) &&
            (self.y + tile_height < screen.y || self.y > screen.y + screen.height() as i32) {
            // No need to draw if we don't see the character.
            return;
        }
        canvas
            .copy(
                &self.texture_handler.texture,
                Rect::new(tile_x, tile_y, tile_width as u32, tile_height as u32),
                Rect::new(self.x - screen.x, self.y - screen.y, tile_width as u32, tile_height as u32),
            )
            .expect("copy character failed");

        // We now update the animation!
        if let Some(ref mut pos) = self.action.movement {
            *pos += 1;
        } else {
            if self.stamina < self.total_stamina {
                self.stamina += 1;
            }
            return;
        }
    }
}