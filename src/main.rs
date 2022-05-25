use rand::prelude::*;
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use serde::{Deserialize, Serialize};
use std::iter::repeat;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Clone, Serialize, Deserialize)]
struct Model {
    first: Vec<usize>,
    occurrences: Vec<Vec<usize>>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Fanfare {
    melody: Vec<usize>,
}

fn pick_next(dist: &Vec<usize>) -> usize {
    let mut rng = rand::thread_rng();
    dist.iter()
        .enumerate()
        .flat_map(|(note, q)| repeat(note).take(*q))
        .choose(&mut rng)
        .unwrap()
}

impl Model {
    fn new() -> Self {
        Self {
            first: repeat(1).take(12).collect(),
            occurrences: repeat(repeat(1).take(12).collect()).take(12).collect(),
        }
    }
    fn improvise(&self) -> Fanfare {
        let mut dist = &self.first;
        let mut melody = vec![];

        while melody.len() < 9 {
            let note = pick_next(dist);
            melody.push(note);
            dist = &self.occurrences[note];
        }

        Fanfare { melody }
    }
    fn play_scale(&self) -> Fanfare {
        Fanfare {
            melody: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0],
        }
    }
    fn learn(&mut self, fanfare: Fanfare) {}
}

impl Fanfare {
    fn play(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        for (i, note) in self.melody.iter().enumerate() {
            // Add a dummy source of the sake of the example.
            let freq = 370.0 * (2f32).powf(*note as f32 / 12.0);

            let source = SineWave::new(freq)
                .take_duration(Duration::from_secs_f32(if i < 8  {0.25} else {0.50}))
                .amplify(0.20);
            sink.append(source);
        }

        // The sound plays in a separate thread. This call will block the current thread until the sink
        // has finished playing all its queued sounds.
        sink.sleep_until_end();
    }
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Give sound to your terminal")]
enum Cli {
    /// Play a random fanfare
    Play,
    /// The last fanfare was nice and the following ones should sound more like it
    WasNice,
    /// Start learning again
    Reset,
}

fn main() {
    let opt = Cli::from_args();
    println!("{:?}", opt);
    match opt {
        Cli::Play => play(),
        Cli::WasNice => {
            todo!()
        }
        Cli::Reset => {
            todo!()
        }
    }
}

fn play() {
    println!("Hello, sound!");

    let m = Model::new();

    let f = m.improvise();

    f.play();
}
