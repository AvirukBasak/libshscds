use crate::traits::{self, RefC};

/// Converts the given data into a `Data` instance using `shsc::Data::from`.
/// ### Arguments
/// * `data` - The input data to be converted.
/// This macro is a convenient wrapper around `shsc::Data::from`.
/// ### Examples
/// ```
/// let my_data = 42;
/// let data_instance = shsc::todata!(my_data);
/// ```
#[macro_export]
macro_rules! todata {
    ($data:expr) => {
        $crate::Data::from($data)
    };
}

pub enum DataTypes {
    BOOL(bool),
    UINT8(u8),
    UINT16(u16),
    UINT32(u32),
    UINT64(u64),
    INT8(i8),
    INT16(i16),
    INT32(i32),
    INT64(i64),
    FLOAT64(f64),
    CHAR(char),
    STRING(crate::String),
    LIST(crate::List),
    MAP(crate::Map),
    NULL,
}

impl DataTypes {
    pub fn typename(&self) -> std::string::String {
        match self {
            DataTypes::BOOL(_) => std::string::String::from("BOOL"),
            DataTypes::UINT8(_) => std::string::String::from("UINT8"),
            DataTypes::UINT16(_) => std::string::String::from("UINT16"),
            DataTypes::UINT32(_) => std::string::String::from("UINT32"),
            DataTypes::UINT64(_) => std::string::String::from("UINT64"),
            DataTypes::INT8(_) => std::string::String::from("INT8"),
            DataTypes::INT16(_) => std::string::String::from("INT16"),
            DataTypes::INT32(_) => std::string::String::from("INT32"),
            DataTypes::INT64(_) => std::string::String::from("INT64"),
            DataTypes::FLOAT64(_) => std::string::String::from("FLOAT64"),
            DataTypes::CHAR(_) => std::string::String::from("CHAR"),
            DataTypes::STRING(_) => std::string::String::from("STRING"),
            DataTypes::LIST(_) => std::string::String::from("LIST"),
            DataTypes::MAP(_) => std::string::String::from("MAP"),
            DataTypes::NULL => std::string::String::from("NULL"),
        }
    }
}

pub struct Data {
    pub data: DataTypes,
}

impl Data {
    pub const NULL: crate::Data = crate::Data {
        data: crate::DataTypes::NULL,
    };

    pub fn null() -> Self {
        crate::Data {
            data: crate::DataTypes::NULL,
        }
    }

    pub fn from<T>(data: T) -> Self
    where
        T: Into<Data>,
    {
        data.into()
    }

    pub fn typename(&self) -> std::string::String {
        self.data.typename()
    }

    pub fn is_null(&self) -> bool {
        match &self.data {
            DataTypes::NULL => true,
            _ => false,
        }
    }
}

impl From<bool> for Data {
    fn from(value: bool) -> Self {
        crate::Data {
            data: crate::DataTypes::BOOL(value),
        }
    }
}

impl From<u8> for Data {
    fn from(value: u8) -> Self {
        crate::Data {
            data: crate::DataTypes::UINT8(value),
        }
    }
}

impl From<u16> for Data {
    fn from(value: u16) -> Self {
        crate::Data {
            data: crate::DataTypes::UINT16(value),
        }
    }
}

impl From<u32> for Data {
    fn from(value: u32) -> Self {
        crate::Data {
            data: crate::DataTypes::UINT32(value),
        }
    }
}

impl From<u64> for Data {
    fn from(value: u64) -> Self {
        crate::Data {
            data: crate::DataTypes::UINT64(value),
        }
    }
}

impl From<i8> for Data {
    fn from(value: i8) -> Self {
        crate::Data {
            data: crate::DataTypes::INT8(value),
        }
    }
}

impl From<i16> for Data {
    fn from(value: i16) -> Self {
        crate::Data {
            data: crate::DataTypes::INT16(value),
        }
    }
}

impl From<i32> for Data {
    fn from(value: i32) -> Self {
        crate::Data {
            data: crate::DataTypes::INT32(value),
        }
    }
}

impl From<i64> for Data {
    fn from(value: i64) -> Self {
        crate::Data {
            data: crate::DataTypes::INT64(value),
        }
    }
}

impl From<f64> for Data {
    fn from(value: f64) -> Self {
        crate::Data {
            data: crate::DataTypes::FLOAT64(value),
        }
    }
}

impl From<char> for Data {
    fn from(value: char) -> Self {
        crate::Data {
            data: crate::DataTypes::CHAR(value),
        }
    }
}

