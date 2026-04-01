use clap::Parser;
use image::imageops::FilterType;
use image::Luma;
use qrcode::QrCode;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Parser)]
#[command(name = "qrgen")]
#[command(about = "Generate a QR code PNG from a URL.")]
#[command(after_help = "EXAMPLES:
    qrgen 'https://example.com'
        Generate a QR code with default settings.

    qrgen 'https://example.com' -o[utput] myqr.png -s[ize] 256
        Generate a 256x256 QR code and save it as myqr.png.

    qrgen 'https://example.com' --no-quiet-zone
        Generate a QR code without the standard quiet zone.")]
struct Args {
    url: String,

    #[arg(short, long, default_value = "qr.png")]
    output: PathBuf,

    #[arg(short, long, default_value_t = 512)]
    size: u32,

    #[arg(long)]
    no_quiet_zone: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    validate_url(&args.url)?;

    if args.size == 0 {
        return Err("image size must be greater than zero".into());
    }

    ensure_parent_dir(&args.output)?;

    let code = QrCode::new(args.url.as_bytes())?;
    let base_image = code.render::<Luma<u8>>()
        .quiet_zone(!args.no_quiet_zone)
        .build();

    let image = image::imageops::resize(&base_image, args.size, args.size, FilterType::Nearest);
    image.save(&args.output)?;

    println!("Wrote QR code to {}", args.output.display());
    Ok(())
}

fn validate_url(input: &str) -> Result<(), Box<dyn Error>> {
    let parsed = Url::parse(input)?;

    match parsed.scheme() {
        "http" | "https" => Ok(()),
        scheme => Err(format!(
            "unsupported URL scheme `{scheme}`; use http or https"
        )
        .into()),
    }
}

fn ensure_parent_dir(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    Ok(())
}
