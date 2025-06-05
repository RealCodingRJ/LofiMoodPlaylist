use std::{ fs::File, io::{ stdin, Write } };

use mongodb::{ bson::Document, options::ClientOptions, Client, Collection };

fn get_url(url: String) {
    open::with(url, "chrome").unwrap();
}

#[tokio::main]
async fn connect_mongo(m: String) {
    let op = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = mongodb::Client::with_options(op)?;
    let message_code: Collection<Document> = client.database("URL").collection("URLS");
    let m = message_code.find_one(m.as_bytes());
    println!("Code: {}", message_code);

    Ok(());
}

fn main() {
    let message = "Lofi Playlist App";
    println!("\n{}", message);

    let mut mood = String::new();

    println!("Enter Message: ");
    stdin().read_line(&mut mood).expect("No URL Defined");
    let mut is_appended: bool = false;
    let mut addedtofile = String::new();

    while !is_appended {
        if mood.contains("Happy") {
            let mut url: String = String::new();

            println!("Enter URL: ");
            stdin().read_line(&mut url).expect("No URL Defined");

            connect_mongo(url);

            println!("Want to Add to File: ");
            stdin().read_line(&mut addedtofile).expect("No URL Defined");

            if addedtofile == "Y" || addedtofile == "y" {
                if !is_appended {
                    get_url(url.clone());

                    let mut lofi_file = File::create("lofi.io.history").unwrap();
                    let _ = lofi_file.write(url.as_bytes());

                    break;
                }
            }

            is_appended = true;
        }
    }
}
