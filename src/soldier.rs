use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

use crate::ui;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HookPlugin);
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, CustomMaterial>,
        >::default());
        app.add_systems(Startup, spawn);
        app.add_systems(Update, (customize_materials, customize_color));
    }
}

#[derive(Component)]
struct CustomizeMaterial {
    original_material: Handle<StandardMaterial>,
    mask_handle: Handle<Image>,
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
struct CustomMaterial {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[texture(100)]
    #[sampler(101)]
    pub mask: Option<Handle<Image>>,

    #[uniform(102)]
    pub custom_color: Color,
}

impl MaterialExtension for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}

#[allow(clippy::needless_pass_by_value)]
fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    const SCENE_TO_LOAD: &str = "character-soldier.glb#Scene0";
    const MATERIAL_TO_CUSTOMIZE: &str = "character-soldier.glb#Material0";

    let mask_handle: Handle<Image> = asset_server.load("Textures/mask.png");

    commands.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load(SCENE_TO_LOAD),
            ..default()
        },
        hook: SceneHook::new(move |entity, commands| {
            if let Some(handle) = entity.get::<Handle<StandardMaterial>>() {
                if let Some(path) = handle.path() {
                    if path.to_string() == MATERIAL_TO_CUSTOMIZE {
                        commands
                            .insert(CustomizeMaterial {
                                original_material: handle.clone(),
                                mask_handle: mask_handle.clone(),
                            })
                            .remove::<Handle<StandardMaterial>>();
                    }
                }
            }
        }),
    });
}

#[allow(clippy::needless_pass_by_value)]
fn customize_materials(
    mut commands: Commands,
    entities_to_customize: Query<(Entity, &CustomizeMaterial)>,
    standard_materials: Res<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, CustomMaterial>>>,
) {
    for (entity, customize_material) in entities_to_customize.iter() {
        let original_handle = &customize_material.original_material;
        let mask_handle = &customize_material.mask_handle;

        if let Some(original) = standard_materials.get(original_handle) {
            commands
                .entity(entity)
                .insert(custom_materials.add(ExtendedMaterial {
                    base: original.clone(),
                    extension: CustomMaterial {
                        mask: Some(mask_handle.clone()),
                        custom_color: Color::BLUE,
                    },
                }))
                .remove::<CustomizeMaterial>();
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn customize_color(
    ui_state: Res<ui::State>,
    mut custom_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, CustomMaterial>>>,
) {
    for (_, mat) in custom_materials.iter_mut() {
        let c = ui_state.color;
        mat.extension.custom_color = Color::rgba_u8(c.r(), c.g(), c.b(), c.a());
    }
}
