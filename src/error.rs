use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error
{
    Init(#[from] glfw::InitError),
    Glfw,
    Image(#[from] image::ImageError),
    Io(#[from] std::io::Error),
    Font(#[from] freetype::Error),
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}
