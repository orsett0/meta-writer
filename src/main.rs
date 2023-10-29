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

use jzon::JsonValue;
use lofty::{
    mp4::{Atom, AtomData, AtomIdent, Mp4File, Ilst},
    read_from_path, ItemKey, ItemValue, Tag, TagExt, TagItem, TagType, TaggedFileExt, Picture, AudioFile, ParseOptions,
};

use phf::{phf_map, phf_set};
use std::{borrow::Cow, convert::TryInto, env, process::exit, fs::File, collections::HashMap};

const TAGS_KEY: phf::Map<&'static str, ItemKey> = phf_map![
    "AlbumArtist" => ItemKey::AlbumArtist,
    "AlbumArtistSortOrder" => ItemKey::AlbumArtistSortOrder,
    "AlbumTitle" => ItemKey::AlbumTitle,
    "AlbumTitleSortOrder" => ItemKey::AlbumTitleSortOrder,
    "AppleId3v2ContentGroup" => ItemKey::AppleId3v2ContentGroup,
    "AppleXid" => ItemKey::AppleXid,
    "Arranger" => ItemKey::Arranger,
    "AudioFileUrl" => ItemKey::AudioFileUrl,
    "AudioSourceUrl" => ItemKey::AudioSourceUrl,
    "Barcode" => ItemKey::Barcode,
    "Bpm" => ItemKey::Bpm,
    "CatalogNumber" => ItemKey::CatalogNumber,
    "Color" => ItemKey::Color,
    "Comment" => ItemKey::Comment,
    "CommercialInformationUrl" => ItemKey::CommercialInformationUrl,
    "Composer" => ItemKey::Composer,
    "ComposerSortOrder" => ItemKey::ComposerSortOrder,
    "Conductor" => ItemKey::Conductor,
    "ContentGroup" => ItemKey::ContentGroup,
    "CopyrightMessage" => ItemKey::CopyrightMessage,
    "CopyrightUrl" => ItemKey::CopyrightUrl,
    "Description" => ItemKey::Description,
    "Director" => ItemKey::Director,
    "DiscNumber" => ItemKey::DiscNumber,
    "DiscTotal" => ItemKey::DiscTotal,
    "EncodedBy" => ItemKey::EncodedBy,
    "EncoderSettings" => ItemKey::EncoderSettings,
    "EncoderSoftware" => ItemKey::EncoderSoftware,
    "EncodingTime" => ItemKey::EncodingTime,
    "Engineer" => ItemKey::Engineer,
    "FileOwner" => ItemKey::FileOwner,
    "FileType" => ItemKey::FileType,
    "FlagCompilation" => ItemKey::FlagCompilation,
    "FlagPodcast" => ItemKey::FlagPodcast,
    "Genre" => ItemKey::Genre,
    "InitialKey" => ItemKey::InitialKey,
    "InternetRadioStationName" => ItemKey::InternetRadioStationName,
    "InternetRadioStationOwner" => ItemKey::InternetRadioStationOwner,
    "InvolvedPeople" => ItemKey::InvolvedPeople,
    "Isrc" => ItemKey::Isrc,
    "Label" => ItemKey::Label,
    "Language" => ItemKey::Language,
    "Length" => ItemKey::Length,
    "License" => ItemKey::License,
    "Lyricist" => ItemKey::Lyricist,
    "Lyrics" => ItemKey::Lyrics,
    "MixDj" => ItemKey::MixDj,
    "MixEngineer" => ItemKey::MixEngineer,
    "Mood" => ItemKey::Mood,
    "Movement" => ItemKey::Movement,
    "MovementNumber" => ItemKey::MovementNumber,
    "MovementTotal" => ItemKey::MovementTotal,
    "MusicBrainzArtistId" => ItemKey::MusicBrainzArtistId,
    "MusicBrainzRecordingId" => ItemKey::MusicBrainzRecordingId,
    "MusicBrainzReleaseArtistId" => ItemKey::MusicBrainzReleaseArtistId,
    "MusicBrainzReleaseGroupId" => ItemKey::MusicBrainzReleaseGroupId,
    "MusicBrainzReleaseId" => ItemKey::MusicBrainzReleaseId,
    "MusicBrainzTrackId" => ItemKey::MusicBrainzTrackId,
    "MusicBrainzWorkId" => ItemKey::MusicBrainzWorkId,
    "MusicianCredits" => ItemKey::MusicianCredits,
    "OriginalAlbumTitle" => ItemKey::OriginalAlbumTitle,
    "OriginalArtist" => ItemKey::OriginalArtist,
    "OriginalFileName" => ItemKey::OriginalFileName,
    "OriginalLyricist" => ItemKey::OriginalLyricist,
    "OriginalMediaType" => ItemKey::OriginalMediaType,
    "OriginalReleaseDate" => ItemKey::OriginalReleaseDate,
    // ItemKey::ParentalAdvisory exists, and seems to be only used for rtng, but I can't find a way to use it.
    //"ParentalAdvisory" => ItemKey::ParentalAdvisory,
    "PaymentUrl" => ItemKey::PaymentUrl,
    "Performer" => ItemKey::Performer,
    "PodcastDescription" => ItemKey::PodcastDescription,
    "PodcastGlobalUniqueID" => ItemKey::PodcastGlobalUniqueID,
    "PodcastKeywords" => ItemKey::PodcastKeywords,
    "PodcastReleaseDate" => ItemKey::PodcastReleaseDate,
    "PodcastSeriesCategory" => ItemKey::PodcastSeriesCategory,
    "PodcastURL" => ItemKey::PodcastURL,
    "Popularimeter" => ItemKey::Popularimeter,
    "Producer" => ItemKey::Producer,
    "Publisher" => ItemKey::Publisher,
    "PublisherUrl" => ItemKey::PublisherUrl,
    "RadioStationUrl" => ItemKey::RadioStationUrl,
    "RecordingDate" => ItemKey::RecordingDate,
    "Remixer" => ItemKey::Remixer,
    "ReplayGainAlbumGain" => ItemKey::ReplayGainAlbumGain,
    "ReplayGainAlbumPeak" => ItemKey::ReplayGainAlbumPeak,
    "ReplayGainTrackGain" => ItemKey::ReplayGainTrackGain,
    "ReplayGainTrackPeak" => ItemKey::ReplayGainTrackPeak,
    "Script" => ItemKey::Script,
    "SetSubtitle" => ItemKey::SetSubtitle,
    "ShowName" => ItemKey::ShowName,
    "ShowNameSortOrder" => ItemKey::ShowNameSortOrder,
    "TaggingTime" => ItemKey::TaggingTime,
    "TrackArtist" => ItemKey::TrackArtist,
    "TrackArtistSortOrder" => ItemKey::TrackArtistSortOrder,
    "TrackArtistUrl" => ItemKey::TrackArtistUrl,
    "TrackNumber" => ItemKey::TrackNumber,
    "TrackSubtitle" => ItemKey::TrackSubtitle,
    "TrackTitle" => ItemKey::TrackTitle,
    "TrackTitleSortOrder" => ItemKey::TrackTitleSortOrder,
    "TrackTotal" => ItemKey::TrackTotal,
    "Work" => ItemKey::Work,
    "Writer" => ItemKey::Writer,
    "Year" => ItemKey::Year,
];

