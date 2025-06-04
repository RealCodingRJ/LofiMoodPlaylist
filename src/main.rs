use std::{ fs::File, io::{ stdin, Write } };

fn get_url(url: String) {
    open::with(url, "chrome").unwrap();
}

fn main() {
    let mut mood = String::new();
    let mut is_appended: bool = false;
    let mut addedtofile = String::new();

    while mood != "Sad" {
        stdin().read_line(&mut mood).expect("No URL Defined");

        if mood.contains("Happy") {
            let mut url: String = String::new();

            println!("Enter URL: ");
            stdin().read_line(&mut url).expect("No URL Defined");

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
