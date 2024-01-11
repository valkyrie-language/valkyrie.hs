pub type Result<T> = std::result::Result<T, PlotError>;


#[derive(Debug, Copy, Clone)]
pub struct PlotError {
    kind: Box<PlotErrorKind>,
}

pub enum PlotErrorKind {

}