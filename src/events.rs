use std::fmt::{Display, Formatter};
use std::fs;
use crate::events;

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub enum Operations {
    #[default]
    Delete = 0x01,
    Disable = 0x02,
    Enable = 0x03,
    IssueCertificate = 0x04,
}

impl TryFrom<u8> for Operations {
    type Error = String;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Operations::Delete),
            0x02 => Ok(Operations::Disable),
            0x03 => Ok(Operations::Enable),
            0x04 => Ok(Operations::IssueCertificate),
            _ => Err("Invalid Operations Type".to_string()),
        }
    }
}

impl From<Operations> for u8 {
    fn from(value: Operations) -> Self {
        match value {
            Operations::Delete => 0x01,
            Operations::Disable => 0x02,
            Operations::Enable => 0x03,
            Operations::IssueCertificate => 0x04,
        }
    }
}

impl TryFrom<&str> for Operations {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("delete") {
            Ok(Operations::Delete)
        } else if value.eq_ignore_ascii_case("disable") {
            Ok(Operations::Disable)
        } else if value.eq_ignore_ascii_case("enable") {
            Ok(Operations::Enable)
        } else if value.eq_ignore_ascii_case("issue") {
            Ok(Operations::IssueCertificate)
        } else {
            Err("Invalid Operations Type".to_string())
        }
    }
}

impl Display for Operations {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operations::Delete => write!(f, "Delete"),
            Operations::Disable => write!(f, "Disable"),
            Operations::Enable => write!(f, "Enable"),
            Operations::IssueCertificate => write!(f, "Issue CarKey"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub enum Objects {
    #[default]
    Owner = 0x01,
    Friend = 0x02,
    Middle = 0x03,
}

impl TryFrom<u8> for Objects {
    type Error = String;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Objects::Owner),
            0x02 => Ok(Objects::Friend),
            0x03 => Ok(Objects::Middle),
            _ => Err("Invalid Objects Type".to_string()),
        }
    }
}

impl From<Objects> for u8 {
    fn from(value: Objects) -> Self {
        match value {
            Objects::Owner => 0x01,
            Objects::Friend => 0x02,
            Objects::Middle => 0x03,
        }
    }
}

impl TryFrom<&str> for Objects {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("owner") {
            Ok(Objects::Owner)
        } else if value.eq_ignore_ascii_case("friend") {
            Ok(Objects::Friend)
        } else if value.eq_ignore_ascii_case("middle") {
            Ok(Objects::Middle)
        } else {
            Err("Invalid Objects Type".to_string())
        }
    }
}

impl From<Objects> for &'static str {
    fn from(value: Objects) -> Self {
        match value {
            Objects::Owner => "owner",
            Objects::Friend => "friend",
            Objects::Middle => "middle",
        }
    }
}

impl Display for Objects {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Objects::Owner => write!(f, "Owner"),
            Objects::Friend => write!(f, "Friend"),
            Objects::Middle => write!(f, "Middle"),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct EventMessage {
    operation: Operations,
    object: Objects,
    data: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl EventMessage {
    pub fn new(operation: Operations, object: Objects, data: Option<Vec<u8>>) -> Self {
        EventMessage {
            operation,
            object,
            data,
        }
    }
    pub fn get_operation(&self) -> Operations {
        self.operation
    }
    pub fn set_operation(&mut self, operation: Operations) {
        self.operation = operation;
    }
    pub fn get_object(&self) -> Objects {
        self.object
    }
    pub fn set_object(&mut self, object: Objects) {
        self.object = object;
    }
    pub fn get_data(&self) -> Option<&[u8]> {
        if let Some(ref data) = self.data {
            Some(data)
        } else {
            None
        }
    }
    pub fn set_data(&mut self, data: Option<Vec<u8>>) {
        self.data = data;
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = vec![u8::from(self.get_operation()), u8::from(self.get_object())];
        if let Some(data) = self.get_data() {
            buffer.append(&mut (data.len() as u16).to_be_bytes().to_vec());
            buffer.append(&mut data.to_vec());
        }
        buffer
    }
}

impl TryFrom<&str> for EventMessage {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut cmd_iter = value.split_whitespace();
        if cmd_iter.clone().count() < 0x02 {
            return Err("command count error".to_string());
        }
        let operation = match events::Operations::try_from(cmd_iter.next().unwrap()) {
            Ok(op) => op,
            Err(_) => {
                return Err("Operation is not Valid!!!".to_string());
            }
        };
        let object = match events::Objects::try_from(cmd_iter.next().unwrap()) {
            Ok(obj) => obj,
            Err(_) => {
                return Err("Object is not Valid!!!".to_string());
            }
        };
        let data = if let Some(path) = cmd_iter.next() {
            Some(fs::read_to_string(path).unwrap().as_bytes().to_vec())
        } else {
            None
        };
        Ok(EventMessage::new(
            operation,
            object,
            data,
        ))
    }
}

impl Display for EventMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.get_operation(), self.get_object())
    }
}
