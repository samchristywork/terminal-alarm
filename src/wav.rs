use fon::chan::Ch16;
use fon::pos::{Left, Right};
use fon::Audio;
use std::convert::TryInto;
use std::mem::size_of;
use std::{fs, io};

pub fn write(audio: Audio<Ch16, 2>, filename: &str) -> io::Result<()> {
    let mut buf = vec![];
    let n: u32 = audio.len().try_into().unwrap();

    buf.extend(b"RIFF");
    buf.extend(&(36u32 + n).to_le_bytes());
    buf.extend(b"WAVE");
    buf.extend(b"fmt ");
    buf.extend(&(16u32).to_le_bytes());
    buf.extend(&(0x0001u16).to_le_bytes());
    buf.extend(&(2u16).to_le_bytes());
    buf.extend(&u32::from(audio.sample_rate()).to_le_bytes());
    buf.extend(&(4 * u32::from(audio.sample_rate())).to_le_bytes());
    buf.extend(&(size_of::<u16>() as u16 * 2u16).to_le_bytes());
    buf.extend(&(16u16).to_le_bytes());
    buf.extend(b"data");
    buf.extend(&(4 * audio.len() as u32).to_le_bytes());

    for frame in audio.iter() {
        buf.extend(&i16::from(frame[Left]).to_le_bytes());
        buf.extend(&i16::from(frame[Right]).to_le_bytes());
    }

    fs::write(filename, buf)
}
