use macro_file_proxy::macro_file_proxy_item;


macro_rules! build_mp1_100_symbol {
    ($($addr:tt $sym_name:tt;)*) => {
        #[macro_export]
        macro_rules! mp1_100_symbol {
            $(
                ($sym_name) => { Some($addr) };
            )*
            ($_s:tt) => { None };
        }
    }
}
macro_file_proxy_item!("src/dol_symbol_table/1.00.txt", build_mp1_100_symbol, ;);

macro_rules! build_mp1_101_symbol {
    ($($addr:tt $sym_name:tt;)*) => {
        #[macro_export]
        macro_rules! mp1_101_symbol {
            $(
                ($sym_name) => { Some($addr) };
            )*
            ($_s:tt) => { None };
        }
    }
}
macro_file_proxy_item!("src/dol_symbol_table/1.01.txt", build_mp1_101_symbol, ;);

macro_rules! build_mp1_102_symbol {
    ($($addr:tt $sym_name:tt;)*) => {
        #[macro_export]
        macro_rules! mp1_102_symbol {
            $(
                ($sym_name) => { Some($addr) };
            )*
            ($_s:tt) => { None };
        }
    }
}
macro_file_proxy_item!("src/dol_symbol_table/1.02.txt", build_mp1_102_symbol, ;);

macro_rules! build_mp1_pal_symbol {
    ($($addr:tt $sym_name:tt;)*) => {
        #[macro_export]
        macro_rules! mp1_pal_symbol {
            $(
                ($sym_name) => { Some($addr) };
            )*
            ($_s:tt) => { None };
        }
    }
}
macro_file_proxy_item!("src/dol_symbol_table/pal.txt", build_mp1_pal_symbol, ;);

macro_rules! build_mp1_trilogy_ntsc_symbol {
    ($($addr:tt $sym_name:tt;)*) => {
        #[macro_export]
        macro_rules! mp1_trilogy_ntsc_symbol {
            $(
                ($sym_name) => { Some($addr) };
            )*
            ($_s:tt) => { None };
        }
    }
}
macro_file_proxy_item!("src/dol_symbol_table/trilogy_ntsc.txt", build_mp1_trilogy_ntsc_symbol, ;);

pub struct Mp1Symbol
{
    pub addr_0_00: Option<u32>,
    pub addr_0_01: Option<u32>,
    pub addr_0_02: Option<u32>,
    pub addr_pal: Option<u32>,
    pub addr_trilogy_ntsc: Option<u32>,
}

#[macro_export]
macro_rules! mp1_symbol {
    ($syn_name:tt) => {
        $crate::Mp1Symbol {
            addr_0_00: $crate::mp1_100_symbol!($syn_name),
            addr_0_01: $crate::mp1_101_symbol!($syn_name),
            addr_0_02: $crate::mp1_102_symbol!($syn_name),
            addr_pal: $crate::mp1_pal_symbol!($syn_name),
            addr_trilogy_ntsc: $crate::mp1_trilogy_ntsc_symbol!($syn_name),
        }
    }
}
