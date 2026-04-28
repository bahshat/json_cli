use inquire::{Confirm, CustomType, Select, Text, list_option::ListOption};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Scene {
    layout: String,
    text: String,
    duration: i32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    points: Vec<String>,
}

fn main() {
    let mut scenes = Vec::new();
    let layouts = vec!["grid", "quote", "hero", "comparison", "bullets"];

    loop {
        println!("--- New Scene ---");
        
        let layout = Select::new("Select layout:", layouts.clone()).prompt().unwrap();
        let text = Text::new("Enter text:").prompt().unwrap();
        let duration = CustomType::<i32>::new("Duration (sec):").with_default(5).prompt().unwrap();

        let mut points = Vec::new();
        if layout == "bullets" {
            loop {
                let point = Text::new("Add bullet (empty to finish):").prompt().unwrap();
                if point.is_empty() { break; }
                points.push(point);
            }
        }

        scenes.push(Scene { layout: layout.to_string(), text, duration, points });

        if !Confirm::new("Add another scene?").with_default(true).prompt().unwrap() {
            break;
        }
    }

    let json = serde_json::to_string_pretty(&scenes).unwrap();
    fs::write("scenes.json", &json).expect("Unable to write file");
    
    println!("\n✅ Success! JSON saved to scenes.json");
    println!("{}", json);
}
