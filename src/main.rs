use input::event::EventTrait;
use input::{Libinput, LibinputInterface};
use std::fs::{File, OpenOptions};
use std::fs::read as read_file;
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;
use rodio::{Decoder, OutputStream, source::Source};
use std::sync::Arc;
use std::io::Cursor;
use std::error::Error;
use std::io::Result as res;
extern crate libc;

use libc::{O_RDONLY, O_RDWR, O_WRONLY};

struct Interface;


pub struct Sound (Arc<Vec<u8>>);

impl AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Sound {
    pub fn load(filename: &str) -> res<Sound> {
        use std::fs::File;
        let  buf = read_file(filename)?;
        Ok(Sound(Arc::new(buf)))
    }
    pub fn cursor(self: &Self) -> Cursor<Sound> {
        Cursor::new(Sound(self.0.clone()))
    }
    pub fn decoder(self: &Self) -> rodio::Decoder<Cursor<Sound>> {
        Decoder::new(self.cursor()).unwrap()
    }
}


impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        unsafe {
            File::from(fd);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sound = Sound::load("fart.mp3")?;
    let mut count = 0;
    loop {
        input.dispatch().unwrap();
        for event in &mut input  {
            if event.device().name() == "Elan TrackPoint"{
                count += 1;
                if count == 50 {
                    let _ = stream_handle.play_raw(sound.decoder().convert_samples());
                    std::thread::sleep(std::time::Duration::from_secs(4));
                    count = 0;
                }
            }
        }
    }
}
