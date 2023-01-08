use crate::models::clink::CLink;

pub trait ToCLink {
    fn to_clink(&self) -> CLink;
}