const ATOM_KEY_STRING: phf::Set<[u8; 4]> = phf_set![
    *b"----",
    *b"aART",
    *b"\xA9alb",
    *b"apID",
    *b"\xA9ART",
    *b"\xA9cmt",
    *b"cprt",
    *b"\xA9day",
    *b"\xA9gen",
    *b"\xA9grp",
    *b"\xA9lyr",
    *b"\xA9mvn",
    *b"\xA9nam",
    *b"ownr",
    *b"purd",
    *b"soaa",
    *b"soal",
    *b"soar",
    *b"soco",
    *b"sonm",
    *b"\xA9too",
    *b"\xA9wrk",
    *b"\xA9wrt",
    *b"xid "
];

const ATOM_KEY_UINT64: phf::Set<[u8; 4]> = phf_set![
    *b"plID"
];

const ATOM_KEY_UINT32: phf::Set<[u8; 4]> = phf_set![
    *b"atID",
    *b"cmID",
    *b"cnID",
    *b"geID",
    *b"sfID"
];

const ATOM_KEY_UINT16: phf::Set<[u8; 4]> = phf_set![
    *b"\xA9mvc",
    *b"\xA9mvi",
    *b"tmpo"
];

const ATOM_KEY_UINT8: phf::Set<[u8; 4]> = phf_set![
    *b"cpil",
    *b"gnre",
    *b"pgap",
    *b"rtng",
    *b"shwm",
    *b"stik"
];

