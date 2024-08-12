pub trait GPSerializer {
    // Serialize the data to a byte array
    fn gp_serialize(&self) -> Vec<u8>;
}

pub trait GPSerializerTrivial {
    // Serialize the data to a byte array
    fn gp_serialize_trivial(&self, l: u8) -> Vec<u8>;
}

pub trait GPSerializerWithLength: GPSerializer {
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
        let x = *self;
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

impl GPSerializerTrivial for u32 {
    fn gp_serialize_trivial(&self, l: u8) -> Vec<u8> {
        let mut bytes = Vec::new();
        for i in 0..l {
            let byte = ((self >> (8 * i)) & 0xFF) as u8;
            bytes.push(byte);
        }
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_slice_serialization() {
        let data: &[u8] = &[1, 2, 3, 4];
        assert_eq!(data.gp_serialize(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_byte_slice_serialization_with_length() {
        let data: &[u8] = &[1, 2, 3, 4];
        let mut expected = vec![4];
        expected.extend_from_slice(&[1, 2, 3, 4]);
        assert_eq!(data.gp_serialize_with_length(), expected);
    }

    #[test]
    fn test_vec_serialization() {
        let data: Vec<u8> = vec![1, 2, 3, 4];
        assert_eq!(data.gp_serialize(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_vec_serialization_with_length() {
        let data: Vec<u8> = vec![1, 2, 3, 4];
        let mut expected = vec![4];
        expected.extend_from_slice(&[1, 2, 3, 4]);
        assert_eq!(data.gp_serialize_with_length(), expected);
    }

    #[test]
    fn test_u64_serialization() {
        let value: u64 = 0;
        assert_eq!(value.gp_serialize(), vec![0]);

        let value: u64 = 1;
        assert_eq!(value.gp_serialize(), vec![1]);

        let value: u64 = 255;
        assert_eq!(value.gp_serialize(), vec![128, 255]);

        let value: u64 = 256;
        assert_eq!(value.gp_serialize(), vec![129, 0]);
    }

    #[test]
    fn test_u32_serialization() {
        let value: u32 = 0;
        assert_eq!(value.gp_serialize_trivial(1), vec![0]);

        let value: u32 = 1;
        assert_eq!(value.gp_serialize_trivial(1), vec![1]);

        let value: u32 = 255;
        assert_eq!(value.gp_serialize_trivial(2), vec![255, 0]);

        let value: u32 = 256;
        assert_eq!(value.gp_serialize_trivial(2), vec![0, 1]);
    }
}
