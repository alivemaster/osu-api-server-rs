use regex::Regex;
use std::error::Error;

pub async fn cache_api_assets(url: &str) -> Result<String, Box<dyn Error>> {
    let url_pattern = Regex::new(
        r"^https?://(?<sub_domain>a|assets|osu)\.ppy\.sh/(?:(?<sub_dir>[a-z0-9]+(?:(?:-|/|\?)[a-z0-9]+)*)(?:/|\?))(?<file_name>.+)$",
    ).unwrap();
    let url_captures = match url_pattern.captures(&url) {
        Some(captures) => captures,
        None => return Err("url doesn't match!".into()),
    };

    let sub_domain = &url_captures["sub_domain"];
    let mut sub_dir = url_captures["sub_dir"].to_owned();
    let mut file_name = url_captures["file_name"].to_owned();

    // determine the directory to save file
    if sub_domain == "a" {
        sub_dir = "users/avatar/".to_owned() + &sub_dir;
    }
    sub_dir = sub_dir.replace("user-profile-covers", "users/profile_cover");
    sub_dir = sub_dir.replace("user-cover-presets", "users/profile_cover/presets");
    sub_dir = sub_dir.replace("profile-badges", "users/profile_badges");
    if Regex::new("^beatmap")
        .unwrap()
        .is_match(&sub_dir)
    {
        sub_dir = sub_dir.replace("/covers", "");
        // rename file
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
    let file_dir = String::from("cache/") + &sub_dir + "/";

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

    let url_path = "/".to_owned() + &file_dir.replace("cache", "assets") + &file_name;
    Ok(url_path)
}
