#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        //     if let Media::Book { title, author } = self {
        //         format!("Book: {} {}", title, author)
        //     } else if let Media::Movie { title, director } = self {
        //         format!("Movie: {} {}", title, director)
        //     } else if let Media::Audiobook { title } = self {
        //         format!("Audiobook: {}", title)
        //     } else {
        //         String::from("Media Description")
        //     }

        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {} ", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            MightHaveAValue::ThereIaAValue(&self.items[index])
        } else {
            MightHaveAValue::NoValueAvailable
        }
    }
}

#[derive(Debug)]
enum MightHaveAValue<'a> {
    ThereIaAValue(&'a Media),
    NoValueAvailable,
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Title"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());

    // print_media(good_movie);
    // print_media(bad_book);
    // print_media(audiobook);

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    // let item = catalog.get_by_index(0);
    // println!("Item: {:#?}", item);

    // match catalog.get_by_index(60) {
    //     MightHaveAValue::ThereIaAValue(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     MightHaveAValue::NoValueAvailable => {
    //         println!("No value here!");
    //     }
    // }

    if let MightHaveAValue::ThereIaAValue(value) = catalog.get_by_index(98980) {
        println!("Item in pattern match: {:#?}", value)
    } else {
        println!("No value here!!!!!!");
    }

    // match catalog.items.get(100) {
    //     Some(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     None => {
    //         println!("Nothing at that index");
    //     }
    // }
}
