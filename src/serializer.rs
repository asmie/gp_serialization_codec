use std::ops::{Shr, BitAnd};

pub trait GPSerializer {
    // Serialize the data to a byte array
    fn gp_serialize(&self) -> Vec<u8>;
}

pub trait GPSerializerWithLength : GPSerializer {
    fn gp_serialize_with_length(&self) -> Vec<u8>;
}

// Implement the trait for byte slices (octet-sequences)
impl GPSerializer for &[u8] {
    fn gp_serialize(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl GPSerializerWithLength for &[u8] {
    fn gp_serialize_with_length(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let len = self.len() as u64;
        bytes.extend_from_slice(&len.gp_serialize());
        bytes.extend_from_slice(self);
        bytes
    }
}

// Implement the trait for vector of bytes (octet-sequences)
impl GPSerializer for Vec<u8> {
    fn gp_serialize(&self) -> Vec<u8> {
        self.clone()
    }
}

impl GPSerializerWithLength for Vec<u8> {
    fn gp_serialize_with_length(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let len = self.len() as u64;
        bytes.extend_from_slice(&len.gp_serialize());
        bytes.extend_from_slice(self);
        bytes
    }
}

impl GPSerializer for u64 {
    fn gp_serialize(&self) -> Vec<u8> {
        if *self == 0 {
            return vec![0];
        }

        let mut l = 0u8;
        let mut x = *self;
        let mut found = false;

        for i in 0..8 {
            if x >= (1 << (7 * i)) && x < (1 << (7 * (i + 1))) {
                found = true;
                break;
            }
            l += 1;
        }

        let mut bytes = Vec::new();

        if found {
            let prefix = (256 - (1 << (8 - l))) as u8 + ((x >> (8 * l)) & 0xFF) as u8;
            bytes.push(prefix);
        } else {
            bytes.push(255);
            l = 8;
        }

        // Trivial serialization of integer values in little-endian order
        for i in 0..l {
            let byte = ((x >> (8 * i)) & 0xFF) as u8;
            bytes.push(byte);
        }

        bytes
    }
}
