use std;
use std::io::{Read, Write};

use super::super::msg_def::msgcont::Data;
use std::fmt::{self, Display, Formatter};

trait FromByteStream: Default + Sized {
    fn read_from<R: Read>(reader: &mut R) -> Option<Self> {
        let size = std::mem::size_of::<Self>();
        let mut result = Self::default();
        {
            let raw = unsafe {
                std::slice::from_raw_parts_mut((&mut result) as *mut Self as *mut u8, size)
            };
            match reader.read(raw) {
                Ok(s) if s == size => (),
                _ => return None,
            }
        }
        Some(result)
    }
}

trait ToByteStream: Default + Sized {
    fn write_to<W: Write>(&self, writer: &mut W) {
        let size = std::mem::size_of::<Self>();
        let raw = unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, size) };
        let _ = writer.write(raw).unwrap();
    }
}

fn write_vec_to<T: Sized, W: Write>(data: &[T], writer: &mut W) {
    let t_size = std::mem::size_of::<T>();
    let len = data.len();
    let raw = unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, t_size * len) };
    let _ = writer.write(raw).unwrap();
}

fn read_vec_from<T: Sized + Default + Clone, R: Read>(
    reader: &mut R,
    len: usize,
) -> Option<Vec<T>> {
    let t_size = std::mem::size_of::<T>();
    let mut result = vec![T::default(); len];
    let raw =
        unsafe { std::slice::from_raw_parts_mut(result.as_mut_ptr() as *mut u8, t_size * len) };
    match reader.read(raw) {
        Ok(s) if s == t_size * len => Some(result),
        _ => None,
    }
}

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
#[repr(C)]
pub struct BasicFileHeader {
    pub length: u32,
    pub runnr: u32,
    pub run_mod: u32,
    pub seral: u32,
    pub first_event: u32,
    pub first_event_sec: u32,
    pub last_event: u32,
    pub last_event_sec: u32,
}

impl Display for BasicFileHeader {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "length: {}", self.length).and(writeln!(f, "runnr: {}", self.runnr).and(
            writeln!(f, "run_mod: {}", self.run_mod).and(
                writeln!(f, "serial: {}", self.seral).and(
                    writeln!(f, "first_event: {}", self.first_event).and(
                        writeln!(f, "first_event_sec: {}", self.first_event_sec).and(
                            writeln!(f, "last_event: {}", self.last_event).and(writeln!(
                                f,
                                "last_event_sec: {}",
                                self.last_event_sec
                            )),
                        ),
                    ),
                ),
            ),
        ))
    }
}

impl FromByteStream for BasicFileHeader {}
impl ToByteStream for BasicFileHeader {}

#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub struct FileHeader {
    pub basic_header: BasicFileHeader,
    pub additional_header: Vec<u32>,
}

impl FileHeader {
    pub fn new() -> Self {
        let mut bh = BasicFileHeader::default();
        bh.length = std::mem::size_of::<BasicFileHeader>() as u32;
        FileHeader {
            basic_header: bh,
            additional_header: vec![0],
        }
    }

    pub fn with_additional_header(mut self, ah: Vec<u32>) -> Self {
        let al = ah.len();
        assert!(!ah.is_empty());
        self.basic_header.length =
            ((al - 1) * std::mem::size_of::<u32>() + std::mem::size_of::<BasicFileHeader>()) as u32;
        self.additional_header = ah;
        self
    }

    pub fn with_basic_file_header(mut self, bh: BasicFileHeader) -> Self {
        self.basic_header = bh;
        let al = self.additional_header.len();
        self.basic_header.length =
            ((al - 1) * std::mem::size_of::<u32>() + std::mem::size_of::<BasicFileHeader>()) as u32;
        self
    }

    pub fn read_from<R: Read>(reader: &mut R) -> Option<FileHeader> {
        if let Some(bh) = BasicFileHeader::read_from(reader) {
            let additional_length = (bh.length as usize - std::mem::size_of::<BasicFileHeader>())
                / std::mem::size_of::<u32>()
                + 1;
            if let Some(additional_header) = read_vec_from(reader, additional_length) {
                Some(
                    FileHeader::new()
                        .with_basic_file_header(bh)
                        .with_additional_header(additional_header),
                )
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) {
        self.basic_header.write_to(writer);
        write_vec_to(&self.additional_header, writer);
    }
}

impl Display for FileHeader {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{}", self.basic_header).and(writeln!(
            f,
            "additional_header: {:#?}",
            self.additional_header
        ))
    }
}

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
#[repr(C)]
pub struct EventHeader {
    pub header_length: u32,
    pub runnr: u32,
    pub eventnr: u32,
    pub t3eventnr: u32,
    pub first_ls: u32,
    pub event_sec: u32,
    pub event_nsec: u32,
    pub event_type: u16,
    pub event_vers: u16,
    pub ad1: u32,
    pub ad2: u32,
    pub ls_cnt: u32,
}

