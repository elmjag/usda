use crate::Prim;
use crate::usda::loader;

#[derive(Debug)]
pub struct Stage {
    prims: Vec<Prim>,
}

pub struct PrimRange<'a> {
    prims: Vec<&'a Prim>,
}

impl<'a> Iterator for PrimRange<'a> {
    type Item = &'a Prim;

    fn next(&mut self) -> Option<Self::Item> {
        self.prims.pop()
    }
}

impl Stage {
    pub fn open(file_path: &str) -> Self {
        Self {
            prims: loader::load_usda_file(file_path),
        }
    }

    pub fn traverse(&self) -> PrimRange<'_> {
        PrimRange {
            prims: self.prims.iter().rev().collect(),
        }
    }
}
