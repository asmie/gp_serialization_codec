trait GPSerializer {
    // Serialize the data to a byte array
    fn gp_serialize(&self) -> Vec<u8>;

}

// Implement the trait for byte slices (octet-sequences)
impl GPSerializer for &[u8] {
    fn gp_serialize(&self) -> Vec<u8> {
        self.to_vec()
    }
}

// Implement the trait for vector of bytes (octet-sequences)
impl GPSerializer for Vec<u8> {
    fn gp_serialize(&self) -> Vec<u8> {
        self.clone()
    }
}

// Implement the trait for u32 (using general encoding)
impl GPSerializer for u32 {
    fn gp_serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl GPSerializer for u64  {
    fn gp_serialize(&self) -> Vec<u8> {

    }
}