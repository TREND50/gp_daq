use std::convert::AsRef;
use bincode::{serialize, deserialize};


#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct BasicFileHeader{
    pub length:u32,
    pub runnr:u32,
    pub run_mod:u32,
    pub seral:u32,
    pub first_event:u32,
    pub first_event_sec:u32,
    pub last_event:u32,
    pub last_event_sec:u32
}


#[derive(Eq, PartialEq, Clone)]
pub struct FileHeader{
    pub basic_header:BasicFileHeader,
    pub additional_header:Vec<u32>
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct EventHeader{
    pub header_length:u32,
    pub runnr:u32,
    pub eventnr:u32,
    pub t3eventnr:u32,
    pub first_ls:u32,
    pub event_sec:u32,
    pub event_nsec:u32,
    pub event_type:u16,
    pub event_vers:u16,
    pub ad1:u32,
    pub ad2:u32,
    pub ls_cnt:u32,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[repr(C)]
pub struct LocalStationHeader{
    pub length:u16,
    pub event_nr:u16,
    pub ls_id:u16,
    pub header_length:u16,
    pub GPSseconds:u32,
    pub GPSnanoseconds:u32,
    pub trigger_flag:u16,
    pub trigger_pos:u16,
    pub sampling_freq:u16,
    pub channel_mask:u16,
    pub ADC_resolution:u16,
    pub trace_length:u16,
    pub version:u16,
}


#[derive(Eq, PartialEq, Clone)]
pub struct LocalStation{
    header:LocalStationHeader,
    header_data:Vec<u16>,
    adc_buffer:Vec<u16>
}

#[derive(Eq, PartialEq, Clone)]
pub struct Event{
    header:EventHeader,
    local_station_list:Vec<LocalStation>,
}

#[derive(Eq, PartialEq, Clone)]
pub struct EventFile{
    header:FileHeader,
    event_list:Vec<Event>,
}

