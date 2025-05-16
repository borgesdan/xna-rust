// use crate::xna::csharp::Exception;
// use crate::xna::framework::content::{ContentManager, IContentManager};
//
// impl ContentManager{
//     pub fn new(root_directory: &str)-> Self {
//         ContentManager {
//             root_directory: root_directory.to_string(),
//         }
//     }
//
//     fn read_asset<T>(&self, asset_name: &str) -> Result<Option<T>, Exception> {
//
//     }
//
//     fn open_stream(asset_name: &str) -> Result<Stre, Exception> {}
// }
//
// impl IContentManager for ContentManager {
//     fn get_root_directory(&self) -> &str {
//         self.root_directory.as_str()
//     }
//
//     fn set_root_directory(&mut self, value: &str) -> Result<(), Exception> {
//         self.root_directory = value.to_string();
//         Ok(())
//     }
//
//     fn unload(&mut self) -> Result<(), Exception> {
//         Ok(())
//     }
//
//     fn load<T>(&mut self, asset_name: &str) -> Result<Option<T>, Exception> {
//         if asset_name.is_empty() {
//             return Err(Exception::new("Asset name is empty.", None));
//         }
//
//         let obj3 = self.read_asset(asset_name)?;
//
//         Ok(obj3)
//     }
// }