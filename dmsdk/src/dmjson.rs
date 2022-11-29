use crate::ffi::dmJson;
use std::{
    convert::TryInto,
    ffi::{CStr, CString},
    ptr,
};

#[derive(Debug)]
pub enum Error {
    SyntaxError,
    Incomplete,
    Unknown,
}

#[derive(Debug)]
pub enum Result<T> {
    Ok(T),
    Err(Error),
}

#[derive(Debug)]
pub enum Kind {
    Primitive,
    Object,
    Array,
    String,
}

impl From<u32> for Kind {
    fn from(n: u32) -> Self {
        match n {
            0 => Kind::Primitive,
            1 => Kind::Object,
            2 => Kind::Array,
            3 => Kind::String,
            _ => Kind::Primitive,
        }
    }
}

impl From<i32> for Kind {
    fn from(n: i32) -> Self {
        Kind::from(n as u32)
    }
}

#[derive(Debug)]
pub struct Node {
    pub kind: Kind,
    pub start: i32,
    pub end: i32,
    pub size: i32,
    pub sibling: Option<i32>,
}

impl From<&dmJson::Node> for Node {
    fn from(node: &dmJson::Node) -> Self {
        let sibling = if node.m_Sibling == -1 {
            None
        } else {
            Some(node.m_Sibling)
        };

        Node {
            kind: Kind::from(node.m_Type),
            start: node.m_Start,
            end: node.m_End,
            size: node.m_Size,
            sibling,
        }
    }
}

#[derive(Debug)]
pub struct Document {
    pub nodes: Vec<Node>,
    pub json: String,
}

pub fn parse(json: &str) -> Result<Document> {
    let json = CString::new(json).unwrap();

    let mut c_document = dmJson::Document {
        m_Nodes: ptr::null_mut(),
        m_NodeCount: 0,
        m_Json: ptr::null_mut(),
        m_UserData: ptr::null_mut(),
    };

    unsafe {
        let result = dmJson::Parse1(json.as_ptr(), &mut c_document);

        let c_nodes = ptr::slice_from_raw_parts(
            c_document.m_Nodes,
            c_document.m_NodeCount.try_into().unwrap(),
        );

        let nodes = c_nodes.as_ref().unwrap().iter().map(Node::from).collect();
        let document = Document {
            nodes,
            json: CStr::from_ptr(c_document.m_Json)
                .to_str()
                .unwrap()
                .to_owned(),
        };

        dmJson::Free(&mut c_document);

        match result {
            0 => Result::Ok(document),
            -1 => Result::Err(Error::SyntaxError),
            -2 => Result::Err(Error::Incomplete),
            _ => Result::Err(Error::Unknown),
        }
    }
}
