use crate::physmodel::spaceobject::Spaceobject;

pub struct Plasma {
    source: u32,
    pub object: Spaceobject,
}

impl Plasma {
    pub fn new(source_id: u32) -> Plasma {
        Plasma { 
            source: source_id,
            object: Spaceobject::new()
        }
    }
    pub fn get_source_id(&self) -> u32 {
        self.source
    }
}