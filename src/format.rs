pub struct RtpHeader {
    csrc: u8,
    marker: bool,
    payload_type: u8,
    seq: u16,
    timestamp: u32,
    ssrc: u32,
}

pub struct RtpPacket {
    header: RtpHeader,
}

