use macroquad::input::*;
use macroquad::prelude::*;
use macroquad::time::*;
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Skin, Ui,
};
use serde::Deserialize;
use serde_json::Result;
use std::collections::HashMap;
use std::default;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;
use imagesize::size as getsize;

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

async fn tsleep(t: f32) {
    let mut a = 0.0;
    while a < t {
        a += get_frame_time();
    }
}

async fn get_pos(x: i32, y: i32) -> Vec2 {
    let w = screen_width();
    let h = screen_height();
    vec2(
        match x {
            -1 => w * 25.0 / 100.0,
            0 => w * 50.0 / 100.0,
            1 => w * 75.0 / 100.0,
            _ => w * 50.0 / 100.0,
        },
        match y {
            0 => h * 40.0 / 100.0,
            1 => h * 60.0 / 100.0,
            2 => h * 80.0 / 100.0,
            _ => h * 60.0 / 100.0,
        },
    )
}

// async fn get_textureparams  (
//     img : &str,
// )-> DrawTextureParams{

//     match getsize(img) {
//         Ok(dim) => {
//             assert_eq!(dim.width, 716);
//             assert_eq!(dim.height, 716);
//         }
//         Err(why) => println!("Error getting size: {:?}", why)
//     }

//     DrawTextureParams {
//         dest_size: Some(vec2(300.0, 300.0)),
//         ..Default::default()
//     };

// }


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
    let mut dialogindex: usize = 0;

    //Files I think
    //Add HashMap<Name from path , path.png>
    let mut paths : HashMap<usize, &str> = HashMap::new();

    let path = ".";
    let entries = std::fs::read_dir("assets/").unwrap();
        for entry in entries {
            match entry {
                Ok(entry) => {
                    let filename1 = match entry.file_name().into_string() {
                        Ok(k) => k,
                        Err(e) => panic!("{:?}",e),
                    };
                    println!("Processing entry: {}", filename1.trim_end_matches(".png")  );
                    // path.insert(,fotmat!(entry))
                }
                Err(e) => {
                    println!("  entry error: {:?}", e);
                }
            }
        }


    let C1: Texture2D = load_texture("assets/Charlotte.png").await.unwrap();
    let C2: Texture2D = load_texture("assets/Ferris.png").await.unwrap();

    // do scale HERE
    
    let mut active_char_id: i32 = 0;
    let mut active_char: String = String::new();
    let mut active_part: String = String::new();
    let mut active_char_x: i32 = 0;
    let mut active_char_y: i32 = 0;

    let skin = {
        let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("../htowert.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(30)
            .build();

        Skin {
            label_style,
            ..root_ui().default_skin()
        }
    };

    let mut win_skin = skin.clone();

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

            root_ui().push_skin(&win_skin);
            widgets::Window::new(
                hash!(),
                vec2(
                    screen_width() * (boxsize.x / 100.0),
                    screen_height() - (screen_height() * (boxsize.x / 100.)) - boxsize.y,
                ),
                vec2(
                    screen_width() - 2.0 * (screen_width() * (boxsize.x / 100.)),
                    boxsize.y,
                ),
            )
            .label(title1.clone().as_str())
            .titlebar(false)
            .movable(false)
            .close_button(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, &txtrend2);
            });

            root_ui().pop_skin();
        }

        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(300.0, 300.0))
            .label("Dia input")
            .movable(true)
            .ui(&mut *root_ui(), |ui| {
                ui.label(None, "Insert Dialoge");
                ui.editbox(hash!(), vec2(250., 150.), &mut txtinp1);

                if ui.button(None, "Send") {
                    temp1 = txtinp1.clone();
                    txtrend1.clear();

                    if let Some(dialog_part) = dialog.dialog.get(dialogindex) {
                        println!("{:?}", dialog_part);
                        active_char_x = dialog_part.posx.clone();
                        active_char_y = dialog_part.posy.clone();
                        active_char_id = dialog_part.id.clone();
                        active_char = dialog_part.character.clone();
                        active_part = dialog_part.text.clone();

                        title1 = active_char.clone();
                        txtrend2 = active_part.clone();

                        dialogindex += 1;
                    } else {
                        println!("Index out of bounds: {}", dialogindex);
                        dialogindex = 0;
                    }
                }
            });
        // UI done.

        //int

        let mut activepos: Vec2 = get_pos(active_char_x, active_char_y).await;

        // match active_char_id {
        //     // 1 => draw_texture_ex(&C1, activepos.x, activepos.y, WHITE, get_textureparams()),
        //     // 2 => draw_texture_ex(&C2, activepos.x, activepos.y, WHITE, get_textureparams()),

        //     _ => info!("None??!"),
        // }

        next_frame().await;
    }
}
