# Image Combiner

A straightforward project designed to seamlessly merge two images into one.

## Getting Started

Follow these simple steps to combine your images effortlessly:

1. Navigate to the project directory in your console.

2. Run the following command to build the project with optimal performance:
   ```bash
   cargo build --release
   ```

3. Execute the combiner with the paths to your desired input images and the desired output location:
   ```bash
   ./target/release/combiner images/pro.png images/fcc_glyph.png images/output.png
   ```

   - Replace `images/pro.png` and `images/fcc_glyph.png` with the paths to your two images to be combined.
   - Set `images/output.png` as the path for the resulting image.

## Example
```bash
./target/release/combiner images/beautiful.jpg images/logo.png images/result.jpg
```

Now, behold your seamlessly combined image in the specified output location!

Feel free to explore and experiment with different image combinations to create stunning visual compositions.
