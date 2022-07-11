use super::Attribute;

#[derive(Debug,Clone)]
pub enum Vertices{
    /// Attirbutes and stride in bytes
    Interleaved(Vec<Attribute>,usize),
    /// Attribute
    Separate(Vec<Attribute>)
}

impl Vertices{
    /// Convert interleaved vertex format to separate vertex format
    pub fn into_separate(self) -> Self {
        match self {
            Vertices::Interleaved(_, _) => todo!(),
            Vertices::Separate(_) => self,
        }
    }

    pub fn into_interleaved(self) -> Self {
        match self {
            Vertices::Interleaved(_, _) => self,
            Vertices::Separate(_) => todo!(),
        }
    }
}