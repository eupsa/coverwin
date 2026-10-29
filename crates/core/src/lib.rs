use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use image::codecs::{avif::AvifEncoder, jpeg::JpegEncoder};
use image::{DynamicImage, ImageFormat};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("formato não suportado: {0}")]
    UnsupportedFormat(String),
    #[error(transparent)]
    Image(#[from] image::ImageError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("falha ao codificar WebP: {0}")]
    Webp(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Png,
    Jpg,
    Webp,
    Avif,
    Bmp,
    Gif,
    Tiff,
    Ico,
}

impl Format {
    pub const ALL: [Format; 8] = [
        Format::Png,
        Format::Jpg,
        Format::Webp,
        Format::Avif,
        Format::Bmp,
        Format::Gif,
        Format::Tiff,
        Format::Ico,
    ];

    pub fn extension(self) -> &'static str {
        match self {
            Format::Png => "png",
            Format::Jpg => "jpg",
            Format::Webp => "webp",
            Format::Avif => "avif",
            Format::Bmp => "bmp",
            Format::Gif => "gif",
            Format::Tiff => "tiff",
            Format::Ico => "ico",
        }
    }
}

impl std::str::FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s.to_ascii_lowercase().as_str() {
            "jpeg" => Ok(Format::Jpg),
            "tif" => Ok(Format::Tiff),
            other => Format::ALL
                .into_iter()
                .find(|f| f.extension() == other)
                .ok_or_else(|| Error::UnsupportedFormat(s.to_string())),
        }
    }
}

/// Converte `input` para `format`, salvando ao lado do original. Nunca sobrescreve: usa "nome (1).ext" se preciso.
/// `quality` (1–100) vale para JPG, WebP e AVIF.
pub fn convert(input: &Path, format: Format, quality: u8) -> Result<PathBuf, Error> {
    let img = image::open(input)?;
    let output = free_path(&input.with_extension(format.extension()));
    let quality = quality.clamp(1, 100);

    match format {
        Format::Jpg => {
            // JPG não tem transparência
            let rgb = DynamicImage::ImageRgb8(img.to_rgb8());
            rgb.write_with_encoder(JpegEncoder::new_with_quality(create(&output)?, quality))?;
        }
        Format::Webp => {
            // o encoder do crate `image` só faz WebP lossless
            let rgba = img.to_rgba8();
            let data = webp::Encoder::from_rgba(&rgba, rgba.width(), rgba.height())
                .encode_simple(false, quality as f32)
                .map_err(|e| Error::Webp(format!("{e:?}")))?;
            std::fs::write(&output, &*data)?;
        }
        Format::Avif => {
            img.write_with_encoder(AvifEncoder::new_with_speed_quality(
                create(&output)?,
                6,
                quality,
            ))?;
        }
        Format::Ico if img.width() > 256 || img.height() > 256 => {
            img.thumbnail(256, 256)
                .save_with_format(&output, ImageFormat::Ico)?;
        }
        _ => {
            let f = ImageFormat::from_extension(format.extension()).expect("extensão conhecida");
            img.save_with_format(&output, f)?;
        }
    }
    Ok(output)
}

fn create(path: &Path) -> Result<BufWriter<File>, Error> {
    Ok(BufWriter::new(File::create(path)?))
}

fn free_path(path: &Path) -> PathBuf {
    if !path.exists() {
        return path.to_path_buf();
    }
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let ext = path.extension().unwrap_or_default().to_string_lossy();
    (1..)
        .map(|i| path.with_file_name(format!("{stem} ({i}).{ext}")))
        .find(|p| !p.exists())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_to_every_format_without_overwriting() {
        let dir = std::env::temp_dir().join("coverwin-test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let src = dir.join("a.png");
        image::RgbaImage::from_pixel(300, 200, image::Rgba([200, 50, 50, 128]))
            .save(&src)
            .unwrap();

        for f in Format::ALL {
            let out = convert(&src, f, 80).unwrap();
            assert!(out.exists(), "{f:?}");
            assert_eq!(image::open(&out).is_ok(), f != Format::Avif, "{f:?}"); // decode de AVIF não está habilitado
        }
        // a.png e a (1).png (do loop) já existem → a (2).png
        assert_eq!(
            convert(&src, Format::Png, 80).unwrap(),
            dir.join("a (2).png")
        );
        assert!("JPEG".parse::<Format>().is_ok() && "xyz".parse::<Format>().is_err());
    }
}
