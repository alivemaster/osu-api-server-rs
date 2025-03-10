use regex::Regex;
use std::error::Error;

pub async fn cache_api_assets(url: &str) -> Result<String, Box<dyn Error>> {
    let url_pattern = Regex::new(
        r"^https?://(?<sub_domain>a|assets|osu)\.ppy\.sh/(?:(?<category>[a-z]+(?:(?:-|/)[a-z]+)*)/)?(?:(?<item_id>[0-9]+)(?:/|\?))?(?:[a-z]+/)?(?<file_name>.+)$",
    ).unwrap();
    let url_captures = match url_pattern.captures(&url) {
        Some(captures) => captures,
        None => return Err("url doesn't match!".into()),
    };

    let sub_domain = match url_captures.name("sub_domain") {
        Some(value) => value.as_str(),
        None => "",
    };
    let item_id = match url_captures.name("item_id") {
        Some(value) => value.as_str(),
        None => "",
    };
    let category = match url_captures.name("category") {
        Some(value) => value.as_str(),
        None => "",
    };
    let file_name = match url_captures.name("file_name") {
        Some(value) => value.as_str(),
        None => "",
    };
    let mut file_name = file_name.to_owned();

    // determine the directory to save file
    let mut file_dir = String::from("cache/");
    if sub_domain == "a" {
        file_dir = file_dir + "users/avatar/";
    } else if category == "user-profile-covers" {
        file_dir = file_dir + "users/profile_cover/";
    } else if category == "user-cover-presets" {
        file_dir = file_dir + "users/profile_cover/presets/";
    } else {
        file_dir = file_dir + &category + "/";
    }
    file_dir = if item_id != "" {
        file_dir + &item_id + "/"
    } else {
        file_dir
    };

    // rename beatmap assets
    if category == "beatmaps" {
        let filename_pattern = Regex::new(
            r"^(?<name>[a-z]+(?:@2x)?)(?<ext>\.(?:jpeg|jpg|png|webp))\?(?<cache_id>[0-9]+)$",
        )
        .unwrap();
        let captures = filename_pattern.captures(&file_name);
        if let Some(captures) = captures {
            file_name =
                captures["name"].to_owned() + "_" + &captures["cache_id"] + &captures["ext"];
        }
    }
    // "@2x" -> "_2x"
    file_name = file_name.replace("@", "_");

    let exe_path = std::env::current_exe().unwrap();
    let self_dir = exe_path
        .parent()
        .unwrap();
    let absolute_dir = self_dir.join(&file_dir);

    if !absolute_dir.exists() {
        std::fs::create_dir_all(&absolute_dir)?;
    }
    let file_path = absolute_dir.join(&file_name);

    if !file_path.exists() {
        super::download_file(&url, &file_path).await?;
    }

    let url_path = file_dir.replace("cache", "assets") + &file_name;
    Ok(url_path)
}
