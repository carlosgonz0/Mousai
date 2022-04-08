use gtk::{glib, prelude::*, subclass::prelude::*};
use once_cell::sync::Lazy;
use once_cell::unsync::OnceCell;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::cell::RefCell;

use super::SongId;
use crate::{core::DateTime, AlbumArt};

mod imp {
    use super::*;

    #[derive(Debug, Default, Serialize, Deserialize)]
    #[serde(default)]
    pub struct SongInner {
        pub last_heard: DateTime,
        pub title: String,
        pub artist: String,
        pub album: String,
        pub release_date: String,
        pub info_link: String,
        pub album_art_link: Option<String>,
        pub playback_link: Option<String>,
    }

    #[derive(Debug, Default)]
    pub struct Song {
        pub inner: RefCell<SongInner>,
        pub album_art: OnceCell<AlbumArt>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Song {
        const NAME: &'static str = "MsaiSong";
        type Type = super::Song;
    }

    impl ObjectImpl for Song {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecBoxed::new(
                        "last-heard",
                        "Last Heard",
                        "The DateTime when this was last heard",
                        DateTime::static_type(),
                        glib::ParamFlags::READWRITE | glib::ParamFlags::EXPLICIT_NOTIFY,
                    ),
                    glib::ParamSpecString::new(
                        "title",
                        "Title",
                        "Title of the song",
                        Some(""),
                        glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
                    ),
                    glib::ParamSpecString::new(
                        "artist",
                        "Artist",
                        "Artist of the song",
                        Some(""),
                        glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
                    ),
                    glib::ParamSpecString::new(
                        "album",
                        "Album",
                        "Album",
                        Some(""),
                        glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
                    ),
                    glib::ParamSpecString::new(
                        "release-date",
                        "Release Date",
                        "Release Date",
                        Some(""),
                        glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
                    ),
                    glib::ParamSpecString::new(
                        "info-link",
                        "Info Link",
                        "Link to website containing song information",
                        Some(""),
                        glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
                    ),
                    glib::ParamSpecString::new(
                        "album-art-link",
                        "Album Art Link",
                        "Link where the album art can be downloaded",
                        None,
                        glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
                    ),
                    glib::ParamSpecString::new(
                        "playback-link",
                        "Playback Link",
                        "Link containing an excerpt of the song",
                        None,
                        glib::ParamFlags::READWRITE | glib::ParamFlags::CONSTRUCT_ONLY,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "last-heard" => {
                    let last_heard = value.get().unwrap();
                    obj.set_last_heard(last_heard);
                }
                "title" => {
                    let title = value.get().unwrap();
                    self.inner.borrow_mut().title = title;
                }
                "artist" => {
                    let artist = value.get().unwrap();
                    self.inner.borrow_mut().artist = artist;
                }
                "album" => {
                    let album = value.get().unwrap();
                    self.inner.borrow_mut().album = album;
                }
                "release-date" => {
                    let release_date = value.get().unwrap();
                    self.inner.borrow_mut().release_date = release_date;
                }
                "info-link" => {
                    let info_link = value.get().unwrap();
                    self.inner.borrow_mut().info_link = info_link;
                }
                "album-art-link" => {
                    let album_art_link = value.get().unwrap();
                    self.inner.borrow_mut().album_art_link = album_art_link;
                }
                "playback-link" => {
                    let playback_link = value.get().unwrap();
                    self.inner.borrow_mut().playback_link = playback_link;
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "last-heard" => obj.last_heard().to_value(),
                "title" => obj.title().to_value(),
                "artist" => obj.artist().to_value(),
                "album" => obj.album().to_value(),
                "release-date" => obj.release_date().to_value(),
                "info-link" => obj.info_link().to_value(),
                "album-art-link" => obj.album_art_link().to_value(),
                "playback-link" => obj.playback_link().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct Song(ObjectSubclass<imp::Song>);
}

impl Song {
    /// The parameter `info_link` must be unique to each [`Song`] so that [`SongList`] will
    /// treat them different.
    ///
    /// The last heard will be the `DateTime` when this is constructed
    pub fn builder<'a>(
        title: &'a str,
        artist: &'a str,
        album: &'a str,
        release_date: &'a str,
        info_link: &'a str,
    ) -> SongBuilder {
        SongBuilder::new(title, artist, album, release_date, info_link)
    }

    pub fn set_last_heard(&self, last_heard: DateTime) {
        self.imp().inner.borrow_mut().last_heard = last_heard;
        self.notify("last-heard");
    }

    pub fn last_heard(&self) -> DateTime {
        self.imp().inner.borrow().last_heard
    }

    pub fn title(&self) -> String {
        self.imp().inner.borrow().title.clone()
    }

    pub fn artist(&self) -> String {
        self.imp().inner.borrow().artist.clone()
    }

    pub fn album(&self) -> String {
        self.imp().inner.borrow().album.clone()
    }

    pub fn release_date(&self) -> String {
        self.imp().inner.borrow().release_date.clone()
    }

    pub fn info_link(&self) -> String {
        self.imp().inner.borrow().info_link.clone()
    }

    pub fn album_art_link(&self) -> Option<String> {
        self.imp().inner.borrow().album_art_link.clone()
    }

    pub fn playback_link(&self) -> Option<String> {
        self.imp().inner.borrow().playback_link.clone()
    }

    pub fn id(&self) -> SongId {
        // Song's info_link is unique to every song
        SongId::new(&self.info_link())
    }

    pub fn album_art(&self) -> anyhow::Result<&AlbumArt> {
        self.imp()
            .album_art
            .get_or_try_init(|| AlbumArt::for_song(self))
    }
}

impl Serialize for Song {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.imp().inner.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Song {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let song: Self = glib::Object::new(&[]).expect("Failed to create song.");
        song.imp()
            .inner
            .replace(imp::SongInner::deserialize(deserializer)?);
        Ok(song)
    }
}

pub struct SongBuilder {
    properties: Vec<(&'static str, glib::Value)>,
}

impl SongBuilder {
    pub fn new(
        title: &str,
        artist: &str,
        album: &str,
        release_date: &str,
        info_link: &str,
    ) -> Self {
        Self {
            properties: vec![
                ("title", title.to_value()),
                ("artist", artist.to_value()),
                ("album", album.to_value()),
                ("release-date", release_date.to_value()),
                ("info-link", info_link.to_value()),
            ],
        }
    }

    pub fn album_art_link(&mut self, album_art_link: &str) -> &mut Self {
        self.properties
            .push(("album-art-link", album_art_link.to_value()));
        self
    }

    pub fn playback_link(&mut self, playback_link: &str) -> &mut Self {
        self.properties
            .push(("playback-link", playback_link.to_value()));
        self
    }

    pub fn build(&self) -> Song {
        glib::Object::with_values(Song::static_type(), &self.properties)
            .expect("Failed to create Song.")
            .downcast()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn properties() {
        let song = Song::builder(
            "Some song",
            "Someone",
            "https://somewhere.com",
            "SomeAlbum",
            "00-00-0000",
        )
        .album_art_link("https://album.png")
        .playback_link("https://test.mp3")
        .build();

        assert_eq!(song.title(), "Some song");
        assert_eq!(song.artist(), "Someone");
        assert_eq!(song.album(), "SomeAlbum");
        assert_eq!(song.release_date(), "00-00-0000");
        assert_eq!(song.info_link(), "https://somewhere.com");
        assert_eq!(song.album_art_link().as_deref(), Some("https://album.png"));
        assert_eq!(song.playback_link().as_deref(), Some("https://test.mp3"));
    }
}
