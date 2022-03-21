struct Upload {
    filename: String,
}

#[allow(dead_code)]
struct Photo {
    filename: String,
    width: u32,
    height: u32,
}

trait Summary {
    fn summarize(&self) -> String {
        String::from("No summary available.")
    }
}

// All default implementations
impl Summary for Upload {}

// Default implementations can be overwritten
impl Summary for Photo {
    fn summarize(&self) -> String {
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
    let upload = Upload { filename: String::from("notes.txt") };
    
    println!("Upload: {}", upload.summarize());

    let photo = Photo {
        filename: String::from("stock_crustacean.png"),
        width: 100,
        height: 150,
    };

    println!("Photo: {}", photo.summarize());
    println!("Size: {}", photo.size());
}