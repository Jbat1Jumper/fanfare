use rand::prelude::*;
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::iter::repeat;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Model {
    relaxed: bool,
    wrote_down_stuff: bool,
    first: Vec<usize>,
    changes: Vec<Vec<usize>>,
    last_fanfare: Option<Fanfare>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Fanfare {
    melody: Vec<usize>,
}

fn pick_next(dist: &Vec<usize>, relaxed: bool) -> usize {
    let mut rng = rand::thread_rng();
    dist.iter()
        .enumerate()
        .flat_map(|(note, q)| repeat(note).take(if relaxed { *q } else { *q - 1 }))
        .choose(&mut rng)
        .unwrap_or(0)
}

impl Model {
    fn new() -> Self {
        Self {
            relaxed: true,
            wrote_down_stuff: false,
            last_fanfare: None,
            first: repeat(1).take(12).collect(),
            changes: repeat(repeat(1).take(12).collect()).take(12).collect(),
        }
    }
    fn improvise(&mut self) -> Fanfare {
        let mut dist = &self.first;
        let mut melody = vec![];

        while melody.len() < 9 {
            let note = pick_next(dist, self.relaxed);
            melody.push(note);
            dist = &self.changes[note];
        }

        let f = Fanfare { melody };
        self.last_fanfare = Some(f.clone());
        f
    }
    fn _play_scale(&self) -> Fanfare {
        Fanfare {
            melody: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0],
        }
    }
    fn write_down_last_fanfare(&mut self) {
        if let Some(ref f) = self.last_fanfare {
            let n = f.melody.first().unwrap();
            self.first[*n] += 1;

            for c in f.melody.windows(2) {
                let (s, d) = (c[0], c[1]);
                self.changes[s][d] += 1;
            }
            self.wrote_down_stuff = true;
            self.last_fanfare = None;
        }
    }
    fn path() -> std::path::PathBuf {
        let path_str = if cfg!(windows) {
            format!("{}/.fanfare", std::env::var("USERPROFILE").unwrap())
        } else {
            format!("{}/.fanfare", std::env::var("HOME").unwrap())
        };
        std::path::PathBuf::from(path_str)
    }
    fn load() -> Self {
        let path = Self::path();
        if path.exists() {
            //println!("Loading model");
            let f = std::fs::File::open(path).expect("Failed to read resources file");
            let model = serde_json::from_reader(f).expect("Failed to parse resources file");
            //println!("Loaded model {:?}", model);
            model
        } else {
            //println!("New model created");
            Model::new()
        }
    }
    fn store(&self) {
        let path = Self::path();
        //println!("Storing model");
        let f = std::fs::File::create(path).expect("Failed to write to resources file");
        serde_json::to_writer_pretty(f, self).expect("Failed to rialize to resources files");
        //println!("Stored model {:?}", self);
    }
}

impl Fanfare {
    fn play(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        for (i, note) in self.melody.iter().enumerate() {
            // Add a dummy source of the sake of the example.
            let freq = 370.0 * (2f32).powf(*note as f32 / 12.0);

            let source = SineWave::new(freq)
                .take_duration(Duration::from_secs_f32(if i < 8 { 0.25 } else { 0.50 }))
                .amplify(0.20);
            sink.append(source);
        }

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();
    }
}

#[derive(Debug, StructOpt)]
/// Give sound to your terminal
///
/// Play improvised melodies, give feedback on which ones
/// were nice and then focus on replicating them.
///
enum Cli {
    /// Play a random fanfare
    Play,
    /// Play the last fanfare again
    Replay,
    /// The last fanfare was nice and the following ones should sound more like it
    WasNice,
    /// Start learning again
    Reset,
    /// From now on, improvise with freedom
    Relax,
    /// From now on, focus on replicating nice tunes
    Focus,
}

fn main() {
    let opt = Cli::from_args();
    //println!("{:?}", opt);
    let mut model = Model::load();
    match opt {
        Cli::Play => {
            println!("Shoo bop doo wop boo...");
            let f = model.improvise();
            f.play();
        }
        Cli::Replay => {
            if let Some(ref f) = model.last_fanfare {
                println!("From the top!");
                f.play();
            } else {
                println!("Did not play anything yet!");
            }
        }
        Cli::WasNice => {
            println!("Thanks!");
            model.write_down_last_fanfare();
        }
        Cli::Reset => {
            println!("Lets start again!");
            model = Model::new();
        }
        Cli::Relax => {
            if model.relaxed {
                println!("I'm already chill!");
            } else {
                model.relaxed = true;
                println!("I'll chill and improvise a little bit...");
            }
        }
        Cli::Focus => {
            if model.wrote_down_stuff {
                if model.relaxed {
                    println!("I'll try to focus...");
                    model.relaxed = false;
                } else {
                    println!("I'm already focused!");
                }
            } else {
                println!("Cannot focus until I've got some decent melodies!");
            }
        }
    }
    model.store();
}
