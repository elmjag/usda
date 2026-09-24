#[derive(Debug)]
pub enum AttributeType {
    String,
    Token,
    Int,
    Float,
    Float3,
    Point3f,
    Array(Box<AttributeType>),
}

impl AttributeType {
    pub fn new_array(base_type: Self) -> Self {
        Self::Array(Box::new(base_type))
    }

    pub fn base_type(&self) -> &Self {
        match self {
            AttributeType::Array(base) => base,
            AttributeType::Float3 => &AttributeType::Float,
            AttributeType::Point3f => &AttributeType::Float,
            _ => panic!("no base type for {self:?}"),
        }
    }
}

#[derive(Debug)]
pub enum AttributeValue {
    String(String),
    Token(String),
    Int(i32),
    Float(f32),
    Float3(f32, f32, f32),
    Point3f(f32, f32, f32),
    Array(Box<Vec<AttributeValue>>),
}

impl AttributeValue {
    pub fn new_array(elements: Vec<AttributeValue>) -> Self {
        Self::Array(Box::new(elements))
    }
}

impl TryFrom<&AttributeValue> for f32 {
    type Error = &'static str;

    fn try_from(value: &AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::Float(f) => Ok(*f),
            _ => Err("AttributeValue is not a Float"),
        }
    }
}

#[derive(Debug)]
pub struct Attribute {
    name: String,
    value: AttributeValue,
}

impl Attribute {
    pub fn new(name: &str, value: AttributeValue) -> Self {
        Self {
            name: String::from(name),
            value,
        }
    }
}

#[derive(Debug)]
pub struct Prim {
    name: String,
    type_name: Option<String>,
    attributes: Vec<Attribute>,
    children: Vec<Prim>,
}

impl Prim {
    pub fn new(
        name: &str,
        type_name: Option<&str>,
        attributes: Vec<Attribute>,
        children: Vec<Prim>,
    ) -> Self {
        let type_name = if let Some(t) = type_name {
            Some(String::from(t))
        } else {
            None
        };

        Self {
            name: String::from(name),
            type_name,
            attributes,
            children,
        }
    }
}
