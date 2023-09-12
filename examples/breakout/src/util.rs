use tori::math::{Vec2, Vec4};
pub fn point_inside_rect(rect: Vec4, point: Vec2) -> bool
{
    let rect_right = rect.x + rect[2];
    let rect_bottom = rect.y + rect[3];

    point.x >= rect.x && point.x <= rect_right && point.y >= rect.y && point.y <= rect_bottom
}

fn clamp(value: f32, min: f32, max: f32) -> f32
{
    if value < min
    {
        return min;
    }
    else if value > max
    {
        return max;
    }
    value
}


pub fn circle_rect_intersects(circle: Vec2, radius: f32, rect: Vec4) -> bool
{
    let closest_x = clamp(circle.x, rect.x, rect.x + rect[2]);
    let closest_y = clamp(circle.y, rect.y, rect.y + rect[3]);

    let distance_x = circle.x - closest_x;
    let distance_y = circle.y - closest_y;

    let distance_squared = distance_x * distance_x + distance_y * distance_y;
    let radius_squared = radius * radius;

    distance_squared <= radius_squared
}
