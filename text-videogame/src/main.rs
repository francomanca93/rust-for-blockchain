use csv::{ReaderBuilder, StringRecord};
use std::collections::{HashMap};
use std::io::Read;
use std::{fs};

const FILENAME: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

// TIPO, TAG, TEXTO, VIDA
#[derive(Debug)]
struct StoryData {
    data_type: String,
    tag: String,
    text: String,
    live: i32,
    options: Vec<StoryData>,
}

impl StoryData {
    fn new(row: StringRecord) -> StoryData{
        let live: &str = row.get(3).unwrap().trim();
        let live: i32 = live.parse().unwrap_or(0);
        return StoryData{
            data_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            live: live,
            options: vec![]
        };

    }
}

fn main() {

    let mut live = 100;
    let mut current_tag = FIRST_TAG;

    let mut last_record: String = "".to_string();

    let mut story_data_all: HashMap<String, StoryData> = HashMap::new();

    let content: String = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records(){
        let result = result.unwrap();
        let data: StoryData = StoryData:: new(result);

        if data.data_type == "SITUACION"{
            let record_tag = data.tag.clone();
            story_data_all.insert(record_tag.clone(), data);
            last_record = record_tag;

        } else if data.data_type == "OPCION"{
            if let Some(data_option) = story_data_all.get_mut(&last_record){
                (*data_option).options.push(data);
                //println!("{}", (*data_option).tag);

            }
        }
    }

    // Game loop
    loop{
        println!("You have {} lives", live);

        if let Some(data_option) = story_data_all.get(current_tag){
            println!("{:?}", data_option.text);
            
            for (index, option) in data_option.options.iter().enumerate() {
                println!("[{}] {}", index, option.text);
            }
            
            let mut choice = String::new();
            std::io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim().parse().unwrap_or(99);
            
            if let Some(choice_taken) = &data_option.options.get(choice){
                current_tag = &choice_taken.tag;
            } else{
                println!("You choice an invalid command")
            }

            live += data_option.live;
            println!("");
        } else {
            break;
        }

        // If live <= 0, end game
        if live <= 0{
            println!("You have lost");
            break;
        }
    }

}
