use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts};

pub fn font_plugin(app: &mut App) {
    app.init_asset::<EguiFont>();
    app.init_asset_loader::<EguiFontLoader>();
    app.add_systems(Startup, load_font_assets);
    app.add_systems(Update, load_font_to_egui);
}

#[derive(Asset, TypePath)]
struct EguiFont {
    data: egui::FontData,
}

#[derive(Default)]
struct EguiFontLoader;

impl AssetLoader for EguiFontLoader {
    type Asset = EguiFont;
    type Error = std::io::Error;
    type Settings = ();
    async fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        info!("Loading Fonts...");
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(EguiFont {
            data: egui::FontData::from_owned(bytes),
        })
    }
}

#[derive(Resource)]
struct FontHandles {
    fonts: Vec<Handle<EguiFont>>,
}

fn load_font_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fonts: Vec<Handle<EguiFont>> = ["fonts/hei.TTF"]
        .iter()
        .map(|path| asset_server.load(*path))
        .collect();
    commands.insert_resource(FontHandles { fonts });
}

fn load_font_to_egui(
    handles: Res<FontHandles>,
    mut fonts: ResMut<Assets<EguiFont>>,
    mut loaded: Local<bool>,
    mut contexts: EguiContexts,
) {
    // Only do this once
    if *loaded {
        return;
    }
    // Wait until the scene is loaded
    let Some(_) = fonts.get(&handles.fonts[0]) else {
        return;
    };
    *loaded = true;

    let mut font_defs = egui::FontDefinitions::default();
    for i in 0..handles.fonts.len() {
        let handle = &handles.fonts[i];
        let font = fonts.get(handle).unwrap();
        font_defs.font_data.insert(i.to_string(), font.data.clone());
        font_defs
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .push(i.to_string());
        fonts.remove(handle);
    }

    contexts.ctx_mut().set_fonts(font_defs);
}
