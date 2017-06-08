extern crate sdl2;

use std::ops::{Deref, DerefMut};

use std::path::Path;
use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;

use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::render::Texture;
use sdl2::render::BlendMode;
use sdl2::image::LoadTexture;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use node::Node;
use display::Displayable;

pub struct Layer {
    visible: bool,
    children: Vec<Rc<RefCell<Displayable>>>,
    node: Node,
}

impl Layer {
    pub fn new(renderer: &Renderer, path: &str) -> Layer {
        Layer {
            visible: true,
            children: Vec::new(),
            node: Node::new(renderer, &[path]),
        }
    }
}

// impl Displayable for Scene {
// fn on_key_down(&mut self, event: &Event) {
// TODO: allow cancel propagating events based on logic in parent.
// for child in &self.children {
// child.borrow_mut().on_key_down(event);
// }
// }
//
// fn update(&mut self) {
// TODO: allow cancel propagating events based on logic in parent.
// for child in &self.children {
// child.borrow_mut().update();
// }
//
// Nothing to do for the background at this point sucka.
// TODO
// }
//
// fn paint(&self, renderer: &mut Renderer) {
// if self.visible {
// self.spirits.paint(renderer);
//
// for child in &self.children {
// child.borrow_mut().paint(renderer);
// }
// }
// }
// }
//

impl Deref for Layer {
    type Target = Node;

    fn deref<'a>(&'a self) -> &'a Node {
        &self.node
    }
}

impl DerefMut for Layer {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Node {
        &mut self.node
    }
}
