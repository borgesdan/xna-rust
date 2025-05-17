use std::path::Path;
use crate::xna::csharp::Exception;
use crate::xna::csharp::io::{FileAccess, FileMode, FileStream};
use crate::xna::framework::content::{ContentManager, IContentManager};
use crate::xna::framework::TitleContainer;

impl ContentManager{
    pub fn new(root_directory: &str)-> Self {
        ContentManager {
            root_directory: root_directory.to_string(),
            ..Default::default()
        }
    }

    // fn read_asset<T>(&self, asset_name: &str) -> Result<Option<T>, Exception> {
    //
    // }
    //
    // fn open_stream(asset_name: &str) -> Result<Stre, Exception> {}

    pub fn open_stream(&self, asset_name: &str) -> Result<FileStream, Exception> {
        let path = Path::new(self.root_directory.as_str());
        let join = path.join(asset_name.to_string() + ".xnb");
        let full_root = join.to_str().unwrap();

        if self.is_root_directory_absolute {
            FileStream::new_with_access(asset_name, FileMode::Open, FileAccess::Read)
        } else {
            TitleContainer::open_stream(full_root)
        }
    }
}

impl IContentManager for ContentManager {
    fn get_root_directory(&self) -> &str {
        self.root_directory.as_str()
    }

    fn set_root_directory(&mut self, value: &str) -> Result<(), Exception> {
        self.root_directory = value.to_string();
        Ok(())
    }

    fn unload(&mut self) -> Result<(), Exception> {
        Ok(())
    }

    fn load<T>(&mut self, asset_name: &str) -> Result<Option<T>, Exception> {
        if asset_name.is_empty() {
            return Err(Exception::new("Asset name is empty.", None));
        }

        //let obj3 = self.read_asset(asset_name)?;

        Ok(None)
    }
}