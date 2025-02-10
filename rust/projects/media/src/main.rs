use crate::Media::Placeholder;
use crate::MightHavaAValue::HaveAValue;

#[derive(Debug)]
enum Media {
    Book {
        title: String,
        author: String,
    },
    Movie {
        title: String,
        director: String,
    },
    Audiobook {
        title: String,
    },
    /// episode number
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("book: {} by {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("movie: {} by {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("audiobook: {}", title)
        // }else {
        //     String::from("Media description")
        // }
        match self {
            Media::Book { title, author } => format!("book: {} by {}", title, author),
            Media::Movie { title, director } => format!("movie: {} by {}", title, director),
            Media::Audiobook { title } => {
                format!("audiobook: {}", title)
            }
            Media::Podcast(episode) => {
                format!("podcast episode: {}", episode)
            }
            Media::Placeholder => String::from("placeholder"),
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
    fn get_by_index(&self, index: usize) -> MightHavaAValue {
        if self.items.len() > index {
            MightHavaAValue::HaveAValue(&self.items[index])
        } else {
            MightHavaAValue::NoValue
        }
    }
    fn get_by_index_with_option(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            Option::Some(&self.items[index])
        } else {
            Option::None
        }
    }
}

/// lifetime
#[derive(Debug)]
enum MightHavaAValue<'a> {
    HaveAValue(&'a Media),
    NoValue,
}

fn print_media(media: &Media) {
    println!("(2) {:#?}", media);
}

fn main() {
    let book = Media::Book {
        title: String::from("Origin"),
        author: String::from("Dan Brown"),
    };
    let movie = Media::Movie {
        title: String::from("Avatar"),
        director: String::from("James Cameron"),
    };
    let audiobook = Media::Audiobook {
        title: String::from("Harry Potter"),
    };
    println!("(1) {:#?}", book.description());
    println!("(1) {:#?}", movie.description());
    println!("(1) {:#?}", audiobook.description());

    print_media(&book);
    print_media(&movie);
    print_media(&audiobook);

    let mut catalog = Catalog::new();
    catalog.add(book);
    catalog.add(movie);
    catalog.add(audiobook);

    println!("(3) {:#?}", &catalog);

    catalog.add(Media::Podcast(1));
    catalog.add(Media::Placeholder);
    println!("(4) {:#?}", &catalog);
    println!("(5) {:#?}", &catalog.items.get(0));
    println!("(6) {:#?}", &catalog.items.get(100));
    match catalog.items.get(0) {
        Some(value) => {
            println!("(7) {:#?}", &value);
        }
        None => {
            println!("(7) Nothing at that index");
        }
    }

    let item = catalog.get_by_index(40);
    println!("(8) {:#?}", &item);

    if let MightHavaAValue::HaveAValue(value) = catalog.get_by_index(2) {
        println!("(9) {:#?}", value);
    } else {
        println!("(9) Nothing at that index");
    }

    if let Some(value) = catalog.get_by_index_with_option(2) {
        println!("(10) {:#?}", value);
    } else {
        println!("(10) Nothing at that index");
    }

    // not recommended
    // change index 3 -> 8 panic
    let item = catalog.get_by_index_with_option(3)
        .unwrap();
    println!("(11) {:#?}", item);

    // change index 3 -> 8 panic
    let item = catalog
        .get_by_index_with_option(3)
        .expect("Expected to find an item");
    println!("(12) {:#?}", item);

    let item = catalog.get_by_index_with_option(8).unwrap_or(&Placeholder);
    println!("(13) {:#?}", item);
}
