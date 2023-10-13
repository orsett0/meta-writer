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

use lofty::{
    mp4::{Atom, AtomData, AtomIdent, Ilst},
    read_from_path, ItemKey, ItemValue, Tag, TagExt, TagItem, TagType, TaggedFileExt, Picture,
};

use phf::phf_map;
use std::{borrow::Cow, convert::TryInto, env, process::exit, fs::File};

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
    "ParentalAdvisory" => ItemKey::ParentalAdvisory,
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

    let mut metadata = json::parse(json_encoded_metadata)
        .expect("ERROR: Metadata string is not valid JSON!");

    let front_cover_path = metadata.remove("FrontCover");
    if front_cover_path.is_string() {
        let front_cover = &mut File::open(front_cover_path.as_str().unwrap_or_default())
            .expect("Error opening the front cover image.");

        tag.set_picture(0, Picture::from_reader(front_cover).expect("Error reading front cover image."));
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

    // If the file accepts ilst metadata, loop through those we set asides
    if tag.tag_type() != TagType::Mp4Ilst {
        tag.save_to_path(sandboxed_file_path)
            .expect("ERROR: Failed to write the tag!");

        return;
    }

    // "When converting from Tag, only items with a value of ItemValue::Text, as well as pictures, will be preserved.
    // An attempt will be made to create the TrackNumber/TrackTotal (trkn) and DiscNumber/DiscTotal (disk) pairs."
    // From https://docs.rs/lofty/latest/lofty/mp4/struct.Ilst.html#conversions
    let mut ilst = Ilst::from(tag.to_owned());
    let default: [u8; 4] = [b'-'; 4];

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

    for key in ilst_style {
        ilst.replace_atom(Atom::new(
            AtomIdent::Fourcc(key.as_bytes()[0..4].try_into().unwrap_or(default)),
            AtomData::UTF8(metadata[key].to_string()),
        ));
    }

    ilst.save_to_path(sandboxed_file_path)
        .expect("ERROR: Failed to write the tag!");
}
