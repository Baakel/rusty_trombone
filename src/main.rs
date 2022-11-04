use rspotify::{model::{AdditionalType, Country, Market}, prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth, Config};
use rspotify::model::PlayableItem;

#[tokio::main]
async fn main() {
    // env_logger::init();
    let creds = Credentials::from_env().unwrap();

    let oauth = OAuth::from_env(scopes!("user-read-currently-playing")).unwrap();

    let config = Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };

    // let mut spotify = AuthCodeSpotify::new(creds, oauth);
    let mut spotify = AuthCodeSpotify::with_config(creds, oauth, config);

    let url = spotify.get_authorize_url(false).unwrap();

    spotify.prompt_for_token(&url).await.unwrap();

    let market = Market::Country(Country::Germany);
    let additional_types = [AdditionalType::Track];
    let artists = spotify
        .current_playing(Some(&market), Some(&additional_types))
        .await;
    // println!("Response: {artists:?}");

    if artists.is_err() {
        println!("There was an error fetching your request {:?}", artists);
    }

    let artist_result = artists.unwrap().unwrap();

    // println!("Artist res is {artist_result:?}");

    let currently_playing = artist_result.is_playing;
    if !currently_playing {
        return;
    }

    let song = artist_result.item.unwrap();

    match song {
        PlayableItem::Track(track) => {
            let song_name = track.name;
            println!("Found the fkn song name! {song_name}");
        },
        _ => println!("do nothing"),
    };

}
