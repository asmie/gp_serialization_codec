mod deserializer;
mod serializer;

use deserializer::GPDeserializer;
use deserializer::GPDeserializerWithLength;
use serializer::GPSerializer;
use serializer::GPSerializerWithLength;

fn main() {
    let values = [
        0,
        1,
        127,
        128,
        255,
        256,
        1023,
        1024,
        16383,
        65535,
        65536,
        2u64.pow(63),
    ];

    for &value in values.iter() {
        let serialized = value.gp_serialize();
        println!("{} -> {:?}", value, serialized);

        let mut deserialized = 0u64;
        deserialized.gp_deserialize(&serialized);
        println!("{:?} -> {}", serialized, deserialized);
    }

    let data = vec![1, 2, 3, 4];
    let serialized = data.gp_serialize();
    println!("{:?} -> {:?}", data, serialized);

    let mut deserialized = Vec::new();
    deserialized.gp_deserialize(&serialized);
    println!("{:?} -> {:?}", serialized, deserialized);

    let data = vec![255, 199, 210, 40];
    let serialized = data.gp_serialize_with_length();
    println!("{:?} -> {:?}", data, serialized);

    let mut deserialized = Vec::new();
    deserialized.gp_deserialize_with_length(&serialized);
    println!("{:?} -> {:?}", serialized, deserialized);
}
