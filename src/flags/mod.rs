use std::collections::BTreeMap;
use eframe::egui;
use eframe::egui::TextureOptions;
use eframe::epaint::TextureHandle;
use egui_extras::image::FitTo;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Country {
    Australia,
    Brazil,
    UnitedStates
}

impl Country {
    fn flag(&self) -> Flag {
        match self {
            Country::Australia => Flag {
                code: "AU".to_owned(),
                name: "Australia".to_owned(),
                country: Country::Australia,
                bytes: include_bytes!("../../assets/flags/au.svg"),
                handle: None
            },
            Country::Brazil => Flag {
                code: "BR".to_owned(),
                name: "Brazil".to_owned(),
                country: Country::Brazil,
                bytes: include_bytes!("../../assets/flags/br.svg"),
                handle: None
            },
            Country::UnitedStates => Flag {
                code: "US".to_owned(),
                name: "United States".to_owned(),
                country: Country::UnitedStates,
                bytes: include_bytes!("../../assets/flags/us.svg"),
                handle: None
            }
        }
    }
}

pub struct Flag {
    pub code: String,
    pub name: String,
    pub country: Country,
    bytes: &'static [u8],
    pub handle: Option<TextureHandle>
}

impl Flag {
    pub fn load_textures(ctx: &egui::Context) -> BTreeMap<Country, Flag>{
        let mut map = BTreeMap::new();
        for country in Country::iter() {
            let mut flag = country.flag();

            let image = egui_extras::image::load_svg_bytes_with_size(
                flag.bytes,
                FitTo::Height(360)
            );

            match image {
                Ok(color_image) => {
                    let texture = ctx.load_texture(
                        flag.code.as_str(),
                        color_image,
                        TextureOptions::default()
                    );
                    flag.handle = Some(texture);
                    map.insert(country, flag);
                }
                Err(e) => {
                    println!("Error loading SVG: {}", e);
                }
            }

        }
        map
    }
}
