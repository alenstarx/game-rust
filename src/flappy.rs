extern crate sdl2;

use std::ops::{Deref, DerefMut};

use std::path::Path;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use std::cell::RefCell;
use std::rc::Rc;
use sdl2::keyboard::Keycode;

use display::Displayable;
use sprite::Sprite;
use scene::Scene;

pub struct Bird {
    speed: f32,
    xaccelerate: f32,
    yaccelerate: f32,
    sprite: Sprite,
}

impl Bird {
    // add code here
    pub fn new(renderer: &Renderer) -> Bird {
        Bird {
            speed: 0.0,
            xaccelerate: 0.0,
            yaccelerate: 0.0,
            sprite: Sprite::new(renderer,
                                &["res/imgs/bird_frame_1.png",
                                  "res/imgs/bird_frame_2.png",
                                  "res/imgs/bird_frame_3.png",
                                  "res/imgs/bird_frame_4.png"]),
        }
    }
}

impl Displayable for Bird {
    // add code here
    fn update(&mut self) {
        self.sprite.update();
    }

    fn paint(&self, renderer: &mut Renderer) {
        self.sprite.paint(renderer);
    }
}

impl Deref for Bird {
    type Target = Sprite;

    fn deref<'a>(&'a self) -> &'a Sprite {
        &self.sprite
    }
}

impl DerefMut for Bird {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Sprite {
        &mut self.sprite
    }
}

pub struct FlappyScene {
    scroll_step: u32,
    scroll_x1: u32,
    scroll_x2: u32,
    scroll_w1: u32,
    scroll_w2: u32,
    speed: f32,
    slice_number: u8,
    scroll_index: u8,
    width: u32,
    height: u32,
    scene: Scene,
}

impl FlappyScene {
    // add code here
    pub fn new(renderer: &Renderer, w: u32, h: u32) -> FlappyScene {
        let mut bird = Rc::new(RefCell::new(Bird::new(renderer)));
        bird.borrow_mut().set_interval(0.3);
        bird.borrow_mut().start();

        let mut scene = Scene::new(renderer, "res/imgs/background.png");
        scene.add_child(bird);
        scene.set_interval(0.3);

        FlappyScene {
            scroll_step: 1,
            scroll_x1: 0,
            scroll_x2: w,
            scroll_w1: 0,
            scroll_w2: 0,
            speed: 2.0,
            slice_number: 3,
            scroll_index: 0,
            width: w,
            height: h,
            scene: scene,
        }
    }

    pub fn start(&mut self) {
        // TODO
    }

    pub fn stop(&mut self) {
        // TODO
    }
}


impl Deref for FlappyScene {
    type Target = Scene;

    fn deref<'a>(&'a self) -> &'a Scene {
        &self.scene
    }
}

impl DerefMut for FlappyScene {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Scene {
        &mut self.scene
    }
}

impl Displayable for FlappyScene {
    // add code here
    fn update(&mut self) {

        if self.get_elapsed() >= self.get_interval() {
            // TODO
            self.cursor_incr();
            self.update_time();
        }
        self.scene.update();

        let sz = self.get_texture_size(0).unwrap();

        self.scroll_x1 += self.scroll_step;
        if self.scroll_x1 > sz.0 {
            self.scroll_x1 = 0;
        }

        if self.scroll_x1 > (sz.0 - self.width) {
            self.scroll_w1 = sz.0 - self.scroll_x1;
        } else {
            self.scroll_w1 = self.width;
        }

        self.scroll_x2 += self.scroll_step;
        if (self.scroll_x2 - self.width) > sz.0 {
            self.scroll_x2 = self.width;
        }

        self.scroll_w2 = self.width - self.scroll_w1;
    }

    fn paint(&self, renderer: &mut Renderer) {

        let mut current_texture = self.get_texture(0).unwrap();
        renderer.copy(&mut current_texture,
                      Some(Rect::new(self.scroll_x1 as i32, 0, self.scroll_w1, self.height)),
                      Some(Rect::new(0, 0, self.scroll_w1, self.height)))
                .expect("background should have rendered.");

        if self.scroll_w2 > 0 {
            renderer.copy(&mut current_texture,
                          Some(Rect::new(0, 0, self.scroll_w2, self.height)),
                          Some(Rect::new((self.width - self.scroll_w2) as i32,
                                         0,
                                         self.scroll_w2,
                                         self.height)))
                    .expect("background should have rendered.");
        }

        self.scene.paint_child(renderer);
    }
}