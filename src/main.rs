mod content;
use content::catalog::Catalog;
use content::media::Media;

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

    let item = catalog.get_by_index(0);
    // println!("Item: {:#?}", item);

    // match catalog.get_by_index(60) {
    //     Some(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     None => {
    //         println!("No value here!");
    //     }
    // }

    // if let Some(value) = catalog.get_by_index(98980) {
    //     println!("Item in pattern match: {:#?}", value)
    // } else {
    //     println!("No value here!!!!!!");
    // }

    // match catalog.items.get(100) {
    //     Some(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     None => {
    //         println!("Nothing at that index");
    //     }
    // }
}
