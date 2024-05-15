use bevy::prelude::*;

#[derive(Component)]
pub struct Collider {
    pub rect: Rectangle,
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            rect: Rectangle::new(width, height),
        }
    }
}

pub fn draw_gizmos(mut gizmos: Gizmos, query: Query<(&Transform, &Collider)>) {
    for (transform, collider) in &mut query.iter() {
        let translation = transform.translation.xy();
        gizmos.primitive_2d(collider.rect, translation, 0., Color::rgb(0., 1., 0.));
    }
}

