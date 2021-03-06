use bevy::{
    prelude::*,
    render::camera::{ActiveCameras, CameraPlugin},
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[macro_export]
macro_rules! next_enum {
    ($l:ident, $k:expr) => {
        $l::iter()
            .enumerate()
            .nth(
                $l::iter()
                    .enumerate()
                    .find(|a| a.1 == *$k.current())
                    .map(|(i, _)| {
                        if i + 1 > $l::iter().count() - 1 {
                            0usize
                        } else {
                            i + 1
                        }
                    })
                    .unwrap(),
            )
            .unwrap()
            .1
    };
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_state(CameraState::Camera2)
        .add_startup_system(setup)
        .add_system(switch_cameras)
        .add_system(switch_cam_state)
        .add_system(input)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut active_cameras: ResMut<ActiveCameras>,
    cam_state: Res<State<CameraState>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraA(CameraState::Camera1));

    //Defaults to the latest added camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 5.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraA(CameraState::Camera2));

    for em in CameraState::iter() {
        if cam_state.current().ne(&em) {
            active_cameras.add(format!("{:?}", em).as_str());
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, EnumIter)]
enum CameraState {
    Camera1,
    Camera2,
}

#[derive(Component)]
struct CameraA(CameraState);

fn switch_cam_state(mut cam_state: ResMut<State<CameraState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        let n_state = next_enum!(CameraState, cam_state);
        let _ = cam_state.set(n_state);
    }
}

fn switch_cameras(
    mut active_cameras: ResMut<ActiveCameras>,
    cam_state: Res<State<CameraState>>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut CameraA, &mut Camera)>,
) {
    if keys.just_pressed(KeyCode::C) {
        if let Some(ac) = active_cameras.get(CameraPlugin::CAMERA_3D) {
            if let Some(o) = ac.entity {
                if let Ok((a, mut b)) = query.get_mut(o) {
                    b.name = Some(format!("{:?}", a.0));
                    let s = &cam_state.current();
                    if a.0.ne(s) {
                        if let Some((_, mut d)) = query
                            .iter_mut()
                            .filter(|(_, b)| b.name.is_some())
                            .filter(|(_, b)| {
                                b.name
                                    .as_ref()
                                    .unwrap()
                                    .eq(&CameraPlugin::CAMERA_3D.to_string())
                            })
                            .last()
                        {
                            active_cameras.remove(CameraPlugin::CAMERA_3D);

                            d.name = Some(CameraPlugin::CAMERA_3D.to_string());

                            active_cameras.add(CameraPlugin::CAMERA_3D);
                        }
                        if let Some((_, mut d)) = query.iter_mut().filter(|(a, _)| a.0.eq(s)).last()
                        {
                            d.name = Some(CameraPlugin::CAMERA_3D.to_string());
                        }
                    }
                }
            }
        }
    }
}

fn input(
    active_cameras: Res<ActiveCameras>,
    keys: Res<Input<KeyCode>>,
    query: Query<(&CameraA, &Camera)>,
) {
    if keys.just_pressed(KeyCode::C) {
        println!();
        for active_cam in active_cameras.iter() {
            print!("{:?}:", active_cam.name);
            if let Some(b) = active_cam.entity {
                if let Ok(d) = query.get(b) {
                    println!("{:?}", d.0 .0);
                }
            } else {
                println!();
            }
        }
    }
}
