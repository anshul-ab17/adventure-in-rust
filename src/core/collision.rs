use crate::core::game_object::GameObject;

pub fn collides(a: &dyn GameObject, b: &dyn GameObject) -> bool {
    let dx = a.position().x - b.position().x;
    let dy = a.position().y - b.position().y;

    let dist_sq = dx * dx + dy * dy;
    let r = a.radius() + b.radius();

    dist_sq <= r * r
}
