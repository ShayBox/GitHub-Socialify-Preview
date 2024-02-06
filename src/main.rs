#![allow(clippy::multiple_crate_versions)]

use std::{fs::File, time::Duration};

use anyhow::Result;
use clap::{Parser, ValueEnum};
use config::{json::Value, ConfigFile};
use headless_chrome::{browser::default_executable, Browser, LaunchOptions};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Clone, Debug, Default, Display, ValueEnum)]
enum Theme {
    #[default]
    Light,
    Dark,
    Auto,
}

#[derive(Clone, Debug, Default, Display, ValueEnum)]
enum Font {
    #[default]
    Inter,
    Bitter,
    Raleway,
    Rokkitt,
    #[strum(to_string = "source+code+pro")]
    SourceCodePro,
    #[strum(to_string = "koho")]
    KoHo,
    Jost,
}

#[derive(Clone, Debug, Default, Display, ValueEnum)]
enum Pattern {
    Signal,
    #[strum(to_string = "charlie+brown")]
    CharlieBrown,
    #[strum(to_string = "formal+invitation")]
    FormalInvitation,
    #[default]
    Plus,
    #[strum(to_string = "circuit+board")]
    CircuitBoard,
    #[strum(to_string = "overlapping+hexagons")]
    OverlappingHexagons,
    #[strum(to_string = "brick+wall")]
    BrickWall,
    #[strum(to_string = "floating+cogs")]
    FloatingCogs,
    #[strum(to_string = "diagonal+stripes")]
    DiagonalStripes,
    Solid,
}

#[derive(Debug, Parser)]
#[allow(clippy::struct_excessive_bools)]
#[command(author, version, about)]
struct Args {
    /// GitHub Repository (Example: ShayBox/GitHub-Social-Preview)
    #[arg(short, long)]
    repo: String,

    /// Socialify Theme
    #[arg(short, long, value_enum)]
    theme: Option<Theme>,

    /// Socialify Font
    #[arg(short, long, value_enum)]
    font: Option<Font>,

    /// Socialify Background Pattern
    #[arg(short, long, value_enum)]
    pattern: Option<Pattern>,

    /// Socialify SVG Logo (Image URL or Data URI)
    #[arg(short, long)]
    logo: Option<String>,

    /// Socialify Show Owner Name
    #[arg(long, default_value_t = false)]
    owner: bool,

    /// Socialify Show Language
    #[arg(long, default_value_t = false)]
    language: bool,

    /// Socialify Show Stars Count
    #[arg(long, default_value_t = false)]
    stargazers: bool,

    /// Socialify Show Forks Count
    #[arg(long, default_value_t = false)]
    forks: bool,

    /// Socialify Show Issues Count
    #[arg(long, default_value_t = false)]
    issues: bool,

    /// Socialify Show Pull Requests Count
    #[arg(long, default_value_t = false)]
    pulls: bool,

    /// Socialify Show Description
    #[arg(short, long, default_value = None)]
    description: Option<String>,
}

#[derive(ConfigFile, Debug, Default, Deserialize, Serialize)]
struct Cookies(Value);

fn main() -> Result<()> {
    // Parse the args into a url with query parameters
    let args = Args::parse();
    let mut url = format!("https://socialify.git.ci/{}/png?", args.repo);

    if let Some(description) = args.description {
        url.push_str("&description=1");
        url.push_str(&format!("&descriptionEditable={description}"));
    }

    if let Some(font) = args.font {
        url.push_str(&format!("&font={font}"));
    };

    if args.forks {
        url.push_str("&forks=1");
    }

    if args.issues {
        url.push_str("&issues=1");
    }

    if args.language {
        url.push_str("&language=1");
    }

    if let Some(logo) = args.logo {
        url.push_str(&format!("&logo={logo}"));
    }

    if args.owner {
        url.push_str("&owner=1");
    }

    if let Some(pattern) = args.pattern {
        url.push_str(&format!("&pattern={pattern}"));
    };

    if args.pulls {
        url.push_str("&pulls=1");
    }

    if args.stargazers {
        url.push_str("&stargazers=1");
    }

    if let Some(theme) = args.theme {
        url.push_str(&format!("&theme={theme}"));
    };

    // Create a headful browser for preview and authentication
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(false)
            .idle_browser_timeout(Duration::MAX)
            .path(default_executable().ok())
            .build()?,
    )?;

    // Attempt to load previously saved cookies
    let mut cookies = Cookies::load().unwrap_or_default();
    let tab = browser.new_tab()?;
    if let Ok(cookies) = config::json::from_value(cookies.0) {
        tab.set_cookies(cookies)?;
    }

    // Attempt to navigate to the repo settings page
    tab.navigate_to(&format!("https://github.com/{}/settings", args.repo))?;
    tab.set_default_timeout(Duration::MAX);
    tab.set_file_chooser_dialog_interception(true)?;
    tab.wait_for_element("body.logged-in")?;

    // Attempt to download the Socialify repo social preview image
    let tempdir = std::env::temp_dir().join(args.repo.replace('/', "-") + ".png");
    let mut response = reqwest::blocking::get(url)?;
    let mut tempfile = File::create(&tempdir)?;
    response.copy_to(&mut tempfile)?;

    // Attempt to upload the Socialify repo social preview image
    let file = tab.wait_for_element("input[type='file']")?;
    tab.handle_file_chooser(vec![tempdir.to_string_lossy().into_owned()], file.node_id)?;

    // Attempt to save cookies for future use
    if let Ok(new_cookies) = tab.get_cookies() {
        cookies.0 = config::json::to_value(new_cookies)?;
        cookies.save()?;
    }

    // Wait 3 seconds for upload to finish
    std::thread::sleep(Duration::from_secs(3));

    Ok(())
}
