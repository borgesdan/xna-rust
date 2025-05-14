use crate::xna::csharp::Exception;

trait IContentManager {
    fn get_root_directory(&self) -> &str;
    fn set_root_directory(&mut self, value: &str) -> Result<(), Exception>;
    fn unload(&mut self) -> Result<(), Exception>;
    fn load<T>(&mut self, asset_name: &str) -> Result<T, Exception>;
}