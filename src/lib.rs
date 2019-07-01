#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl std::fmt::Debug for GUID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let data4: Vec<String> = self.Data4.iter().map(|s| format!("{:02x}", s)).collect();
        let data4 = data4.concat();
        write!(f, "{:08x}-{:04x}-{:04x}-{}", self.Data1 ,self.Data2, self.Data3, data4)
    }
}

macro_rules! make_guid {
    ($d1:expr, $d2:expr, $d3:expr, $d4:expr) => {
        GUID {
            Data1: $d1,
            Data2: $d2,
            Data3: $d3,
            Data4: $d4,
        }
    };
}

pub mod guids;