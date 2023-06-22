use std::collections::HashMap;
use std::rc::Rc;

use super::capy_texture::CapyTexture;
use super::capy_texture::TextureName;

pub struct TextureManager {
    pub textures: HashMap<TextureName, Rc<dyn CapyTexture>>
}

impl TextureManager {
    pub fn new() -> Self {
        Self { textures: HashMap::new() }
    }

    pub fn add(&mut self, texture_name: TextureName, texture: Rc<dyn CapyTexture>) {
        self.textures.insert(texture_name, texture);
    }

    pub fn find(&self, texture_name: TextureName) -> Option<Rc<dyn CapyTexture>> {
        
        let text = self.textures.get(&texture_name);
        match text {
            Some(t) => {
                return Some(t.clone());
            }
            None => {
                return None;
            }
        }
        //self.textures.iter().find(|&t| t.name == texture_name)
    }
}