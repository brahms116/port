use super::*;
pub fn get_rectangle(
    height: f64,
    width: f64,
    transform: Transform,
) -> (Transform, Render, Height, Width, Rectangle) {
    let surface = Surface {
        points: vec![],
        color: RGBA {
            r: 67,
            g: 78,
            b: 66,
            a: 1.0,
        },
    };

    (
        transform,
        Render(vec![surface]),
        Height(height),
        Width(width),
        Rectangle(),
    )
}
