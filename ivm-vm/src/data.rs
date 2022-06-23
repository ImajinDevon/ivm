use crate::Pointer;

#[derive(Clone)]
pub enum OwnedData {
    Bytes(Vec<u8>),
    Object(Vec<Pointer>),
}
