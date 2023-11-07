pub mod cache {
    use std::{collections::HashMap, fs::File, io::Read, str::FromStr};

    use zip::ZipArchive;

    use std::{fs, path};

    use flate2::read::GzDecoder;

    pub struct Cache {
        slpk_map: HashMap<String, ZipArchive<File>>,
        slpk_content_map: HashMap<String, HashMap<String, Vec<u8>>>,
    }

    impl Cache {
        pub fn new() -> Cache {
            Cache {
                slpk_map: HashMap::new(),
                slpk_content_map: HashMap::new(),
            }
        }

        fn add(&mut self, filename: &str, archive: ZipArchive<File>) {
            self.slpk_map
                .insert(String::from_str(filename).unwrap(), archive);
            self.slpk_content_map
                .insert(String::from_str(filename).unwrap(), HashMap::new());
        }

        pub fn remove(&mut self, filename: &String) {
            self.slpk_map.remove(filename);
        }

        pub fn get(&mut self, filename: &str) -> &mut ZipArchive<File> {
            return self.slpk_map.get_mut(filename).unwrap();
        }

        pub fn load_slpk(&mut self, filename: &str) {
            let file = fs::File::open(filename).unwrap();
            let archive = zip::ZipArchive::new(file).unwrap();

            let filename = path::Path::new(filename);
            let filename = filename.file_name().unwrap().to_str().unwrap();

            self.add(filename, archive);
        }

        pub fn read_file(&mut self, archive_name: &str, file_name: &str) -> Option<Vec<u8>> {
            if let Some(archive) = self.slpk_map.get_mut(archive_name) {
                let content_map = self.slpk_content_map.get_mut(archive_name).unwrap();
                if let Some(file) = content_map.get(file_name) {
                    return Some(file.clone());
                } else {
                    let mut file = archive.by_name(file_name).unwrap();

                    if file.name().ends_with(".gz") {
                        let mut decoder = GzDecoder::new(file);

                        let mut bytes: Vec<u8> = Vec::new();

                        let _ = decoder.read_to_end(&mut bytes);

                        content_map.insert(String::from_str(file_name).unwrap(), bytes.clone());

                        return Some(bytes);
                    } else {
                        let mut bytes: Vec<u8> = Vec::new();
                        let _ = file.read_to_end(&mut bytes);

                        content_map.insert(String::from_str(file_name).unwrap(), bytes.clone());

                        return Some(bytes);
                    }
                }
            }

            None
        }
    }
}
