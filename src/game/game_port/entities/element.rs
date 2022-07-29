use super::*;

pub fn get_element(
    id: String,
    transform: Transform,
) -> (UI, Transform) {
    (UI { html_id: id }, transform)
}
