use image::ImageError;

#[derive(Debug)]
pub enum Error
{
    Init(glfw::InitError),
    Glfw,
    Image(ImageError),
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<glfw::InitError> for Error
{
    fn from(value: glfw::InitError) -> Self
    {
        Error::Init(value)
    }
}

impl From<ImageError> for Error
{
    fn from(value: ImageError) -> Self
    {
        Error::Image(value)
    }
}
