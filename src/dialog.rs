use imagesize::size as getsize;
use macroquad::input::*;
use macroquad::prelude::*;
use macroquad::time::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Skin, Ui,
};
use miniquad::window::blocking_event_loop;
use serde::Deserialize;
use serde_json::Result;
use std::collections::HashMap;
use std::default;
use std::ffi::c_int;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;

use crate::ConfigDetails;
use crate::Configs;
#[derive(Deserialize, Debug)]
struct Part {
    id: i32,
    character: String,
    text: String,
    posx: i32,
    posy: i32,
    // test: i32,
    ts_in: i32,
    ts_out: i32,
}

#[derive(Deserialize, Debug)]
struct Dialog {
    dialog: Vec<Part>,
}

async fn get_pos(x: i32, y: i32) -> Vec2 {
    let w = screen_width();
    let h = screen_height();
    let char_w: f32 = w * (20.0 / 100.0);
    vec2(
        match x {
            -1 => w * 25.0 / 100.0,
            0 => w * 50.0 / 100.0,
            1 => w * 75.0 / 100.0,
            _ => w * 50.0 / 100.0,
        } - (char_w / 2.0),
        match y {
            0 => h * 40.0 / 100.0,
            1 => h * 30.0 / 100.0,
            2 => h * 20.0 / 100.0,
            _ => h * 30.0 / 100.0,
        },
    )
}

async fn get_textureparams(img: &str) -> DrawTextureParams {
    let sizevec = match getsize(img) {
        Ok(dim) => vec2(dim.width as f32, dim.height as f32),
        Err(why) => vec2(0.0, 0.0),
    };

    let char_w: f32 = screen_width() * (20.0 / 100.0);
    DrawTextureParams {
        dest_size: Some(vec2(char_w, char_w * (sizevec.y / sizevec.x))),
        ..Default::default()
    }
}

async fn load_parts<'a>(
    dialog: &Dialog,
    dialogindex: &mut i32,
    txtinp1: &'a mut String,
    mut temp1: &'a mut String,
    txtrend1: &mut String,
    txtrend2: &mut String,
    title1: &mut String,
    active_char_x: &mut i32,
    active_char_y: &mut i32,
    active_char_id: &mut i32,
    active_char: &mut String,
    active_part: &mut String,
    is_dialog_end: &mut bool,
) {
    if let Some(dialog_part) = dialog.dialog.get(*dialogindex as usize) {
        println!("{:?}", dialog_part);
        *active_char_x = dialog_part.posx;
        *active_char_y = dialog_part.posy;
        *active_char_id = dialog_part.id;
        *active_char = dialog_part.character.clone();
        *active_part = dialog_part.text.clone();

        *is_dialog_end = false;

        *title1 = active_char.clone();
        *txtrend2 = active_part.clone();
    } else {
        info!("Index out of bounds: {}", dialogindex);

        *active_char_x = 0;
        *active_char_y = 0;
        *active_char_id = 0;
        *active_char = String::from("None_0");
        *active_part = String::from(".");

        *is_dialog_end = true;
        *dialogindex = -1;

        *title1 = active_char.clone();
        *txtrend2 = active_part.clone();
    }
}

pub fn prev_dialog(dialogindex: &mut i32) {
    *dialogindex += -1;
}

