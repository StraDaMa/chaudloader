use std::path::PathBuf;
use std::error::Error;
#[derive(Clone)]
pub struct ModAudioPath {
  modpath:Box<PathBuf>
}
impl ModAudioPath {
  pub fn new(modpath:Box<PathBuf>) -> Result<Self, Box<dyn Error>> {
  Ok( ModAudioPath{
    modpath:modpath
   })


}

pub fn create_otherpck(self,
  mut mod_pck_file: impl std::io::Write + std::io::Seek
) -> Result<(), std::io::Error> {

  let modfile=std::fs::read(self.modpath.to_path_buf())?;
  mod_pck_file.write(&*modfile)?;

Ok(())  
}



}