const ATOM_KEY_PIC: phf::Set<[u8; 4]> = phf_set![
    *b"covr"
];

const ATOM_KEY_INTPAIR: phf::Set<[u8; 4]> = phf_set![
    *b"trkn",
    *b"disk"
];

const ATOM_DEFAULT: [u8; 4] = [b'-'; 4];

fn generate_atom_data(key: [u8; 4], value: &JsonValue) -> AtomData {
    if ATOM_KEY_STRING.contains(&key) {
        return AtomData::UTF8(value.to_string());
    }

    if ATOM_KEY_PIC.contains(&key) {
        return AtomData::Picture(Picture::from_reader(
            &mut File::open(value.to_string())
            .expect("Error opening the front cover image."))
            .expect("Error reading front cover image."))
    }

    if ATOM_KEY_UINT64.contains(&key) {
        return AtomData::Unknown { code: 21, data: Vec::from(value.as_i64().unwrap_or_default().to_be_bytes()) };
    }

    if ATOM_KEY_UINT32.contains(&key) {
        return AtomData::Unknown { code: 21, data: Vec::from(value.as_i32().unwrap_or_default().to_be_bytes()) };
    }

    if ATOM_KEY_UINT16.contains(&key) {
        return AtomData::Unknown { code: 21, data: Vec::from(value.as_i16().unwrap_or_default().to_be_bytes()) };
    }

    if ATOM_KEY_UINT8.contains(&key) {
        return AtomData::Unknown { code: 21, data: Vec::from(value.as_i8().unwrap_or_default().to_be_bytes()) };
    }

    if ATOM_KEY_INTPAIR.contains(&key) {
        let mut data: Vec<u8> = vec![0, 0];

        data.extend_from_slice(&value[0].as_i16().unwrap_or(1).to_be_bytes());
        data.extend_from_slice(&value[1].as_i16().unwrap_or(1).to_be_bytes());

        return AtomData::Unknown { code: 21, data };
    }

    println!("{} is unknown", String::from_utf8_lossy(&key));
    return AtomData::Unknown { code: 0, data: Vec::from(value.as_str().unwrap_or_default()) };
}


