use crate::mods::MODAUDIOFILES;
use byteorder::WriteBytesExt;

pub mod AudioPcks;






pub fn create_chaudpck(
  mut mod_pck_file: impl std::io::Write + std::io::Seek,
) -> Result<(), std::io::Error>  {
let mod_audio= MODAUDIOFILES.get().unwrap().lock().unwrap();
//let mut mod_pck_file = std::fs::File::create("audio/chaudloader.pck")?;
let num_wem = mod_audio.wems.len() as u32;
let mut wem_file_offset = 0x8C + num_wem * 20;
// Write AKPK
mod_pck_file.write_all(&[0x41, 0x4B, 0x50, 0x4B])?;
// Write Pck header length?
mod_pck_file.write_u32::<byteorder::LittleEndian>(wem_file_offset)?;
// Write next part of header
mod_pck_file.write_all( &[0x01, 0x00, 0x00, 0x00, 0x68, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00])?;
// Write length of entries
mod_pck_file.write_u32::<byteorder::LittleEndian>(num_wem * 20)?;
// Write next part of header
mod_pck_file.write_all(&[
    0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
    0x34, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x4C, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x5E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x00, 0x68, 0x00, 0x69, 0x00, 0x6E, 0x00,
    0x65, 0x00, 0x73, 0x00, 0x65, 0x00, 0x00, 0x00, 0x65, 0x00, 0x6E, 0x00, 0x67, 0x00, 0x6C, 0x00,
    0x69, 0x00, 0x73, 0x00, 0x68, 0x00, 0x28, 0x00, 0x75, 0x00, 0x73, 0x00, 0x29, 0x00, 0x00, 0x00,
    0x6A, 0x00, 0x61, 0x00, 0x70, 0x00, 0x61, 0x00, 0x6E, 0x00, 0x65, 0x00, 0x73, 0x00, 0x65, 0x00,
    0x00, 0x00, 0x73, 0x00, 0x66, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])?;
// Write number of entries
mod_pck_file.write_u32::<byteorder::LittleEndian>(num_wem)?;
// IDs / Hashes need to be sorted in ascending order or the lookup fails
let mut hashes: Vec<_> = mod_audio.wems.keys().cloned().collect();
hashes.sort();
// Skip entries and write wem files first

/*
mod_pck_file.set_len(wem_file_offset as u64)?;
Nothing wrong with this, but Weenie implemented his callbacks to take only write+seek traits and file is a subset of that,
so we can't use set_len because write+seek don't implement it. We have to do a weird way. 

*/

let length=wem_file_offset as u64-mod_pck_file.stream_position()?;

let filler=[0;1];
for _i in 0..length{
  mod_pck_file.write(&filler)?;
}


mod_pck_file.seek(std::io::SeekFrom::Start(wem_file_offset as u64))?;
// Write the actual wems and keep track and offsets and lengths
let mut wem_offset_lens : Vec<(u32, u32)> = Vec::with_capacity(hashes.len());
for hash in &hashes {
    let path = mod_audio.wems.get(hash).unwrap();
    let wem_contents : Vec<u8> = std::fs::read(path)?;
    mod_pck_file.write_all(wem_contents.as_slice())?;
    wem_offset_lens.push((wem_file_offset, wem_contents.len() as u32));
    wem_file_offset += wem_contents.len() as u32;
}
// Go back to write the actual entries
mod_pck_file.seek(std::io::SeekFrom::Start(0x8C as u64))?;
for (&hash, &(wem_offset, wem_size)) in hashes.iter().zip(wem_offset_lens.iter()) {
    mod_pck_file.write_u32::<byteorder::LittleEndian>(hash)?;
    mod_pck_file.write_u32::<byteorder::LittleEndian>(0x01)?;
    mod_pck_file.write_u32::<byteorder::LittleEndian>(wem_size)?;
    mod_pck_file.write_u32::<byteorder::LittleEndian>(wem_offset)?;
    mod_pck_file.write_u32::<byteorder::LittleEndian>(0x00)?;
}
Ok(())
}