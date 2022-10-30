use csv::{ReaderBuilder, StringRecord};
use std::collections::{HashMap};
use std::{fs};

const FILENAME: &str = "history.csv";

// TIPO, TAG, TEXTO, VIDA
#[derive(Debug)]
struct StoryData {
    data_type: String,
    tag: String,
    text: String,
    live: i32
}

impl StoryData {
    fn new(row: StringRecord) -> StoryData{
        let live: &str = row.get(3).unwrap().trim();
        let live: i32 = live.parse().unwrap_or(0);
        return StoryData{
            data_type: row.get(0).unwrap().trim().to_string(),
            tag: row.get(1).unwrap().trim().to_string(),
            text: row.get(2).unwrap().trim().to_string(),
            live: live
        };

    }
}

fn main() {
    let mut story_data_all: HashMap<String, StoryData> = HashMap::new();

    let content: String = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());

    for result in rdr.records(){
        let result = result.unwrap();
        let data: StoryData = StoryData:: new(result);
        story_data_all.insert(data.tag.clone(), data);
    }

    println!("{:?}", story_data_all);
}
