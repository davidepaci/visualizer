use bevy::prelude::Component;

#[derive(Default, Component)]
pub struct LastUpdate {
    pub(crate) value: f64,
}