impl EventHeader {
    pub fn from_trend_data(cont: &Data, sss_corr:i32) -> Self {
        EventHeader {
            header_length: 0,
            runnr: 0,
            eventnr: cont.event_count() as u32,
            t3eventnr: 0,
            first_ls: 0,
            event_sec: (i32::from(cont.sss())+sss_corr) as u32,
            event_nsec: (f64::from(
                4 * cont.ts2() + u32::from(cont.ts1pps()) - u32::from(cont.ts1trigger()),
            ) * 2.0) as u32,
            event_type: 0,
            event_vers: 0,
            ad1: 0,
            ad2: 0,
            ls_cnt: 1,
        }
    }
}

impl Display for EventHeader {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "header_length: {}", self.header_length).and(
            writeln!(f, "runnr: {}", self.runnr).and(writeln!(f, "eventnr: {}", self.eventnr).and(
                writeln!(f, "t3eventnr: {}", self.t3eventnr).and(
                    writeln!(f, "first_ls: {}", self.first_ls).and(
                        writeln!(f, "event_sec: {}", self.event_sec).and(
                            writeln!(f, "event_nsec: {}", self.event_nsec).and(
                                writeln!(f, "event_type: {}", self.event_type).and(
                                    writeln!(f, "event_vers: {}", self.event_vers).and(
                                        writeln!(f, "ad1: {}", self.ad1).and(
                                            writeln!(f, "ad2: {}", self.ad2).and(writeln!(
                                                f,
                                                "ls_cnt: {}",
                                                self.ls_cnt
                                            )),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            )),
        )
    }
}

impl FromByteStream for EventHeader {}
impl ToByteStream for EventHeader {}

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
#[repr(C)]
pub struct LocalStationHeader {
    pub length: u16,
    pub event_nr: u16,
    pub ls_id: u16,
    pub header_length: u16,
    pub gps_seconds: u32,
    pub gps_nanoseconds: u32,
    pub trigger_flag: u16,
    pub trigger_pos: u16,
    pub sampling_freq: u16,
    pub channel_mask: u16,
    pub adc_resolution: u16,
    pub trace_length: u16,
    pub version: u16,
}

impl LocalStationHeader {
    pub fn from_trend_data(cont: &Data, sss_corr:i32) -> Self {
        LocalStationHeader {
            length: 0,
            event_nr: cont.event_count() as u16,
            ls_id: (cont.ip() & 0xffff) as u16,
            header_length: 0,
            gps_seconds: (i32::from(cont.sss())+sss_corr) as u32,
            gps_nanoseconds: (f64::from(
                4 * cont.ts2() + u32::from(cont.ts1pps()) - u32::from(cont.ts1trigger()),
            ) * 2.1) as u32,
            trigger_flag: u16::from(cont.trig_pattern()),
            trigger_pos: 0,
            sampling_freq: 0,
            channel_mask: 0,
            adc_resolution: 12,
            trace_length: 0,
            version: 0,
        }
    }
}

impl Display for LocalStationHeader {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "length: {}", self.length).and(
            writeln!(f, "event_nr: {}", self.event_nr).and(
                writeln!(f, "ls_id: {}", self.ls_id).and(
                    writeln!(f, "header_length: {}", self.header_length).and(
                        writeln!(f, "gps_seconds: {}", self.gps_seconds).and(
                            writeln!(f, "gps_nanoseconds: {}", self.gps_nanoseconds).and(
                                writeln!(f, "trigger_flag: {}", self.trigger_flag).and(
                                    writeln!(f, "trigger_pos: {}", self.trigger_pos).and(
                                        writeln!(f, "sampling_freq: {}", self.sampling_freq).and(
                                            writeln!(f, "channel_mask: {}", self.channel_mask).and(
                                                writeln!(
                                                    f,
                                                    "adc_resolution: {}",
                                                    self.adc_resolution
                                                ).and(
                                                    writeln!(
                                                        f,
                                                        "trace_length: {}",
                                                        self.trace_length
                                                    ).and(writeln!(f, "version: {}", self.version)),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        )
    }
}

impl FromByteStream for LocalStationHeader {}
impl ToByteStream for LocalStationHeader {}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct LocalStation {
    pub header: LocalStationHeader,
    pub header_data: Vec<u16>,
    pub adc_buffer: Vec<u16>,
}

impl LocalStation {
    pub fn size(&self) -> usize {
        std::mem::size_of_val(&self.header)
            + self.header_data.len() * std::mem::size_of::<u16>()
            + self.adc_buffer.len() * std::mem::size_of::<u16>()
    }

    pub fn new(lsh: LocalStationHeader, header_data: Vec<u16>, adc_buffer: Vec<u16>) -> Self {
        let mut result = LocalStation {
            header: lsh,
            header_data,
            adc_buffer,
        };
        result.header.length = result.size() as u16 / 2;
        result.header.header_length = result.header_data.len() as u16 + 13;
        result.header.trace_length = result.adc_buffer.len() as u16;
        result
    }

    pub fn read_from<R: Read>(reader: &mut R) -> Option<LocalStation> {
        if let Some(h) = LocalStationHeader::read_from(reader) {
            let hl = h.header_length as usize - 13;
            let dl = h.trace_length as usize;
            if let Some(header_data) = read_vec_from(reader, hl) {
                if let Some(adc_buffer) = read_vec_from(reader, dl) {
                    Some(LocalStation {
                        header: h,
                        header_data,
                        adc_buffer,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) {
        self.header.write_to(writer);
        write_vec_to(&self.header_data, writer);
        write_vec_to(&self.adc_buffer, writer);
    }
}

impl Display for LocalStation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "header: {}", self.header).and(
            writeln!(f, "header_data: {:#?}", self.header_data).and(writeln!(
                f,
                "payload: [u16; {}]",
                self.adc_buffer.len()
            )),
        )
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Event {
    pub header: EventHeader,
    pub local_station_list: Vec<LocalStation>,
}

impl Event {
    pub fn size(&self) -> usize {
        let mut result = std::mem::size_of::<EventHeader>();
        for ls in &self.local_station_list {
            result += ls.size();
        }
        result
    }

    pub fn new(h: EventHeader) -> Self {
        let mut result = Event {
            header: h,
            local_station_list: Vec::new(),
        };
        result.header.header_length = result.size() as u32 - 3;
        result
    }

    pub fn push_local_station(&mut self, ls: LocalStation) {
        self.local_station_list.push(ls);
        self.header.ls_cnt = self.local_station_list.len() as u32;
        self.header.header_length = self.size() as u32 - 4;
    }

    pub fn from_trend_data(cont: &Data, adc_buffer: &[u16], sss_corr:i32) -> Self {
        let ls = LocalStation::new(
            LocalStationHeader::from_trend_data(cont, sss_corr),
            vec![],
            adc_buffer.to_owned(),
        );
        let eh = EventHeader::from_trend_data(cont, sss_corr);
        let mut ev = Event::new(eh);
        ev.push_local_station(ls);
        ev
    }

    pub fn read_from<R: Read>(reader: &mut R) -> Option<Event> {
        if let Some(h) = EventHeader::read_from(reader) {
            let nls = h.ls_cnt as usize;
            let mut lsl = Vec::<LocalStation>::new();
            for _i in 0..nls {
                if let Some(ls) = LocalStation::read_from(reader) {
                    lsl.push(ls);
                } else {
                    return None;
                }
            }
            Some(Event {
                header: h,
                local_station_list: lsl,
            })
        } else {
            None
        }
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) {
        self.header.write_to(writer);
        for ls in &self.local_station_list {
            ls.write_to(writer);
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "header: {}", self.header).and(writeln!(
            f,
            "with payload of [LS; {}]",
            self.local_station_list.len()
        ))
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EventFile {
    pub header: FileHeader,
    pub event_list: Vec<Event>,
}

impl EventFile {
    pub fn read_from<R: Read>(reader: &mut R) -> Option<EventFile> {
        if let Some(h) = FileHeader::read_from(reader) {
            let mut el = Vec::new();
            while let Some(ev) = Event::read_from(reader) {
                el.push(ev);
            }
            Some(EventFile {
                header: h,
                event_list: el,
            })
        } else {
            None
        }
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) {
        self.header.write_to(writer);
        for ev in &self.event_list {
            ev.write_to(writer);
        }
    }
}

impl Display for EventFile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "header: {}", self.header).and(writeln!(
            f,
            "event_list: [e; {}]",
            self.event_list.len()
        ))
    }
}
