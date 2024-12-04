#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
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
        }
    }
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

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());

    print_media(good_movie);
    print_media(bad_book);
    print_media(audiobook);
}
