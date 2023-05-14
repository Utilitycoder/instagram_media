use std::fs;
use std::io::Write;
use std::path::Path;

use instagram_private_api::{Client, MediaOrAd};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = "instagram";
    let folder_name = "instagram_images";
    let client = Client::new()?;

    // Get user ID of the given username
    let user_id = client.username_info(username).await?.user.pk;

    // Create folder for the images
    let folder_path = Path::new(".").join(folder_name);
    fs::create_dir_all(&folder_path)?;

    // Fetch all media posts of the user
    let mut max_id = None;
    loop {
        let feed = client.get_user_feed(user_id, max_id).await?;
        if feed.items.is_empty() {
            break;
        }
        for item in feed.items {
            if let MediaOrAd::Media(media) = item.media_or_ad {
                if let Some(image_url) = media.image_versions2.as_ref().and_then(|v| v.candidates.get(0)).map(|c| c.url.as_str()) {
                    // Download the image and save it to the folder
                    let file_name = format!("{}.jpg", media.pk);
                    let file_path = folder_path.join(&file_name);
                    let response = reqwest::get(image_url).await?;
                    let mut file = fs::File::create(&file_path)?;
                    file.write_all(&response.bytes().await?)?;
                }
            }
        }
        max_id = Some(feed.next_max_id);
    }

    Ok(())
}
