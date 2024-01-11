#[derive(Debug, Copy, Clone)]
pub enum PlotError {
    UnknownError
}

pub type Result<T> = std::result::Result<T, PlotError>;
