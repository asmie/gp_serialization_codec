
mod serializer;

use serializer::GPSerializer;

fn main() {
    let values = [0, 1, 127, 128, 255, 256, 1023, 1024, 65535, 65536, 2u64.pow(63)];

    for &value in values.iter() {
        let serialized = value.gp_serialize();
        println!("{} -> {:?}", value, serialized);
    }
}