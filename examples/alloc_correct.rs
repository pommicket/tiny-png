fn main() {
	let mut png = &include_bytes!("image.png")[..];
	let header = tiny_png::read_png_header(&mut png).expect("bad PNG");
	println!("need {} bytes of memory", header.required_bytes());
	let mut buffer = vec![0; header.required_bytes()];
	let image = tiny_png::read_png(&mut png, Some(&header), &mut buffer).expect("bad PNG");
	println!("{}x{} image", image.width(), image.height());
	let pixels = image.pixels();
	println!(
		"top-left pixel is #{:02x}{:02x}{:02x}",
		pixels[0], pixels[1], pixels[2]
	);
	// (^ this only makes sense for RGB 8bpc images)
}
