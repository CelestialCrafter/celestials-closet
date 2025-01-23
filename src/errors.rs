use warp::reject::Reject;

#[derive(Debug)]
pub struct Failed(pub eyre::Report);
impl Reject for Failed {}
