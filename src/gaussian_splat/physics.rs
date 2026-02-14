use avian3d::collision::collision_events::{CollisionEnd, CollisionStart};
use avian3d::prelude::*;
use bevy::prelude::*;

use super::splat_types::*;

pub struct GaussianSplatPhysicsPlugin;

impl Plugin for GaussianSplatPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SplatPhysicsEvent>()
            .add_systems(Update, setup_splat_physics)
            .add_systems(Update, update_physics_from_splats)
            .add_systems(Update, handle_splat_physics_interactions)
            .add_systems(Update, update_splat_physics_materials);
    }
}

#[derive(Component, Debug, Clone)]
pub struct SplatPhysicsBody {
    pub body_type: SplatBodyType,
    pub density: f32,
    pub friction: f32,
    pub restitution: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub lock_rotation: bool,
    pub collider_type: SplatColliderType,
}

impl Default for SplatPhysicsBody {
    fn default() -> Self {
        Self {
            body_type: SplatBodyType::Dynamic,
            density: SPLAT_PHYSICS_DENSITY,
            friction: SPLAT_PHYSICS_FRICTION,
            restitution: SPLAT_PHYSICS_RESTITUTION,
            linear_damping: 0.1,
            angular_damping: 0.1,
            lock_rotation: false,
            collider_type: SplatColliderType::Sphere,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplatBodyType {
    Static,
    Dynamic,
    Kinematic,
    Sensor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplatColliderType {
    Sphere,
    Capsule,
    Box,
    ConvexHull,
    Ball,
}

pub struct SplatPhysicsBuilder {
    body: SplatPhysicsBody,
    rigid_body: RigidBody,
    collider: Collider,
    density: ColliderDensity,
    friction: Friction,
    restitution: Restitution,
    linear_damping: LinearDamping,
    angular_damping: AngularDamping,
    locked_axes: LockedAxes,
}

impl Default for SplatPhysicsBuilder {
    fn default() -> Self {
        Self {
            body: SplatPhysicsBody::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::sphere(0.1),
            density: ColliderDensity(SPLAT_PHYSICS_DENSITY),
            friction: Friction::new(SPLAT_PHYSICS_FRICTION),
            restitution: Restitution::new(SPLAT_PHYSICS_RESTITUTION),
            linear_damping: LinearDamping(0.1),
            angular_damping: AngularDamping(0.1),
            locked_axes: LockedAxes::default(),
        }
    }
}

impl SplatPhysicsBuilder {
    pub fn static_body() -> Self {
        Self {
            body: SplatPhysicsBody {
                body_type: SplatBodyType::Static,
                ..default()
            },
            rigid_body: RigidBody::Static,
            ..default()
        }
    }

    pub fn dynamic_body() -> Self {
        Self {
            body: SplatPhysicsBody {
                body_type: SplatBodyType::Dynamic,
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            ..default()
        }
    }

    pub fn kinematic_body() -> Self {
        Self {
            body: SplatPhysicsBody {
                body_type: SplatBodyType::Kinematic,
                ..default()
            },
            rigid_body: RigidBody::Kinematic,
            ..default()
        }
    }

    pub fn with_collider(mut self, collider: Collider) -> Self {
        self.collider = collider;
        self
    }

    pub fn with_sphere(self, radius: f32) -> Self {
        self.with_collider(Collider::sphere(radius.max(0.001)))
    }

    pub fn with_box(self, half_extents: Vec3) -> Self {
        self.with_collider(Collider::cuboid(
            half_extents.x.max(0.001),
            half_extents.y.max(0.001),
            half_extents.z.max(0.001),
        ))
    }

    pub fn with_capsule(self, half_height: f32, radius: f32) -> Self {
        self.with_collider(Collider::capsule(half_height.max(0.001), radius.max(0.001)))
    }

    pub fn with_friction(mut self, friction: f32) -> Self {
        self.friction = Friction::new(friction);
        self.body.friction = friction;
        self
    }

    pub fn with_restitution(mut self, restitution: f32) -> Self {
        self.restitution = Restitution::new(restitution);
        self.body.restitution = restitution;
        self
    }

    pub fn with_density(mut self, density: f32) -> Self {
        self.density = ColliderDensity(density);
        self.body.density = density;
        self
    }

    pub fn with_damping(mut self, linear: f32, angular: f32) -> Self {
        self.linear_damping = LinearDamping(linear);
        self.angular_damping = AngularDamping(angular);
        self.body.linear_damping = linear;
        self.body.angular_damping = angular;
        self
    }

    pub fn lock_rotation(mut self) -> Self {
        self.locked_axes = LockedAxes::ROTATION_LOCKED;
        self.body.lock_rotation = true;
        self
    }

    pub fn build(
        self,
    ) -> (
        SplatPhysicsBody,
        RigidBody,
        Collider,
        ColliderDensity,
        Friction,
        Restitution,
        LinearDamping,
        AngularDamping,
        LockedAxes,
    ) {
        (
            self.body,
            self.rigid_body,
            self.collider,
            self.density,
            self.friction,
            self.restitution,
            self.linear_damping,
            self.angular_damping,
            self.locked_axes,
        )
    }
}

#[allow(dead_code)]
pub fn spawn_splat_physics(
    commands: &mut Commands,
    physics_builder: SplatPhysicsBuilder,
) -> Entity {
    let (
        body,
        rigid_body,
        collider,
        density,
        friction,
        restitution,
        linear_damping,
        angular_damping,
        locked_axes,
    ) = physics_builder.build();
    commands
        .spawn((
            body,
            rigid_body,
            collider,
            density,
            friction,
            restitution,
            linear_damping,
            angular_damping,
            locked_axes,
            Transform::default(),
            GlobalTransform::default(),
        ))
        .id()
}

fn setup_splat_physics(
    mut commands: Commands,
    query: Query<
        (Entity, &GaussianSplat, &Transform),
        (Without<RigidBody>, Without<SplatPhysicsBody>),
    >,
) {
    for (entity, splat, _transform) in query.iter() {
        if !splat.physics_enabled {
            continue;
        }

        let radius = splat.get_world_radius();
        let density = splat.solidity * SPLAT_PHYSICS_DENSITY;
        let friction = (1.0 - splat.texture_weight) * SPLAT_PHYSICS_FRICTION;

        let builder = SplatPhysicsBuilder::dynamic_body()
            .with_sphere(radius)
            .with_density(density)
            .with_friction(friction)
            .with_restitution(SPLAT_PHYSICS_RESTITUTION)
            .with_damping(0.5, 0.5);

        let (body, rigid_body, collider, dens, fric, rest, linear_damp, angular_damp, locked) =
            builder.build();

        commands.entity(entity).insert((
            body,
            rigid_body,
            collider,
            dens,
            fric,
            rest,
            linear_damp,
            angular_damp,
            locked,
        ));
    }
}

fn update_physics_from_splats(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Collider,
        &mut Friction,
        &mut Restitution,
        &mut ColliderDensity,
        &GaussianSplat,
        &SplatPhysicsBody,
    )>,
) {
    for (entity, mut collider, mut friction, mut restitution, mut density, splat, body) in
        query.iter_mut()
    {
        let new_rb = match body.body_type {
            SplatBodyType::Static => RigidBody::Static,
            SplatBodyType::Dynamic => RigidBody::Dynamic,
            SplatBodyType::Kinematic => RigidBody::Kinematic,
            SplatBodyType::Sensor => RigidBody::Kinematic,
        };
        commands.entity(entity).insert(new_rb);

        let target_radius: f32 = splat.get_world_radius();
        *collider = Collider::sphere(target_radius);

        let target_density = splat.solidity * body.density;
        *density = ColliderDensity(target_density);

        let target_friction = (1.0 - splat.texture_weight) * body.friction;
        *friction = Friction::new(target_friction);

        *restitution = Restitution::new(body.restitution * splat.stability);
    }
}

fn handle_splat_physics_interactions(
    mut collision_events: MessageReader<CollisionStart>,
    mut collision_ended_events: MessageReader<CollisionEnd>,
    mut splat_query: Query<&mut GaussianSplat>,
    mut event_writer: MessageWriter<SplatPhysicsEvent>,
) {
    for event in collision_events.read() {
        let entity1 = event.collider1;
        let entity2 = event.collider2;
        let impact_force = 1.0;

        if let Ok(mut splat1) = splat_query.get_mut(entity1) {
            splat1.solidity = (splat1.solidity * 1.05).min(1.0);
            splat1.on_collision();
        }

        if let Ok(mut splat2) = splat_query.get_mut(entity2) {
            splat2.solidity = (splat2.solidity * 1.05).min(1.0);
            splat2.on_collision();
        }

        event_writer.write(SplatPhysicsEvent {
            entity: entity1,
            other_entity: entity2,
            impact_velocity: Vec3::ZERO,
            impact_force,
            event_type: SplatPhysicsEventType::CollisionStarted,
        });
    }

    for event in collision_ended_events.read() {
        let entity1 = event.collider1;
        let entity2 = event.collider2;

        if let Ok(mut splat1) = splat_query.get_mut(entity1) {
            splat1.on_collision_end();
        }

        if let Ok(mut splat2) = splat_query.get_mut(entity2) {
            splat2.on_collision_end();
        }

        event_writer.write(SplatPhysicsEvent {
            entity: entity1,
            other_entity: entity2,
            impact_velocity: Vec3::ZERO,
            impact_force: 0.0,
            event_type: SplatPhysicsEventType::CollisionEnded,
        });
    }
}

fn update_splat_physics_materials(
    mut splat_query: Query<
        (Entity, &mut GaussianSplat, &mut Friction, &mut Restitution),
        Changed<GaussianSplat>,
    >,
    body_query: Query<&SplatPhysicsBody>,
) {
    for (entity, _splat, mut friction, mut restitution) in splat_query.iter_mut() {
        if let Ok(body) = body_query.get(entity) {
            let target_friction = (1.0 - _splat.texture_weight) * body.friction;
            *friction = Friction::new(target_friction);

            *restitution = Restitution::new(body.restitution * _splat.stability);
        }
    }
}

#[allow(dead_code)]
#[derive(Message, Debug, Clone)]
pub struct SplatPhysicsEvent {
    pub entity: Entity,
    pub other_entity: Entity,
    pub impact_velocity: Vec3,
    pub impact_force: f32,
    pub event_type: SplatPhysicsEventType,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplatPhysicsEventType {
    CollisionStarted,
    CollisionEnded,
    ContactForce,
}

pub fn create_splat_ground(commands: &mut Commands, position: Vec3, size: Vec2) -> Entity {
    let builder = SplatPhysicsBuilder::static_body()
        .with_box(Vec3::new(size.x, 0.1, size.y))
        .with_friction(0.8);
    let (
        body,
        rigid_body,
        collider,
        density,
        friction,
        restitution,
        linear_damping,
        angular_damping,
        locked,
    ) = builder.build();

    commands
        .spawn((
            body,
            rigid_body,
            collider,
            density,
            friction,
            restitution,
            linear_damping,
            angular_damping,
            locked,
            Transform::from_translation(position),
            GlobalTransform::from(Transform::from_translation(position)),
            Name::new("SplatGround"),
        ))
        .id()
}

pub fn create_splat_wall(commands: &mut Commands, position: Vec3, size: Vec3) -> Entity {
    let builder = SplatPhysicsBuilder::static_body()
        .with_box(size)
        .with_friction(0.5);
    let (
        body,
        rigid_body,
        collider,
        density,
        friction,
        restitution,
        linear_damping,
        angular_damping,
        locked,
    ) = builder.build();

    commands
        .spawn((
            body,
            rigid_body,
            collider,
            density,
            friction,
            restitution,
            linear_damping,
            angular_damping,
            locked,
            Transform::from_translation(position),
            GlobalTransform::from(Transform::from_translation(position)),
            Name::new("SplatWall"),
        ))
        .id()
}

pub fn create_splat_obstacle(commands: &mut Commands, position: Vec3, radius: f32) -> Entity {
    let builder = SplatPhysicsBuilder::dynamic_body()
        .with_sphere(radius)
        .with_density(500.0);
    let (
        body,
        rigid_body,
        collider,
        density,
        friction,
        restitution,
        linear_damping,
        angular_damping,
        locked,
    ) = builder.build();

    commands
        .spawn((
            body,
            rigid_body,
            collider,
            density,
            friction,
            restitution,
            linear_damping,
            angular_damping,
            locked,
            Transform::from_translation(position),
            GlobalTransform::from(Transform::from_translation(position)),
            Name::new("SplatObstacle"),
        ))
        .id()
}
