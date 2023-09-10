pub enum CharSet
{
    Ascii,
    Custom(Box<dyn Iterator<Item = char>>),
}