fn add_mapped_int_values(
    ilst: &mut Ilst,
    tags_list: &mut Vec<&str>,
    key: &str,
    value: &JsonValue,
    values: HashMap<&str, i32>,
    default: i32,
) {
    if !tags_list.contains(&key) {
        return;
    }

    let key_b = key.as_bytes()[0..4].try_into().unwrap_or(ATOM_DEFAULT);

    ilst.replace_atom(Atom::new(
        AtomIdent::Fourcc(key_b),
        generate_atom_data(
            key_b,
            &JsonValue::from(
                match values.get(value.as_str().unwrap_or_default().to_lowercase().as_str()) {
                    Some(val) => val.to_owned(),
                    None => default,
                },
            ),
        ),
    ));

    tags_list.retain(|&x| x != key);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ilst_style: Vec<&str> = Vec::new();

    if args.len() != 3 {
        // We need prog_name, json_encoded_metadata, sandboxed_file_path
        eprintln!("Invalid parameters number!");
        exit(1);
    }

    let json_encoded_metadata = &args[1];
    let sandboxed_file_path = &args[2];

    let mut tagged_file = read_from_path(sandboxed_file_path).expect("ERROR: Can't read file!");
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
        }
    };

    let mut metadata = jzon::parse(json_encoded_metadata)
        .expect("ERROR: Metadata string is not valid JSON!");

    let front_cover_path = metadata.remove("FrontCover");
    if front_cover_path.is_string() {
        let mut picture = Picture::from_reader(
            &mut File::open(front_cover_path.as_str().unwrap_or_default())
            .expect("Error opening the front cover image."))
            .expect("Error reading front cover image.");

        picture.set_pic_type(lofty::PictureType::CoverFront);
        tag.set_picture(0, picture);
    }

    // Loop through the given metadata, set aside possible ilst and ignore unknown ones
    for (key, val) in metadata.entries() {
        let item_key = TAGS_KEY.get(key);

        if item_key.is_none() {
            if key == "rDNS" || key.chars().count() == 4 {
                ilst_style.push(key);
            }

            continue;
        }

        // Assuming ItemValue::Text is a bug.
        // Unfortunately, there's no easy way of deciding what ItemValue variant a given ItemKey expects,
        // So for now i'll keep it this way
        let item = TagItem::new(
            item_key
                .unwrap_or(&ItemKey::Unknown(String::from(key)))
                .to_owned(),
            ItemValue::Text(val.to_string()),
        );

        tag.insert(item);
    }

    tag.save_to_path(sandboxed_file_path)
    .expect("ERROR: Failed to write the tag");

    // Return if the file is not an MP4, otherwise re-open it as MP4
    // "When converting from Tag, only items with a value of ItemValue::Text, as well as pictures, will be preserved.
    // An attempt will be made to create the TrackNumber/TrackTotal (trkn) and DiscNumber/DiscTotal (disk) pairs."
    // From https://docs.rs/lofty/latest/lofty/mp4/struct.Ilst.html#conversions
    if tag.tag_type() != TagType::Mp4Ilst {
        return;
    }
    
    // For some reason, save_to() fails with MP4, but read_from_path is not available
    let mut mp4_file = Mp4File::read_from(
        &mut File::open(sandboxed_file_path)
        .expect("Unable to open the file."), 
        ParseOptions::default()).expect("Unable to open the file as an MP4");
    let mut ilst = mp4_file.ilst_mut().expect("Unable to read any Ilst tag.").to_owned();

    // Insert rDNS tags.
    if ilst_style.contains(&"rDNS") {
        for obj in metadata["rDNS"].members() {
            ilst.replace_atom(Atom::new(
                AtomIdent::Freeform {
                    mean: Cow::Owned(obj["mean"].to_string()),
                    name: Cow::Owned(obj["name"].to_string()),
                },
                AtomData::UTF8(obj["data"].to_string()),
            ));
        }

        ilst_style.retain(|&x| x != "rDNS");
    }


    // For this tags, mimic the behaviour of AtomicParsley
    add_mapped_int_values(
        &mut ilst,
        &mut ilst_style,
        "rtng",
        &metadata["rtng"],
        HashMap::from([("explicit", 1), ("clean", 2)]),
        0,
    );

    add_mapped_int_values(
        &mut ilst,
        &mut ilst_style,
        "stik",
        &metadata["stik"],
        HashMap::from([
            ("home video", 0),
            ("audiobook", 2),
            ("whacked bookmark", 5),
            ("music video", 6),
            ("movie", 9),
            ("short film", 9),
            ("tv show", 10),
            ("booklet", 11),
        ]),
        1,
    );

    // Insert the remaining tags.
    for key in ilst_style {
        let atom_key = key.as_bytes()[0..4].try_into().unwrap_or(ATOM_DEFAULT);

        ilst.replace_atom(Atom::new(
            AtomIdent::Fourcc(atom_key),
            generate_atom_data(atom_key, &metadata[key]),
        ));
    }

    ilst.save_to_path(sandboxed_file_path)
    .expect("ERROR: Failed to write the tag");
}
