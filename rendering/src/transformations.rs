use bevy::{
    prelude::{Component, ReflectComponent, Vec3},
    reflect::Reflect,
};

#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Translation {
    pub v: Vec3,
}

#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Scaling(Vec3);

#[derive(Component, Clone, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Rotation{
    pub v: Vec3,
}
