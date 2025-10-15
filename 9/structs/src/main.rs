struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

// structs are private by default
// visible to only module where they're declared
// and its submodules. To make a struct visible outside
// its module. Prefix it with `pub`
// pub struct GrayscaleMap {
//     pixels: Vec<u8>,
//     size: (usize, usize),
// }

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

fn main() {
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
    assert_eq!(image.pixels.len(), width * height);
    assert_eq!(image.size.0 * image.size.1, width * height);
}
