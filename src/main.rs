use clap::{App, Arg};
use image::gif::GifDecoder;
use image::AnimationDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::json;
use std::fs;
use std::fs::create_dir_all;
use std::io::Write;
use std::path::Path;
use uuid::Uuid;

fn main() {
    println!(
        "{}",
        r#"    ___               _
   /   | ____ ___  __(_)   _____  _____
  / /| |/ __ `/ / / / / | / / _ \/ ___/
 / ___ / /_/ / /_/ / /| |/ /  __/ /
/_/  |_\__, /\__,_/_/ |___/\___/_/
         /_/
"#
    );
    let matches = App::new("Aquiver")
        .version("1.0")
        .author("CAIMEO")
        .about("Playing video on Minecraft Bedrock!")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .takes_value(true)
                .help("The path of the video(GIF)"),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .takes_value(true)
                .help("Resource pack's name(String)"),
        )
        .arg(
            Arg::with_name("description")
                .short("d")
                .long("description")
                .takes_value(true)
                .help("Descriptions"),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .help("The video's width (float)"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true)
                .help("The video's height (float)"),
        )
        .get_matches();

    if let Some(path) = matches.value_of("path") {
        if let Some(name) = matches.value_of("name") {
            let description = matches
                .value_of("description")
                .unwrap_or("Powered by Aquiver.");
            let height = matches
                .value_of("height")
                .unwrap_or("1")
                .parse::<f32>()
                .unwrap_or(1.0);
            let width = matches
                .value_of("width")
                .unwrap_or("2")
                .parse::<f32>()
                .unwrap_or(2.0);
            create_dir_all(Path::new(&format!("{}/beh/functions/frames", name)));
            create_dir_all(Path::new(&format!("{}/res/textures/frames", name)));
            create_dir_all(Path::new(&format!("{}/res/particles/frames", name)));
            let manifest_res = json!({
                "format_version": 1,
                "header": {
                    "description": description,
                    "name": name,
                    "uuid": Uuid::new_v4().to_hyphenated().to_string(),
                    "version": [1, 0, 0]
                },
                "modules": [{
                    "description": description,
                    "type": "resources",
                    "uuid": Uuid::new_v4().to_hyphenated().to_string(),
                    "version": [1, 0, 0]
                }]
            });
            let manifest_dat = json!({
                "format_version": 1,
                "header": {
                    "description": description,
                    "name": name,
                    "uuid": Uuid::new_v4().to_hyphenated().to_string(),
                    "version": [1, 0, 0]
                },
                "modules": [{
                    "description": description,
                    "type": "data",
                    "uuid": Uuid::new_v4().to_hyphenated().to_string(),
                    "version": [1, 0, 0]
                }]
            });
            let mut res =
                fs::File::create(Path::new(&format!("{}/res/manifest.json", name))).unwrap();
            let mut dat =
                fs::File::create(Path::new(&format!("{}/beh/manifest.json", name))).unwrap();
            res.write_all(manifest_res.to_string().as_ref());
            dat.write_all(manifest_dat.to_string().as_ref());
            let video = fs::File::open(&Path::new(path));
            let init = vec![
                format!("scoreboard objectives remove {}", name),
                format!("scoreboard objectives add {} dummy {}", name, name),
                format!("scoreboard players add @p {} 0", name),
            ];
            let mut looping: Vec<String> = vec![];
            match video {
                Ok(img) => {
                    print!("Loading GIF Decoder\n");
                    let decoder =
                        GifDecoder::new(img).unwrap_or_else(|_| panic!("Unable to create Decoder"));
                    print!("Converting image into frames\n");
                    let frames = decoder.into_frames();
                    let frames = frames.collect_frames().expect("Error decoding image");
                    println!("Image loaded. Frames: {}", frames.len());
                    let bar = ProgressBar::new(frames.len() as u64);
                    bar.set_style(
                        ProgressStyle::default_bar()
                            .template(
                                "[{percent}%] [{bar:40.cyan/blue}] {pos:>7}/{len:7} Eta: {eta}",
                            )
                            .progress_chars("++="),
                    );
                    for (i, f) in frames.iter().enumerate() {
                        looping.push(format!("execute @a[scores={{{s}={t}}}] ~ ~ ~ execute @e[type=armor_stand,name={s}] ~ ~ ~ particle {s}:img_{t} ~ ~ ~", s = name, t = i));
                        let buf = &f.to_owned().into_buffer();
                        if let Err(e) = buf.save(&Path::new(&format!(
                            "{}/res/textures/frames/img_{}.png",
                            name, i
                        ))) {
                            println!(
                                "{} {}",
                                e,
                                format!("{}/res/textures/frames/img_{}.png", name, i)
                            );
                        }
                        let mut file = fs::File::create(&format!(
                            "{}/res/particles/frames/img_{}.json",
                            name, i
                        ))
                        .unwrap();
                        let particle = json!({
                            "format_version":"1.10.0",
                            "particle_effect":{
                                "description":{
                                    "identifier":format!("{}:img_{}",name,i),
                                    "basic_render_parameters":{
                                        "material": "particles_alpha",
                                        "texture":format!("textures/frames/img_{}.png", i)
                                    }
                                },
                                "components": {
                                    "minecraft:emitter_rate_instant": {
                                        "num_particles": 1
                                    },
                                    "minecraft:emitter_lifetime_once": {
                                        "active_time": 0.05
                                    },
                                    "minecraft:emitter_shape_point": {
                                        "offset":[0,0,0],
                                        "direction":[1,0,0]
                                    },
                                    "minecraft:particle_lifetime_expression": {
                                        "max_lifetime": 0.12
                                    },
                                    "minecraft:particle_appearance_billboard":{
                                        "face_camera_mode":"lookat_xyz",
                                        "size":[width, height]
                                    }
                                }
                            }
                        });
                        file.write_all(particle.to_string().as_ref());
                        bar.inc(1);
                    }
                    looping.push(format!(
                        "execute @p[scores={{{n}=..{t}}}] ~ ~ ~ scoreboard players add @s {n} 1",
                        n = name,
                        t = frames.len()
                    ));
                    bar.finish();
                    let mut fn_loop = fs::File::create(Path::new(&format!(
                        "{}/beh/functions/loop.mcfunction",
                        name
                    )))
                    .unwrap();
                    fn_loop.write_all(looping.join("\n").as_bytes());
                    let mut fn_init = fs::File::create(Path::new(&format!(
                        "{}/beh/functions/init.mcfunction",
                        name
                    )))
                    .unwrap();
                    fn_init.write_all(init.join("\n").as_bytes());
                    print!("Everything was done!");
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}
