use std::panic;

use substring::Substring;
use to_binary::BinaryString;

#[derive(Debug)]
struct Packet {
    version: usize,
    type_id: usize,
    value: usize,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn total_version(&self) -> usize {
        self.version + self.sub_packets.iter().map(|p| p.total_version()).sum::<usize>()
    }

    fn total_value(&self) -> usize {
        match self.type_id {
            0 => self.sub_packets.iter().map(|p| p.total_value()).sum::<usize>(),
            1 => self.sub_packets.iter().map(|p| p.total_value()).product::<usize>(),
            2 => self.sub_packets.iter().min_by(|x, y| x.total_value().cmp(&y.total_value())).unwrap().total_value(),
            3 => self.sub_packets.iter().max_by(|x, y| x.total_value().cmp(&y.total_value())).unwrap().total_value(),
            4 => self.value,
            5 => (self.sub_packets[0].total_value() > self.sub_packets[1].total_value()) as usize,
            6 => (self.sub_packets[0].total_value() < self.sub_packets[1].total_value()) as usize,
            7 => (self.sub_packets[0].total_value() == self.sub_packets[1].total_value()) as usize,
            _ => panic!("Invalid type_id: {}", self.type_id),
        }
    }
}

fn read_field_int(binary_string: &mut String, num_bits: usize) -> usize
{
    let field = usize::from_str_radix(binary_string.substring(0, num_bits), 2).unwrap();
    binary_string.replace_range(0..num_bits, "");
    field
}

fn read_field_hex(binary_string: &mut String, num_bits: usize) -> String
{
    let field = binary_string.substring(0, num_bits).into();
    binary_string.replace_range(0..num_bits, "");
    field
}

fn parse_packets(binary_data: &mut String) -> (Packet, usize)
{
    let initial_len = binary_data.len();

    let version = read_field_int(binary_data, 3);
    let type_id = read_field_int(binary_data, 3);

    let mut value: usize = 0;
    let mut sub_packets = Vec::new();

    if type_id == 4 {
        // Literal value
        let mut value_str = String::new();
        while read_field_int(binary_data, 1) == 1 {
            value_str += &read_field_hex(binary_data, 4);
        }
        // Read last part of the value aswell
        value_str += &read_field_hex(binary_data, 4);
        value = usize::from_str_radix(value_str.as_str(), 2).unwrap();
    }
    else {
        // Operator packet
        if read_field_int(binary_data, 1) == 1 {
            let num_sub_packets = read_field_int(binary_data, 11);

            // Read num_sub_packets
            for _i in 0..num_sub_packets {
                let (packet, _) = parse_packets(binary_data);
                sub_packets.push(packet);
            }
        }
        else {
            let mut len = read_field_int(binary_data, 15);

            // Read packets until we reach len bits
            while len != 0 {
                let (packet, packet_size) = parse_packets(binary_data);
                sub_packets.push(packet);
                len -= packet_size;
            }
        }
    }

    (
        Packet {
            version,
            type_id,
            value,
            sub_packets,
        },
        (initial_len - binary_data.len())
    )
}

fn main()
{
    let hex_string = utils::read_input_file();
    for line in hex_string {
        let mut binary_string = BinaryString::from_hex(line).unwrap().0;
        let (top_packet, _) = parse_packets(&mut binary_string);


        println!("Part 1: {:?}", top_packet.total_version());
        println!("Part 2: {:?}", top_packet.total_value());
    }
}
