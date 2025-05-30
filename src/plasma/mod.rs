use crate::physmodel::spaceobject::Spaceobject;

#[derive(Debug, Copy, Clone)]
pub struct Plasma {
    source: u32,
    pub object: Spaceobject,
}

impl Plasma {
    pub fn new(source_id: u32) -> Plasma {
        let mut object = Spaceobject::new();
        object.set_size(5);
        Plasma { 
            source: source_id,
            object: object
        }
    }
    pub fn get_source_id(&self) -> u32 {
        self.source
    }
    pub fn is_on_boundary(&self, width: u32, height: u32) -> bool {
        let p = self.object.get_pos();
        let r = self.object.get_size();
        p.0 <= r || p.1 <= r || p.0 >= width-r || p.1 >= height-r
    }
}