use super::DataType;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum AttributeType{
    Position,
    Normal,
    Color,
    TexCoord(u32),
}

#[derive(Debug,Clone)]
pub struct Attribute {
    /// The type of attribute
    pub ty: AttributeType,
    /// The data type of the attribute
    pub data_type: DataType,
    /// The number of elements in the attribute
    /// Attribute Size = Data Type Size * Number of Elements
    pub count: usize,
    /// The offset of the attribute in bytes
    pub offset: usize
}
