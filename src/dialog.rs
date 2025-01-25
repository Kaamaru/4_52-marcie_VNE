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
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;

use crate::DIALOGVISIBILITY;
#[derive(Deserialize, Debug)]
struct Part {
    id: i32,
    character: String,
    text: String,
    posx: i32,
    posy: i32,
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
            1 => h * 60.0 / 100.0,
            2 => h * 80.0 / 100.0,
            _ => h * 60.0 / 100.0,
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

fn load_parts<'a>(
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
) {
    let mut file = File::open("dialog.json").expect("CAN'T OPEN FILE YO!!??");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("CAN'T READ FILE BRUHH??");

    let dialog: Dialog = serde_json::from_str(&contents).expect("CAN'T PARSE FILE DUDE!!??");

    temp1 = txtinp1;

    if let Some(dialog_part) = dialog.dialog.get(*dialogindex as usize) {
        println!("{:?}", dialog_part);
        *active_char_x = dialog_part.posx;
        *active_char_y = dialog_part.posy;
        *active_char_id = dialog_part.id;
        *active_char = dialog_part.character.clone();
        *active_part = dialog_part.text.clone();

        *title1 = active_char.clone();
        *txtrend2 = active_part.clone();
    } else {
        println!("Index out of bounds: {}", dialogindex);
        *dialogindex = 0;
    }
}

pub fn prev_dialog(dialogindex: &mut i32) {
    *dialogindex += -1;
}

pub fn next_dialog(dialogindex: &mut i32) {
    *dialogindex += 1;
}

pub async fn run_dialog() {
    let mut file = File::open("dialog.json").expect("CAN'T OPEN FILE YO!!??");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("CAN'T READ FILE BRUHH??");

    let dialog: Dialog = serde_json::from_str(&contents).expect("CAN'T PARSE FILE DUDE!!??");

    for part in &dialog.dialog {
        println!(
            "ID : {} \nCharacter: {} \nText: {}",
            part.id, part.character, part.text
        );
    }

    let mut dialogshow = true;
    let mut slidevar1 = 0.0;
    let mut txtinp1 = String::new();
    let mut temp1 = String::new();
    let mut boxsize: Vec2 = Vec2::new(10., 150.);
    let mut txtrend1 = String::new();
    let mut txtrend2 = String::new();
    let mut title1 = String::new();
    let mut dialogindex: i32 = 0;
    let mut mos_in_dialog: bool = false;

    let wheel_prev : f32= 0.0;
    let wheel_next :f32 = 0.0;


    //Files I think
    //Add HashMap<Name from path , path.png>
    let mut imgpaths: HashMap<String, String> = HashMap::new();

    let path = ".";
    let entries = std::fs::read_dir("assets/").unwrap();
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
                    format!("assets/{}.png", filename1),
                );
                println!("{:?}", imgpaths);
            }
            Err(e) => {
                println!("  entry error: {:?}", e);
            }
        };
    }

    let c1: Texture2D = load_texture("assets/Charlotte.png").await.unwrap();
    let c2: Texture2D = load_texture("assets/Ferris.png").await.unwrap();

    // do scale HERE

    let mut active_char_id: i32 = 0;
    let mut active_char: String = String::new();
    let mut active_part: String = String::new();
    let mut active_char_x: i32 = 0;
    let mut active_char_y: i32 = 0;

    let skin1 = {
        let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("../htowert.ttf"))
            .unwrap()
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

            root_ui().push_skin(&win_skin);

            mos_in_dialog = if boxposrange_x.contains(&mouse_position().0)
                && boxposrange_y.contains(&mouse_position().1)
            {
                true
            } else {
                false
            };
            info!("{}", mos_in_dialog);

            widgets::Window::new(hash!(), dialogboxpos, dialogboxsize)
                .label(title1.clone().as_str())
                .titlebar(false)
                .movable(false)
                .close_button(true)
                .ui(&mut *root_ui(), |ui| {
                    ui.label(None, &txtrend2);
                });

            root_ui().pop_skin();
        }
        // UI done.

        if mos_in_dialog && (is_mouse_button_pressed(MouseButton::Left) || mouse_wheel().1 >0.0) {
            // next_dialog(&mut dialogindex);
            info!("next!");
            load_parts(
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
            );
            // dialogindex += 1;
            next_dialog(&mut dialogindex);
        }
        if mos_in_dialog && (is_mouse_button_pressed(MouseButton::Right) || mouse_wheel().1 <0.0) {
            // next_dialog(&mut dialogindex);
            info!("next!");
            load_parts(
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
            );
            // dialogindex += 1;
            prev_dialog(&mut dialogindex);
        }

        let mut activepos: Vec2 = get_pos(active_char_x, active_char_y).await;

        match active_char_id {
            1 => draw_texture_ex(
                &c1,
                activepos.x,
                activepos.y,
                WHITE,
                get_textureparams(imgpaths.get("CHARLOTTE").expect("Failed to Draw")).await,
            ),

            2 => draw_texture_ex(
                &c2,
                activepos.x,
                activepos.y,
                WHITE,
                get_textureparams(imgpaths.get("FERRIS").expect("Failed to Draw")).await,
            ),

            _ => info!("None??!"),
        }

        next_frame().await;
    }
}
