use std::path::{Path, PathBuf};

use mp3_metadata::OptionalAudioTags;

#[derive(Debug)]
pub struct MusicFile {
    path: PathBuf,
    audio_tag: Vec<OptionalAudioTags>,
}

impl MusicFile {
    pub fn new(path: &Path, audio_info: Vec<OptionalAudioTags>) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            audio_tag: audio_info,
        }
    }
}