impl From<&str> for Data {
    fn from(value: &str) -> Self {
        crate::Data {
            data: crate::DataTypes::STRING(crate::String::from(value)),
        }
    }
}

impl From<crate::String> for Data {
    fn from(value: crate::String) -> Self {
        crate::Data {
            data: crate::DataTypes::STRING(value),
        }
    }
}

impl From<Vec<Data>> for Data {
    fn from(value: Vec<Data>) -> Self {
        crate::Data {
            data: crate::DataTypes::LIST(crate::List::from(value)),
        }
    }
}

impl From<crate::List> for Data {
    fn from(value: crate::List) -> Self {
        crate::Data {
            data: crate::DataTypes::LIST(value),
        }
    }
}

impl From<crate::Map> for Data {
    fn from(value: crate::Map) -> Self {
        crate::Data {
            data: crate::DataTypes::MAP(value),
        }
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        match &self.data {
            crate::DataTypes::BOOL(value) => todata!(*value),
            crate::DataTypes::UINT8(value) => todata!(*value),
            crate::DataTypes::UINT16(value) => todata!(*value),
            crate::DataTypes::UINT32(value) => todata!(*value),
            crate::DataTypes::UINT64(value) => todata!(*value),
            crate::DataTypes::INT8(value) => todata!(*value),
            crate::DataTypes::INT16(value) => todata!(*value),
            crate::DataTypes::INT32(value) => todata!(*value),
            crate::DataTypes::INT64(value) => todata!(*value),
            crate::DataTypes::FLOAT64(value) => todata!(*value),
            crate::DataTypes::CHAR(value) => todata!(*value),
            crate::DataTypes::STRING(value) => todata!(value.clone()),
            crate::DataTypes::LIST(value) => todata!(value.clone()),
            crate::DataTypes::MAP(value) => todata!(value.clone()),
            crate::DataTypes::NULL => Data::NULL,
        }
    }
}

impl traits::ToStr for Data {
    fn tostr(&self) -> std::string::String {
        match &self.data {
            crate::DataTypes::BOOL(value) => value.to_string(),
            crate::DataTypes::UINT8(value) => value.to_string(),
            crate::DataTypes::UINT16(value) => value.to_string(),
            crate::DataTypes::UINT32(value) => value.to_string(),
            crate::DataTypes::UINT64(value) => value.to_string(),
            crate::DataTypes::INT8(value) => value.to_string(),
            crate::DataTypes::INT16(value) => value.to_string(),
            crate::DataTypes::INT32(value) => value.to_string(),
            crate::DataTypes::INT64(value) => value.to_string(),
            crate::DataTypes::FLOAT64(value) => value.to_string(),
            crate::DataTypes::CHAR(value) => value.to_string(),
            crate::DataTypes::STRING(value) => value.tostr(),
            crate::DataTypes::LIST(value) => value.tostr(),
            crate::DataTypes::MAP(value) => value.tostr(),
            crate::DataTypes::NULL => std::string::String::from("NULL"),
        }
    }
}

impl traits::RefC for Data {
    fn incrc(&mut self) {
        match &mut self.data {
            crate::DataTypes::STRING(value) => value.incrc(),
            crate::DataTypes::LIST(value) => value.incrc(),
            crate::DataTypes::MAP(value) => value.incrc(),
            _ => (),
        }
    }

    fn decrc(&mut self) {
        match &mut self.data {
            crate::DataTypes::STRING(value) => value.decrc(),
            crate::DataTypes::LIST(value) => value.decrc(),
            crate::DataTypes::MAP(value) => value.decrc(),
            _ => (),
        }
    }

    fn getrc(&self) -> i64 {
        match &self.data {
            crate::DataTypes::STRING(value) => value.getrc(),
            crate::DataTypes::LIST(value) => value.getrc(),
            crate::DataTypes::MAP(value) => value.getrc(),
            _ => 0,
        }
    }
}

impl traits::RefCopy for Data {
    fn refcopy(&mut self) -> Self {
        match &mut self.data {
            crate::DataTypes::STRING(value) => todata!(value.refcopy()),
            crate::DataTypes::LIST(value) => todata!(value.refcopy()),
            crate::DataTypes::MAP(value) => todata!(value.refcopy()),
            _ => self.clone(),
        }
    }

    fn refdrop(mut self) {
        self.decrc();
        if self.getrc() > 0 {
            return;
        }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
    }
}
