use bevy::prelude::*;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Debug, Clone)]
struct RawTransform {
    trans: [f32; 3],
    rot: [f32; 4],
}

impl From<RawTransform> for Transform {
    fn from(raw: RawTransform) -> Self {
        let mut quat = Quat::from_array(raw.rot);
        if quat.length_squared() > 0.0 {
            quat = quat.normalize();
        }
        Transform {
            translation: Vec3::from_array(raw.trans),
            rotation: quat,
            scale: Vec3::ONE, // default scale
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct TransformStartEnd {
    start: RawTransform,
    end: RawTransform,
}

#[derive(Deserialize, Debug, Clone)]
struct TransformExpr {
    t_exprs: [String; 3],
    r_exprs: [String; 4],
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
enum Animator {
    StartEnd(TransformStartEnd),
    Expr(TransformExpr),
}

#[derive(Deserialize, Debug, Clone)]

struct CubePrimitive {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Deserialize, Debug, Clone)]
struct SpherePrimitive {
    radius: f32,
}

#[derive(Deserialize, Debug, Clone)]
struct TorusPrimitive {
    inner_radius: f32,
    outer_radius: f32,
}

#[derive(Deserialize, Debug, Clone)]
struct ConePrimitive {
    radius: f32,
    height: f32,
}

#[derive(Deserialize, Debug, Clone)]
struct CylinderPrimitive {
    radius: f32,
    height: f32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
enum BevyPrimitive {
    Cube(CubePrimitive),
    Sphere(SpherePrimitive),
    Torus(TorusPrimitive),
    Cone(ConePrimitive),
    Cylinder(CylinderPrimitive),
}

impl BevyPrimitive {
    fn to_mesh(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Mesh3d {
        match self {
            BevyPrimitive::Cube(c) => Mesh3d(meshes.add(Cuboid::new(c.x, c.y, c.z))),
            BevyPrimitive::Sphere(s) => Mesh3d(meshes.add(Sphere::new(s.radius))),
            BevyPrimitive::Torus(t) => {
                Mesh3d(meshes.add(Torus::new(t.inner_radius, t.outer_radius)))
            }
            BevyPrimitive::Cone(c) => Mesh3d(meshes.add(Cone::new(c.radius, c.height))),
            BevyPrimitive::Cylinder(c) => Mesh3d(meshes.add(Cylinder::new(c.radius, c.height))),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Asset {
    id: String,
    primitive: BevyPrimitive,
    animator: Animator,
    children: Vec<Asset>,
}

#[derive(Resource, Deserialize, Debug, Default, Clone)]
struct State {
    assets: Vec<Asset>,
    version: String,
}

static STATE: Lazy<Mutex<State>> = Lazy::new(|| {
    Mutex::new(State {
        assets: vec![],
        version: String::from("init"),
    })
});

#[wasm_bindgen]
pub fn update_state(state: String) -> Result<(), JsValue> {
    let new_state = serde_json::from_str::<State>(&state).map_err(|e| e.to_string())?;
    log::info!("{new_state:?}");
    *STATE.lock().unwrap() = new_state;

    Ok(())
}

fn eval_transform(
    t_exprs: &[String; 3],
    r_exprs: &[String; 4],
    t: f32,
) -> Result<Transform, JsValue> {
    let t_x_expr = t_exprs[0]
        .parse::<meval::Expr>()
        .map_err(|e| e.to_string())?;
    let t_y_expr = t_exprs[1]
        .parse::<meval::Expr>()
        .map_err(|e| e.to_string())?;
    let t_z_expr = t_exprs[2]
        .parse::<meval::Expr>()
        .map_err(|e| e.to_string())?;

    let r_x_expr = r_exprs[0]
        .parse::<meval::Expr>()
        .map_err(|e| e.to_string())?;
    let r_y_expr = r_exprs[1]
        .parse::<meval::Expr>()
        .map_err(|e| e.to_string())?;
    let r_z_expr = r_exprs[2]
        .parse::<meval::Expr>()
        .map_err(|e| e.to_string())?;
    let r_w_expr = r_exprs[3]
        .parse::<meval::Expr>()
        .map_err(|e| e.to_string())?;

    let t_x_func = t_x_expr.bind("t").map_err(|e| e.to_string())?;
    let t_y_func = t_y_expr.bind("t").map_err(|e| e.to_string())?;
    let t_z_func = t_z_expr.bind("t").map_err(|e| e.to_string())?;

    let r_x_func = r_x_expr.bind("t").map_err(|e| e.to_string())?;
    let r_y_func = r_y_expr.bind("t").map_err(|e| e.to_string())?;
    let r_z_func = r_z_expr.bind("t").map_err(|e| e.to_string())?;
    let r_w_func = r_w_expr.bind("t").map_err(|e| e.to_string())?;

    let t_x = t_x_func(t as f64) as f32;
    let t_y = t_y_func(t as f64) as f32;
    let t_z = t_z_func(t as f64) as f32;

    let r_x = r_x_func(t as f64) as f32;
    let r_y = r_y_func(t as f64) as f32;
    let r_z = r_z_func(t as f64) as f32;
    let r_w = r_w_func(t as f64) as f32;

    let transform = Transform {
        translation: Vec3::new(t_x, t_y, t_z),
        rotation: Quat::from_slice(&[r_x, r_y, r_z, r_w]),
        scale: Vec3::ONE,
    };

    Ok(transform)
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");

    log::info!("Hello from Bevy WASM!");
    App::new()
        .insert_resource(State::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sync_state)
        .add_systems(Update, animate_transforms.after(sync_state))
        .run();
}

fn sync_state(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<State>,
    mut query: Query<(Entity, &Shape)>,
) {
    let new_state = STATE.lock().unwrap();
    if new_state.version != state.version {
        log::info!("State update!!");
        *state = new_state.clone();

        for (entity, _) in query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }

        for asset in &state.assets {
            let mesh = asset.primitive.to_mesh(&mut meshes);

            // Spawn parent and recursively add children
            commands
                .spawn((
                    mesh,
                    MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                    Transform::default(),
                    Shape {
                        id: asset.id.clone(),
                    },
                    TransformAnimation {
                        animator: asset.animator.clone(),
                        duration: 3.0,
                        elapsed: 0.0,
                    },
                ))
                .with_children(|child_builder| {
                    for child in &asset.children {
                        spawn_assets(child, child_builder, &mut meshes, &mut materials);
                    }
                });
        }
    }
}

fn spawn_assets(
    asset: &Asset,
    commands: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let mesh = asset.primitive.to_mesh(meshes);

    // Spawn parent and recursively add children
    commands
        .spawn((
            mesh,
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::default(),
            Shape {
                id: asset.id.clone(),
            },
            TransformAnimation {
                animator: asset.animator.clone(),
                duration: 3.0,
                elapsed: 0.0,
            },
        ))
        .with_children(|child_builder| {
            for child in &asset.children {
                spawn_assets(child, child_builder, meshes, materials);
            }
        });
}

#[derive(Component)]
struct Shape {
    id: String,
}

#[derive(Component)]
struct TransformAnimation {
    animator: Animator,
    duration: f32,
    elapsed: f32,
}

impl TransformAnimation {
    fn tick(&mut self, delta: f32) {
        self.elapsed = (self.elapsed + delta) % self.duration;
    }

    fn get_transform(&self) -> Transform {
        let t = (self.elapsed / self.duration).min(1.0);
        match &self.animator {
            Animator::StartEnd(anim) => {
                let start: Transform = anim.start.clone().into();
                let end: Transform = anim.end.clone().into();
                let mut transform = Transform::IDENTITY;
                transform.translation = start.translation.lerp(end.translation, t);
                transform.rotation = start.rotation.slerp(end.rotation, t);

                transform
            }
            Animator::Expr(anim) => match eval_transform(&anim.t_exprs, &anim.r_exprs, t) {
                Ok(t) => t,
                Err(e) => {
                    log::info!("Failed to transform expr: {e:?}");
                    Transform::IDENTITY
                }
            },
        }
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(2.5, 4.5, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let axis_radius = 0.025;
    let axis_length = 2.0;

    let red = Color::srgb_u8(255, 0, 0);
    let green = Color::srgb_u8(0, 255, 0);
    let blue = Color::srgb_u8(0, 0, 255);

    // X axis
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(axis_radius, axis_length))),
        MeshMaterial3d(materials.add(red)),
        Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::X * axis_length / 2.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cone::new(0.15, 0.25))),
        MeshMaterial3d(materials.add(red)),
        Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4 * 6.0))
            .with_translation(Vec3::X * axis_length),
    ));

    // Y axis
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(axis_radius, axis_length))),
        MeshMaterial3d(materials.add(green)),
        Transform::from_translation(Vec3::Y * axis_length / 2.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cone::new(0.15, 0.25))),
        MeshMaterial3d(materials.add(green)),
        Transform::from_translation(Vec3::Y * axis_length),
    ));

    // Z axis
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(axis_radius, axis_length))),
        MeshMaterial3d(materials.add(blue)),
        Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::Z * axis_length / 2.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cone::new(0.15, 0.25))),
        MeshMaterial3d(materials.add(blue)),
        Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_4 * 2.0))
            .with_translation(Vec3::Z * axis_length),
    ));
}

fn animate_transforms(
    mut query: Query<(&mut Transform, &mut TransformAnimation)>,
    time: Res<Time>,
) {
    for (mut transform, mut animation) in query.iter_mut() {
        let delta = time.delta_secs();
        animation.tick(delta);
        *transform = animation.get_transform();
    }
}
