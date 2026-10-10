#![allow(dead_code)]

struct Upload {
    filename: String,
}

#[allow(dead_code)]
struct Photo {
    filename: String,
    width: u32,
    height: u32,
}

trait Description {
    fn describe(&self) -> String {
        String::from("No description available.")
    }
}

// All default implementations
impl Description for Upload {}

// Default implementations can be overwritten
impl Description for Photo {
    fn describe(&self) -> String {
        format!("{} ({} x {})", self.filename, self.width, self.height)
    }
}

// Default implementations can rely on methods with no defaults
trait Size {
    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn size(&self) -> u32 {
        self.width() * self.height()
    }
}

impl Size for Photo {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    // Using default impl of `size()`
}

fn main() {
    let upload = Upload {
        filename: String::from("notes.txt"),
    };

    println!("Upload: {}", upload.describe());

    let photo = Photo {
        filename: String::from("stock_crustacean.png"),
        width: 100,
        height: 150,
    };

    println!("Photo: {}", photo.describe());
    println!("Size: {}", photo.size());
}
