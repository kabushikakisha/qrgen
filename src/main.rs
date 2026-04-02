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
    if input.is_empty() {
        return Err("Error: URL cannot be empty. Please provide a valid URL.".into());
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_validate_url_valid_https() {
        assert!(validate_url("https://example.com").is_ok());
    }

    #[test]
    fn test_validate_url_valid_http() {
        assert!(validate_url("http://example.com").is_ok());
    }

    #[test]
    fn test_validate_url_empty() {
        let result = validate_url("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("URL cannot be empty"));
    }

    #[test]
    fn test_validate_url_invalid_scheme() {
        let result = validate_url("ftp://example.com");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("unsupported URL scheme"));
    }

    #[test]
    fn test_validate_url_malformed() {
        let result = validate_url("not a url");
        assert!(result.is_err());
    }

    #[test]
    fn test_ensure_parent_dir_existing() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.png");
        assert!(ensure_parent_dir(&file_path).is_ok());
    }

    #[test]
    fn test_ensure_parent_dir_new_directory() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("subdir").join("test.png");
        assert!(ensure_parent_dir(&file_path).is_ok());
        assert!(file_path.parent().unwrap().exists());
    }

    #[test]
    fn test_args_parsing() {
        let args = Args::parse_from(["qrgen", "https://example.com"]);
        assert_eq!(args.url, "https://example.com");
        assert_eq!(args.output, PathBuf::from("qr.png"));
        assert_eq!(args.size, 512);
        assert_eq!(args.no_quiet_zone, false);
    }

    #[test]
    fn test_args_parsing_with_options() {
        let args = Args::parse_from([
            "qrgen",
            "https://example.com",
            "-o",
            "custom.png",
            "-s",
            "256",
            "--no-quiet-zone"
        ]);
        assert_eq!(args.url, "https://example.com");
        assert_eq!(args.output, PathBuf::from("custom.png"));
        assert_eq!(args.size, 256);
        assert_eq!(args.no_quiet_zone, true);
    }

    #[test]
    fn test_main_smoke() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test.png");

        let args = Args {
            url: "https://example.com".to_string(),
            output: output_path.clone(),
            size: 256,
            no_quiet_zone: false,
        };

        validate_url(&args.url)?;
        ensure_parent_dir(&args.output)?;

        let code = QrCode::new(args.url.as_bytes())?;
        let base_image = code.render::<Luma<u8>>()
            .quiet_zone(!args.no_quiet_zone)
            .build();

        let image = image::imageops::resize(&base_image, args.size, args.size, FilterType::Nearest);
        image.save(&args.output)?;

        assert!(output_path.exists());

        Ok(())
    }
}