use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Version {
    hash: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CitFileContent {
    num_versions: usize,
    current_version: Option<String>,
    versions: HashMap<String, Version>,
}

pub struct CitFile {
    filepath: PathBuf,
    filename: String,
    cit_file_path: PathBuf,
    not_initialized: bool,
    content: Option<CitFileContent>,
}

impl CitFile {
    pub fn new(filepath: &str) -> Self {
        let filepath = Path::new(filepath).canonicalize()
            .expect("Failed to get absolute path");
        let filename = filepath.file_name()
            .expect("Invalid filename")
            .to_string_lossy()
            .into_owned();
        
        let cit_file_path = Self::build_cit_file_path(&filepath);
        let mut cit = CitFile {
            filepath,
            filename,
            cit_file_path,
            not_initialized: false,
            content: None,
        };

        if let Err(_) = cit.load_cit_file() {
            cit.not_initialized = true;
        }

        cit
    }

    fn build_cit_file_path(filepath: &Path) -> PathBuf {
        let mut hasher = Sha256::new();
        hasher.update(filepath.to_string_lossy().as_bytes());
        let path_hash = format!("{:x}", hasher.finalize());
        
        let mut cit_dir = dirs::home_dir().expect("Could not find home directory");
        cit_dir.push(".cit");
        cit_dir.push("files");
        fs::create_dir_all(&cit_dir).expect("Failed to create .cit directory");
        
        cit_dir.push(format!("{}.cit", path_hash));
        cit_dir
    }

    fn load_cit_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&self.cit_file_path)?;
        self.content = Some(serde_yaml::from_str(&content)?);
        Ok(())
    }

    fn save_cit_file(&self) {
        if let Some(content) = &self.content {
            fs::write(
                &self.cit_file_path,
                serde_yaml::to_string(&content).expect("Failed to serialize content")
            ).expect("Failed to write cit file");
        }
    }

    pub fn init(&mut self) {
        if self.cit_file_path.exists() {
            println!("cit file already exists for {}", self.filename);
            return;
        }

        println!("Initializing {} cit...", self.filename);

        self.content = Some(CitFileContent {
            num_versions: 0,
            current_version: None,
            versions: HashMap::new(),
        });

        self.save_cit_file();
        self.not_initialized = false;
        self.add_version("baseline");
    }

    pub fn add_version(&mut self, version_name: &str) {
        if self.not_initialized {
            println!("File not initialized. Please run cit init first.");
            return;
        }

        let content = fs::read_to_string(&self.filepath)
            .expect("Failed to read file");

        if let Some(ref mut cit_content) = self.content {
            cit_content.num_versions += 1;
            cit_content.versions.insert(
                version_name.to_string(),
                Version {
                    hash: Self::generate_hash(version_name),
                    content,
                }
            );
            cit_content.current_version = Some(version_name.to_string());
            self.save_cit_file();
        }

        println!("Added version {} to {}", version_name, self.filename);
    }

    fn generate_hash(version_name: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(version_name.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn list_versions(&self) {
        if self.not_initialized {
            println!("File not initialized. Please run cit init first.");
            return;
        }

        if let Some(content) = &self.content {
            println!("Listing versions for {}...", self.filename);
            println!("Total # of versions: {}", content.num_versions);
            
            for (name, version) in &content.versions {
                if Some(name) == content.current_version.as_ref() {
                    println!("{}: {} <==", name, version.hash);
                } else {
                    println!("{}: {}", name, version.hash);
                }
            }
        }
    }

    pub fn commit(&mut self) {
        if self.not_initialized {
            println!("File not initialized. Please run cit init first.");
            return;
        }

        if let Some(content) = &mut self.content {
            if let Some(current_version) = &content.current_version {
                println!("Committing changes to version {}...", current_version);
                
                let file_content = fs::read_to_string(&self.filepath)
                    .expect("Failed to read file");

                if let Some(version) = content.versions.get_mut(current_version) {
                    version.content = file_content;
                    self.save_cit_file();
                }
            }
        }
    }

    pub fn switch(&mut self, version: &str, prompt: bool) {
        if self.not_initialized {
            println!("File not initialized. Please run cit init first.");
            return;
        }

        let version_content = if let Some(content) = &self.content {
            if !content.versions.contains_key(version) {
                println!("Version {} does not exist", version);
                return;
            }
            content.versions.get(version).map(|v| v.content.clone())
        } else {
            return;
        };

        if prompt {
            println!("Warning: all uncommited changes will be lost...");
            println!("Continue? [y/n]");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "n" {
                return;
            }
        }

        if let Some(content) = &mut self.content {
            content.current_version = Some(version.to_string());
        }
        self.save_cit_file();

        if let Some(content) = version_content {
            fs::write(&self.filepath, content)
                .expect("Failed to write file content");
            println!("Switched to version {} for {}", version, self.filename);
        }
    }
    
    pub fn remove(&mut self, version: &str) {
        if self.not_initialized {
            println!("File not initialized. Please run cit init first.");
            return;
        }

        // Check if the version is the baseline - cant remove the baseline
        if version == "baseline" {
            println!("Cannot remove the baseline version");
            return;
        }
        // check if version exists
        if !self.content.as_ref().unwrap().versions.contains_key(version) {
            println!("Version {} does not exist", version);
            return;
        }
        // check if the version is the current version
        if self.content.as_ref().unwrap().current_version.as_ref().unwrap() == version {
            println!("Cannot remove the current version, switch to another version first");
            return;
        }
        // // remove the version from the map
        // self.content.as_mut().unwrap().versions.remove(version);
        // // update the number of versions
        // self.content.as_mut().unwrap().num_versions -= 1;

        if let Some(content) = &mut self.content {
            // remove the version from the map
            content.versions.remove(version);
            // update the number of versions
            content.num_versions -= 1;

        }
        self.save_cit_file();

    } 

    pub fn clear(&self) {
        println!("Clearing data for {}...", self.filename);
        println!("not implemented yet");
    }
}