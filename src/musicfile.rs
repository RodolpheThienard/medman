use std::path::{Path, PathBuf};

use mp3_metadata::AudioTag;

#[derive(Debug)]
pub struct MusicFile {
    path: PathBuf,
    audio_tag: AudioTag,
}

impl MusicFile {
    pub fn new(path: &Path, audio_info: AudioTag) -> MusicFile {
        MusicFile {
            path: path.to_path_buf(),
            audio_tag: audio_info,
        }
    }
}
