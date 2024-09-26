# image-to-bytes-converter
convert image to C bytes array for spi/i2c display

## resize an image
use https://ezgif.com/

using ffmpeg:
```sh
ffmpeg -i image.gif -vf scale=WIDTH:HEIGHT blaziken.gif
```

## convert a gif/video to bytes array

```sh
mkdir temp && ffmpeg -i image.gif -vsync 0 temp/temp%d.png
```

```sh
cargo run -- temp/temp%d.png
```