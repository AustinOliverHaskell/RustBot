pub struct tmc2208 {
    fd: String
}

impl tmc2208 {
    pub fn calculate_crc(bytes: &Vec<u8>) -> u8 {
        let mut crc: u8 = 0;
        for byte in bytes {
            let mut val = byte.clone();
    
            for _ in 0..8 {
                if (crc >> 7) ^ (val & 0x01) != 0 {
                    crc = (crc << 1) ^ 0x07;
                } else {
                    crc = crc << 1;
                }
            
                val = val >> 1;
            }
        }
    
        crc
    }
    
    pub fn make_packet(register: u8, payload: u32) -> Vec<u8> {
        let header: u8 = 175; // 10101111
        let slave_address: u8 = 0;
        let register_address = register << 1;
    
        let mut packet: Vec<u8> = Vec::new();
        packet.push(header);
        packet.push(slave_address);
        packet.push(register_address);
    
        let a: u8 = (payload >> 24) as u8;
        let b: u8 = (payload >> 16) as u8;
        let c: u8 = (payload >> 8 ) as u8;
        let d: u8 = payload as u8;
    
        packet.push(a);
        packet.push(b);
        packet.push(c);
        packet.push(d);
    
        let crc = tmc2208::calculate_crc(&packet);
    
        packet.push(crc);
    
        packet
    }
}


#[test]
fn crc_calc_calculates() {

    let test_vec: Vec<u8> = vec![175];
    let actual = tmc2208::calculate_crc(&test_vec);

    // crc = 0
    // 175 = 10101111
    // 1 -> crc =       0 ^ 7 = 07
    // 1 -> crc = 7  << 1 ^ 7 = 09
    // 1 -> crc = 9  << 1 ^ 7 = 15
    // 1 -> crc = 15 << 1 ^ 7 = 2D
    // 0 -> crc = 2D << 1     = 5A
    // 1 -> crc = 5A << 1 ^ 7 = B3
    // 0 -> crc = B3 << 1 ^ 7 = 61
    // 1 -> crc = 61 << 1 ^ 7 = C5
    
    assert_eq!(0xC5, actual)
}

#[test]
fn make_packet_creates() {

    let mut expected: Vec<u8> = vec![175, 0, 0xFE, 0xFA, 0xFB, 0xFC, 0xFD];
    expected.push(tmc2208::calculate_crc(&expected));

    let actual = tmc2208::make_packet(0xFF, 0xFAFBFCFD);

    assert_eq!(expected, actual)
}