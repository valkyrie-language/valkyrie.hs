pub type Result<T> = std::result::Result<T, PlotError>;


#[derive(Debug,  Clone)]
pub struct PlotError {
    kind: Box<PlotErrorKind>,
}

#[derive(Debug,  Clone)]
pub enum PlotErrorKind {

}