async fn next_dialog(
    dialogindex: &mut i32,
    dialog: &Dialog,
    alive_char_map: &mut HashMap<i32, bool>,
    char_map_pos: &mut HashMap<i32, Vec2>,
) {
    // *alive_char_map.insert(, )

    if let Some(dialog_part) = dialog.dialog.get(*dialogindex as usize) {
        char_map_pos.insert(dialog_part.id, vec2(dialog_part.posx as f32, dialog_part.posy as f32));

        if dialog_part.ts_in < 0 {
            alive_char_map.remove(&dialog_part.id);
        }
        if dialog_part.ts_in > 0 {
            alive_char_map.insert(dialog_part.id, true);
        }
        if dialog_part.ts_in == 0 {
            alive_char_map.insert(dialog_part.id, false);
        }
    }

    *dialogindex += 1;

    //     if let Some(dialog_part) = dialog.dialog.get(*dialogindex as usize) {
    //         println!("{:?}", dialog_part);
    //         *active_char_x = dialog_part.posx;
    //         *active_char_y = dialog_part.posy;
    //         *active_char_id = dialog_part.id;
    //         *active_char = dialog_part.character.clone();
    //         *active_part = dialog_part.text.clone();
    //
    //         *is_dialog_end = false;
    //
    //         *title1 = active_char.clone();
    //         *txtrend2 = active_part.clone();
    //     } else {
    //         info!("Index out of bounds: {}", dialogindex);
    //
    //         *active_char_x = 0;
    //         *active_char_y = 0;
    //         *active_char_id = 0;
    //         *active_char = String::from("None_0");
    //         *active_part = String::from(".");
    //
    //         *is_dialog_end = true;
    //         *dialogindex = -1;
    //
    //         *title1 = active_char.clone();
    //         *txtrend2 = active_part.clone();
    //     }
}

async fn load_file(dialog: &mut Dialog, asts_dir: &str) {
    let mut file = File::open(format!("{asts_dir}Dialogues/dialog.json").as_str())
        .expect(format!("2{asts_dir}Dialogues/dialog.json").as_str());
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("2CAN'T READ FILE BRUHH??");
    *dialog = serde_json::from_str(&contents).expect("2CAN'T PARSE FILE DUDE!!??");
}

async fn load_font_map<'a>(
    fontmap: &'a HashMap<String, String>,
    asts_dir: &'a str,
) -> HashMap<&'a str, Font> {
    let mut font_map: HashMap<&str, Font> = HashMap::new();

    for (k, v) in fontmap {
        font_map.insert(k.as_str(), {
            info!("{:?}", &v);
            load_ttf_font(format!("{asts_dir}/Fonts/{v}").as_str())
                .await
                .expect("load_font_map V failed")
        });
    }

    font_map
}

