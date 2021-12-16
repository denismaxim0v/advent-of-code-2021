#[derive(Debug)]
enum Packet {
    Literal(Header, i64),
    Operator(Header, Vec<Packet>),
}

impl Packet {
    fn get_ver(&self) -> i64 {
        match self {
            Packet::Literal(header, _) => header.version,
            Packet::Operator(header, packets) => {
                let child_ver: i64 = packets.iter().map(|p| p.get_ver()).sum();
                header.version + child_ver
            }
        }
    }

    fn calculate(&self) -> i64 {
        match self {
            Packet::Literal(_, v) => *v,
            Packet::Operator(header, packets) => {
                let packets: Vec<i64> = packets.iter().map(|p| p.calculate()).collect();
                match header.id {
                    0 => packets.iter().sum(),
                    1 => packets.iter().product(),
                    2 => *packets.iter().min().unwrap(),
                    3 => *packets.iter().max().unwrap(),
                    5 => (packets[0] > packets[1]) as i64,
                    6 => (packets[0] < packets[1]) as i64,
                    7 => (packets[0] == packets[1]) as i64,
                    _ => panic!(),
                }
            }
        }
    }

    fn from_bits(buffer: &[u8], start: usize) -> (Packet, usize) {
        let header = Header::from_bits(buffer, start);

        match header.id {
            4 => Packet::literal_from_bits(header, buffer, start + 6),
            _ => Packet::operator_from_bits(header, buffer, start + 6),
        }
    }

    fn literal_from_bits(header: Header, buffer: &[u8], start: usize) -> (Packet, usize) {
        let mut value = 0;

        let mut i = start;
        let mut done = false;
        while !done {
            if buffer[i] == 0 {
                done = true;
            }

            i += 1;

            value <<= 4;
            value += bits_to_i64(&buffer[i..(i + 4)]);

            i += 4;
        }

        (Packet::Literal(header, value), i)
    }

    fn operator_from_bits(header: Header, buffer: &[u8], start: usize) -> (Packet, usize) {
        let mut i = start;

        let length_type_id = buffer[i];
        i += 1;

        let length = match length_type_id {
            0 => {
                let len = bits_to_i64(&buffer[i..(i + 15)]);
                i += 15;

                len
            }
            1 => {
                let len = bits_to_i64(&buffer[i..(i + 11)]);
                i += 11;

                len
            }
            _ => panic!(),
        };

        let mut j = i;
        let mut sub_i = 0;
        let mut sub_packets = vec![];
        loop {
            sub_i += 1;

            if length_type_id == 0 && j >= i + length as usize {
                break;
            }
            if length_type_id == 1 && sub_i > length {
                break;
            }

            let results = Packet::from_bits(buffer, j);

            j = results.1;
            sub_packets.push(results.0);
        }

        (Packet::Operator(header, sub_packets), j)
    }
}

#[derive(Debug)]
struct Header {
    version: i64,
    id: i64,
}

impl Header {
    fn from_bits(buffer: &[u8], s: usize) -> Header {
        let version = bits_to_i64(&buffer[s..(s + 3)]);
        let id = bits_to_i64(&buffer[(s + 3)..(s + 6)]);

        Header { version, id }
    }
}

fn parse_input(inp: String) -> Vec<u8> {
    let line = inp.lines().last().unwrap();
    let hex_nums: Vec<i64> = line
        .chars()
        .map(|h| i64::from_str_radix(&(h.to_string()), 16).unwrap())
        .collect();

    let hex_nums: Vec<u8> = hex_nums
        .iter()
        .map(|d| i64_to_bits(*d))
        .flatten()
        .collect::<Vec<u8>>();

    hex_nums
}

fn i64_to_bits(value: i64) -> Vec<u8> {
    let mut bits = vec![];

    for i in (0..4).rev() {
        let mask = 1 << i;
        if value & mask != 0 {
            bits.push(1);
        } else {
            bits.push(0);
        }
    }

    bits
}

fn bits_to_i64(bits: &[u8]) -> i64 {
    let mut value: i64 = 0;

    for b in bits.iter() {
        value <<= 1;
        value += *b as i64;
    }

    value
}

pub fn part1(inp: String) {
    let input = parse_input(inp);
    let packet = Packet::from_bits(&input, 0);

    println!("{}", packet.0.get_ver());
}
pub fn part2(inp: String) {
    let input = parse_input(inp);
    let packet = Packet::from_bits(&input, 0);

    println!("{}", packet.0.calculate());
}
