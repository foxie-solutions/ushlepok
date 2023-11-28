use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UshlepokMeta {
    pub description: String,
    pub redirect_url: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct Ushlepok {
    pub meta: UshlepokMeta,
    pub image: Vec<u8>,
}

#[derive(Clone)]
pub struct Container {
    path: PathBuf,
    image_path: PathBuf,
    current: Arc<Mutex<Option<Ushlepok>>>,
}

impl Container {
    pub fn new<P: AsRef<Path>>(path: P, image_path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            image_path: image_path.as_ref().to_path_buf(),
            current: Arc::new(Mutex::new(None)),
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        // Meta
        let meta_value = match self.current.lock().unwrap().as_ref() {
            Some(ushlepok) => serde_json::to_value(&ushlepok.meta)?,
            None => serde_json::Value::Null,
        };
        let mut file = std::fs::File::create(&self.path)?;
        serde_json::to_writer_pretty(&mut file, &meta_value)?;

        // Image
        if let Some(ushlepok) = self.current.lock().unwrap().as_ref() {
            std::fs::write(&self.image_path, &ushlepok.image)?;
        }

        Ok(())
    }

    pub fn load(&self) -> Result<(), std::io::Error> {
        let file = std::fs::File::open(&self.path)?;
        let meta: UshlepokMeta = serde_json::from_reader(file)?;
        let image = std::fs::read(&self.image_path)?;
        self.current
            .lock()
            .unwrap()
            .replace(Ushlepok { meta, image });
        Ok(())
    }

    pub fn set(&self, ushlepok: Ushlepok) -> Result<(), std::io::Error> {
        self.current.lock().unwrap().replace(ushlepok);
        self.save()
    }

    pub fn get(&self) -> Option<Ushlepok> {
        self.current.lock().unwrap().clone()
    }
}
