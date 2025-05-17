use std::cmp::max;
use std::env;
use std::path::{Path, PathBuf};
use crate::xna::csharp::Exception;
use crate::xna::csharp::io::{FileHelper, FileStream};
use crate::xna::ExceptionConverter;
use crate::xna::framework::TitleContainer;

impl TitleContainer {
    fn get_executable_dir() -> Option<PathBuf> {
        env::args().next()
            .map(PathBuf::from)
            .and_then(|p| p.canonicalize().ok())
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
    }

    pub fn open_stream(name: &str) -> Result<FileStream, Exception> {
        let name1 = if !name.is_empty() {
            Self::get_clean_path(name)
        } else  {
            Err(Exception::new("The argument 'name' cannot be empty", None))
        };

        let executable_dir = Self::get_executable_dir()
            .unwrap_or_exception("Error getting current directory of executable.")?;

        let path1 = executable_dir.to_str().unwrap();

        let path2 = Path::new(path1);
        let path3 = path2.join(name);

        let full_path = path3.to_str().unwrap_or_exception("Error containing file path.")?;

        FileHelper::open_read(full_path)
    }

    pub fn get_clean_path(path: &str) -> Result<String, Exception> {
        let mut path =  path.replace("/", "\\");
        path =  path.replace("\\.\\", "\\");

        while path.starts_with(".\\") {
            let len = ".\\".len();
            path = path.get(len..)
                .unwrap_or_exception("Error getting a substring for a string starting with '.\\'.")?
                .to_string();
        }

        while path.ends_with("\\.") {
            let len = "\\.".len();
            path = if path.len() <= len {
                "\\".to_string()
            } else {
                path.get(0..path.len() - len)
                    .unwrap_or_exception("Error getting a substring for a string ending with '\\.'.")?
                    .to_string()
            }
        }

        let mut position1 = 0usize;
        let mut start_index = 0usize;

        while start_index < path.len() {
            let matched = path[start_index..].find("\\..\\");

            if matched.is_none() {
                break;
            }

            position1 = matched.unwrap();

            start_index = Self::collapse_parent_directory(&mut path, position1, "\\..\\".len())?;
        }

        if path.ends_with("\\..") {
            let position2 = path.len() - "\\..".len();

            if position2 > 0 {
                Self::collapse_parent_directory(&mut path, position2, "\\..".len())?;
            }
        }

        if path == "." {
            path = String::new();
        }

        Ok(path.to_string())
    }

    fn collapse_parent_directory(path: &mut String, position: usize, remove_length: usize) -> Result<usize, Exception> {
        let start_index = &path[position - 1..]
            .rfind('\\')
            .unwrap_or_exception("Error finding last index of character '\\'")?
            + 1;

        let p1 = &mut path[position - start_index + remove_length..].to_string();
        p1.remove(start_index).to_string();

        Ok(max(start_index - 1, 1usize))
    }
}