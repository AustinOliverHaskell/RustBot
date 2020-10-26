mod tmc {
    pub struct Tmc2208 {
        fd: String
    }

    impl Tmc2208 {
        pub fn make_tmc2208(file_descriptor: String) /*-> Self*/ {
            // Todo: Insert code that opens the file descriptor here and returns a new 
            //  tmc struct.                                           - Austin Haskell

        }

        pub fn send_packet(self: &Self, register: u8, payload: u32) {
            // Todo: Add in the send code - Austin Haskell
        }

        

        pub fn make_conf_packet() /*-> Vec<Vec<u8>>*/ {
            // Note: Multiple configuration packets have to be sent, 
            //  once 
        }
    }
}
