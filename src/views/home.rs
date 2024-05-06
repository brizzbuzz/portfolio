use askama::Template;
use rocket::get;

struct Profile<'a> {
    image_src: &'a str,
}

struct Character<'a> {
    image_src: &'a str,
    rotation: &'a str,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    name: &'a str,
    description: &'a str,
    profiles: Vec<Profile<'a>>,
    characters: Vec<Character<'a>>,
}

#[get("/")]
pub fn index() -> HomeTemplate<'static> {
    let profiles = vec![
        Profile { image_src: "/public/images/logos/github.png" },
        Profile { image_src: "/public/images/logos/instagram.png" },
        Profile { image_src: "/public/images/logos/linkedin.png" },
        Profile { image_src: "/public/images/logos/x.png" },
    ];

    let characters = vec![
        Character { image_src: "/public/images/characters/cyberpunk/distressed-profess.png", rotation: "rotate-2" },
        Character { image_src: "/public/images/characters/cyberpunk/pirate-squirrel.png", rotation: "-rotate-2" },
        Character { image_src: "/public/images/characters/cyberpunk/the-man-who-snapped.png", rotation: "rotate-2" },
        Character { image_src: "/public/images/characters/cyberpunk/young-princess.png", rotation: "rotate-2" },
        Character { image_src: "/public/images/characters/cyberpunk/evil-samurai.png", rotation: "-rotate-2" },
    ];

    HomeTemplate {
        name: "RYAN BRINK",
        description: "Cyberspace Cowboy :: Occasionally I write things, more often I code things",
        profiles,
        characters,
    }
}