pub async fn run_dialog(configs: &ConfigDetails) {
    // let mut file = File::open("/assets/Dialogues/dialog.json").expect("1CAN'T OPEN FILE YO!!??");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("1CAN'T READ FILE BRUHH??");
    // let dialog: Dialog = serde_json::from_str(&contents).expect("1CAN'T PARSE FILE DUDE!!??");
    //
    // for part in &dialog.dialog {
    //     println!(
    //         "ID : {} \nCharacter: {} \nText: {}",
    //         part.id, part.character, part.text
    //     );
    // }

    let asts_dir: &str = configs.asset_dir.as_str();
    let mut dialogshow = true;
    let mut is_dialog_end = false;
    let mut title_new_pos: Vec2 = Vec2::new(0.0, 0.0);
    let mut slidevar1 = 0.0;
    let mut txtinp1 = String::new();
    let mut temp1 = String::new();
    let mut boxsize: Vec2 = Vec2::new(10., 150.);
    let mut txtrend1 = String::new();
    let mut txtrend2 = String::new();
    let mut title1 = String::new();
    let mut dialogindex: i32 = 0;
    let mut mos_in_dialog: bool = false;
    let mut dialog: Dialog = Dialog {
        dialog: vec![Part {
            id: 0,
            character: String::from("Error_1"),
            text: String::from("Error_1"),
            posx: 0,
            posy: 0,
            // test: 0,
            ts_in: 0,
            ts_out: 0,
        }],
    };

    let mut alive_char_map: HashMap<i32, bool> = HashMap::new();
    let mut char_map_pos: HashMap<i32, Vec2> = HashMap::new();
    let ts_pos_change_y: f32 = 0.0;
    let inactive_opacity: f32 = 0.70;

    //Load fonts into HashMap , GOSH I LOVE HASHMAPS MWAH MWAH!!
    let font_map: HashMap<&str, Font> = load_font_map(&configs.fontmap, &asts_dir).await;

    let wheel_prev: f32 = 0.0;
    let wheel_next: f32 = 0.0;

    //Files I think
    //Add HashMap<Name from path , path.png>
    let mut imgpaths: HashMap<String, String> = HashMap::new();

    let mut ct: HashMap<&str, Texture2D> = HashMap::new();

    let path = ".";
    let entries = std::fs::read_dir(format!("{asts_dir}Characters").as_str()).unwrap();
    for entry in entries {
        match entry {
            Ok(entry) => {
                let mut filename1 = match entry.file_name().into_string() {
                    Ok(k) => k,
                    Err(e) => panic!("{:?}", e),
                };

                filename1 = filename1.trim_end_matches(".png").to_string();
                // println!("Processing entry: {}", filename1.trim_end_matches(".png"));
                imgpaths.insert(
                    filename1.clone().to_uppercase(),
                    format!("{asts_dir}Characters/{}.png", filename1),
                );

                ct.insert(
                    filename1.clone().to_uppercase().as_str(),
                    load_texture(format!("{asts_dir}Characters/{}.png", filename1).as_str())
                        .await
                        .unwrap(),
                );

                println!("{:?}", imgpaths);
            }
            Err(e) => {
                println!("  entry error: {:?}", e);
            }
        };
    }
    let mut c_id : HashMap<i32,&str> = HashMap::new();
    c_id.insert(1, "CHARLOTTE");
    c_id.insert(2, "FERRIS");


    // do scale HERE

    let mut active_char_id: i32 = 0;
    let mut active_char: String = String::new();
    let mut active_part: String = String::new();
    let mut active_char_x: i32 = 0;
    let mut active_char_y: i32 = 0;

    let font_1 = font_map.get("ft5").unwrap();

    let title_skin1 = {
        let label_style = root_ui()
            .style_builder()
            .with_font(&font_1)
            .expect("2 FONT LOAD FAIL")
            .margin(RectOffset::new(10.0, 0.0, 0.0, 0.0))
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(30)
            .build();

        Skin {
            label_style,
            ..root_ui().default_skin()
        }
    };

    let skin1 = {
        let label_style = root_ui()
            .style_builder()
            .with_font(&font_1)
            .expect("1 FONT LOAD FAIL")
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(30)
            .build();

        let button_style = root_ui()
            .style_builder()
            // .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        Skin {
            label_style,
            button_style,
            ..root_ui().default_skin()
        }
    };

    let mut win_skin = skin1.clone();
    let mut char_title_skin = title_skin1.clone();
    load_file(&mut dialog, &asts_dir).await;

    loop {
        clear_background(WHITE);
        //
        if is_key_pressed(KeyCode::Space) {
            info!("1");
            dialogshow = !dialogshow;
            info!("{:?}", dialogshow);
        }
        if dialogshow {
            if !temp1.is_empty() {
                let mut tail = String::new();
                tail = temp1.split_off(1);
                txtrend1.push(temp1.chars().next().unwrap());
                temp1 = tail.clone();
            }

            let mut dialogboxpos: Vec2 = vec2(
                screen_width() * (boxsize.x / 100.0),
                screen_height() - (screen_height() * (boxsize.x / 100.)) - boxsize.y,
            );
            let mut dialogboxsize: Vec2 = vec2(
                screen_width() - 2.0 * (screen_width() * (boxsize.x / 100.)),
                boxsize.y,
            );

            info!("{:?}", mouse_position());

            let boxposrange_x = dialogboxpos.x..(dialogboxpos.x + dialogboxsize.x);
            let boxposrange_y = dialogboxpos.y..(dialogboxpos.y + boxsize.y);

            if boxposrange_x.contains(&mouse_position().0) {
                info!("X");
            }
            if boxposrange_y.contains(&mouse_position().1) {
                info!("Y");
            }

            mos_in_dialog = if boxposrange_x.contains(&mouse_position().0)
                && boxposrange_y.contains(&mouse_position().1)
            {
                true
            } else {
                false
            };
            info!("{}", mos_in_dialog);

            title_new_pos = vec2((active_char_x + 1) as f32 * dialogboxsize.x / 3.0, 0.0);
            root_ui().pop_skin();
            if active_char_id != 0 {
                //Name Title
                root_ui().push_skin(&char_title_skin);

                widgets::Window::new(
                    hash!(),
                    dialogboxpos - vec2(-title_new_pos.x, dialogboxsize.y / 4.0),
                    vec2(dialogboxsize.x / 3.0, dialogboxsize.y / 4.0),
                )
                .titlebar(false)
                .movable(false)
                .ui(&mut *root_ui(), |ui_1| {
                    ui_1.label(None, &active_char);
                });

                root_ui().pop_skin();
            }

            // MAIN DIALOGUE LABEL HERE!!
            root_ui().push_skin(&win_skin);
            widgets::Window::new(hash!(), dialogboxpos, dialogboxsize)
                .label(title1.clone().as_str())
                .titlebar(false)
                .movable(false)
                .close_button(true)
                .ui(&mut *root_ui(), |ui_0| {
                    // ui_0.window(hash!(), dialogboxpos - vec2( title_new_pos.x , dialogboxsize.y /4.0 ), Vec2::new(450., 200.),|ui_1|{});

                    ui_0.label(None, &txtrend2);
                });

            root_ui().pop_skin();
        }
        // UI done.

        if mos_in_dialog && is_mouse_button_pressed(MouseButton::Left) {
            // next_dialog(&mut dialogindex);
            info!("next!");
            load_parts(
                &dialog,
                &mut dialogindex,
                &mut txtinp1,
                &mut temp1,
                &mut txtrend1,
                &mut txtrend2,
                &mut title1,
                &mut active_char_x,
                &mut active_char_y,
                &mut active_char_id,
                &mut active_char,
                &mut active_part,
                &mut is_dialog_end,
            )
            .await;
            // dialogindex += 1;
            next_dialog(
                &mut dialogindex,
                &dialog,
                &mut alive_char_map,
                &mut char_map_pos,
            );
        }
        if mos_in_dialog && (is_mouse_button_pressed(MouseButton::Right) || mouse_wheel().1 > 0.0) {
            // next_dialog(&mut dialogindex);
            info!("next!");
            load_parts(
                &dialog,
                &mut dialogindex,
                &mut txtinp1,
                &mut temp1,
                &mut txtrend1,
                &mut txtrend2,
                &mut title1,
                &mut active_char_x,
                &mut active_char_y,
                &mut active_char_id,
                &mut active_char,
                &mut active_part,
                &mut is_dialog_end,
            )
            .await;
            // dialogindex += 1;
            prev_dialog(&mut dialogindex);
        }

        //spawing
        for (k, v) in alive_char_map.iter_mut() {
            let chosen_pos: Vec2 = *char_map_pos.get(k).unwrap();
            let mut alivepos: Vec2 = get_pos(chosen_pos.x as i32, chosen_pos.y as i32).await;
            let opacity: f32 = if *v { 1.0 } else { inactive_opacity };
            draw_texture_ex(
                &ct.get(c_id.get(k).unwrap()).unwrap(),
                alivepos.x,
                alivepos.y + ts_pos_change_y,
                Color::new(1.0, 1.0, 1.0, opacity),
                get_textureparams(imgpaths.get(c_id.get(k).unwrap().as_str()).expect("Failed to Draw")).await,
            );

            match k {
                1 => {}

                _ => info!("None!?!"),
            }

            // match active_char_id {
            //     1 => {
            //         ({
            //             draw_texture_ex(
            //                 &c1,
            //                 activepos.x,
            //                 activepos.y + ts_pos_change_y,
            //                 Color::new(1.0, 1.0, 1.0, opacity),
            //                 get_textureparams(imgpaths.get("CHARLOTTE").expect("Failed to Draw"))
            //                     .await,
            //             )
            //         })
            //     }
            //
            //     2 => draw_texture_ex(
            //         &c2,
            //         activepos.x,
            //         activepos.y,
            //         Color::new(1.0, 1.0, 1.0, opacity),
            //         get_textureparams(imgpaths.get("FERRIS").expect("Failed to Draw")).await,
            //     ),
            //
            //     _ => info!("None??!"),
            // }
        }

        next_frame().await;
    }
}
