/*
meta-writerCopyright 2023 Alessio Orsini <alessiorsini.ao@proton.me>
SPDX-License-Identifier: Apache-2.0

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use lofty::{Probe, Tag, TagExt, TaggedFileExt, mp4::{AtomIdent, AtomData, Atom, Ilst}, TagType, ItemKey};

use std::{path::PathBuf, borrow::Cow};

fn main() {
    let path = PathBuf::from("samples/service-login.mp3");

	let mut tagged_file = Probe::open(&path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");

    let mut tag = match tagged_file.primary_tag_mut() {
        Some(primary_tag) => primary_tag.to_owned(),
        None => {
            if let Some(first_tag) = tagged_file.first_tag_mut() {
                first_tag.to_owned()
            } else {
                let tag_type = tagged_file.primary_tag_type();
                eprintln!("WARN: No tags found, creating a new tag of type `{tag_type:?}`");

                tagged_file.insert_tag(Tag::new(tag_type));
                tagged_file.primary_tag_mut().unwrap().to_owned()
            }
        },
    };

    let atom = Atom::new(
        AtomIdent::Freeform { mean: (Cow::Borrowed("com.apple.iTunes")), name: (Cow::Borrowed("MEDIA")) },
        AtomData::UTF8(String::from("Digital Media1"))
    );

    tag.insert_text(ItemKey::TrackTitle, String::from("Title3"));
    tag.insert_text(ItemKey::TrackArtist, String::from("Artist1"));
    tag.insert_text(ItemKey::AlbumTitle, String::from("Album1"));
    tag.insert_text(ItemKey::Genre, String::from("Genre1"));
    
    if tag.tag_type() == TagType::Mp4Ilst {
        let mut ilst = Ilst::from(tag.to_owned());

        ilst.replace_atom(atom);

        tag = Tag::from(ilst);
    }

    tag.save_to_path(&path)
        .expect("ERROR: Failed to write the tag!");
}
