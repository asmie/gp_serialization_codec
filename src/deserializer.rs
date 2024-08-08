pub trait GPDeserializer {
    // Serialize the data to a byte array
    fn gp_deserialize(&mut self, serialized: &[u8]);
}

pub trait GPDeserializerWithLength: GPDeserializer {
    fn gp_deserialize_with_length(&mut self, serialized: &[u8]);
}

// Implement the trait for vector of bytes (octet-sequences)
impl GPDeserializer for Vec<u8> {
    fn gp_deserialize(&mut self, serialized: &[u8]) {
        *self = serialized.to_vec();
    }
}

impl GPDeserializerWithLength for Vec<u8> {
    fn gp_deserialize_with_length(&mut self, serialized: &[u8]) {
        let mut len = 0u64;
        len.gp_deserialize(&serialized);
        let i = serialized.len() - len as usize;
        *self = serialized[i..].to_vec();
    }
}

impl GPDeserializer for u64 {
    fn gp_deserialize(&mut self, serialized: &[u8]) {
        *self = 0;
        if serialized.len() == 0 {
            return;
        }

        // The case with 9 bytes
        if serialized.len() > 8 {
            assert_eq!(serialized[0], 255);
            // Deserialize the next 8 bytes as u64 little endian
            for i in 0..8 {
                *self |= (serialized[i + 1] as u64) << (8 * i);
            }
            return;
        }

        let mut prefix = serialized[0];
        let l = prefix.leading_ones() as usize;
        // Deserialize the next l bytes as u64 little endian
        for i in 0..l {
            *self |= (serialized[i + 1] as u64) << (8 * i);
        }
        // Clear l oldest bits in prefix
        prefix &= 255u8 >> l;
        *self |= (prefix as u64) << (8 * l);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_deserialization() {
        let serialized: &[u8] = &[1, 2, 3, 4];
        let mut data: Vec<u8> = Vec::new();
        data.gp_deserialize(serialized);
        assert_eq!(data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_vec_deserialization_with_length() {
        let serialized: &[u8] = &[4, 1, 2, 3, 4];
        let mut data: Vec<u8> = Vec::new();
        data.gp_deserialize_with_length(serialized);
        assert_eq!(data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_u64_deserialization() {
        let serialized: &[u8] = &[0];
        let mut value: u64 = 0;
        value.gp_deserialize(serialized);
        assert_eq!(value, 0);

        let serialized: &[u8] = &[1];
        value.gp_deserialize(serialized);
        assert_eq!(value, 1);

        let serialized: &[u8] = &[128, 255];
        value.gp_deserialize(serialized);
        assert_eq!(value, 255);

        let serialized: &[u8] = &[129, 0];
        value.gp_deserialize(serialized);
        assert_eq!(value, 256);
    }
}
