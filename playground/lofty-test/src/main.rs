use lofty::{Accessor, Probe, Tag, TagExt, TaggedFileExt};

use structopt::StructOpt;
use std::path::PathBuf;

//use wasm_bindgen::prelude::*


#[derive(Debug, StructOpt)]
#[structopt(name = "tag_writer", about = "A simple tag writer example")]
struct Opt {
	#[structopt(short, long)]
	title: Option<String>,

	#[structopt(short, long)]
	artist: Option<String>,

	#[structopt(short = "A", long)]
	album: Option<String>,

	#[structopt(short, long)]
	genre: Option<String>,

	#[structopt(parse(from_os_str))]
	path: PathBuf,
}

pub fn write_metadata(title: String, artist: String, album: String, genre: String, path: PathBuf) {
    let mut tagged_file = Probe::open(&path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");
    
    let tag = match tagged_file.primary_tag_mut() {
        Some(primary_tag) => primary_tag,
        None => {
            if let Some(first_tag) = tagged_file.first_tag_mut() {
                first_tag
            } else {
                let tag_type = tagged_file.primary_tag_type();
                eprintln!("WARN: No tags found, creating a new tag of type `{tag_type:?}`");

                tagged_file.insert_tag(Tag::new(tag_type));
                tagged_file.primary_tag_mut().unwrap()
            }
        },
    };

    tag.set_title(title);
    tag.set_artist(artist);
    tag.set_album(album);
    tag.set_genre(genre);
    
    tag.save_to_path(&path)
    .expect("ERROR: Failed to write the tag!");
}

fn main() {
	let opt = Opt::from_args();

	write_metadata(opt.title.unwrap(), opt.artist.unwrap(), opt.album.unwrap(), opt.genre.unwrap(), opt.path)
}
