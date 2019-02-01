use crates_index;
use flate2::bufread::GzDecoder;
use reqwest;
use tar;
use toml_edit;
use std::collections::HashSet;
use std::fs;
use std::path;

const WORK_DIR: &str = "_work";

fn crate_file_path(crate_name: &str, version: &str) -> String {
    format!("{}/{}-{}.crate", WORK_DIR, crate_name, version)
}

fn crate_dir_path(crate_name: &str, version: &str) -> String {
    format!("{}/{}-{}", WORK_DIR, crate_name, version)
}

/// Actually downloads the given crate.
fn fetch_crate(crate_name: &str, version: &str) {
    assert_ne!(crate_name, "", "Crate name must not be an empty string!");
    use reqwest::header::*;
    let mut headers = HeaderMap::new();
    const USER_AGENT_STR: &str = "isildur (https://crates.io/crates/isildur)";
    headers.insert(USER_AGENT, USER_AGENT_STR.parse().unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Could not build HTTP client?");

    let url_string = format!(
        "https://static.crates.io/crates/{}/{}-{}.crate",
        crate_name, crate_name, version
    );
    let url = reqwest::Url::parse(&url_string).expect("Invalid URL!");

    let mut resp = client
        .get(url)
        .send()
        .expect("Could not send HTTP request?");

    fs::create_dir_all(WORK_DIR).expect("Could not create work dir?");
    let crate_file_path = crate_file_path(crate_name, version);
    let crate_file =
        &mut fs::File::create(crate_file_path).expect("Could not open output crate file?");

    let byte_count = resp.copy_to(crate_file)
        .expect("Could not write crate to output file?");
    println!(
        "    downloaded {}, {} kb written.",
        url_string,
        (byte_count / 1024) + 1
    );
}

fn extract_crate(src_crate: &str, version: &str) {
    let crate_path = crate_file_path(src_crate, version);
    use std::io;
    let in_stream =
        io::BufReader::new(fs::File::open(crate_path).expect("Could not read crate file?"));
    let gz_stream = GzDecoder::new(in_stream);
    let mut archive = tar::Archive::new(gz_stream);
    archive
        .unpack(WORK_DIR)
        .expect("Could not unpack crate archive.");
}

fn fiddle_cargo_toml(src_crate: &str, dest_crate: &str, version: &str) {
    // oh noez we have to actually use real paths now ;_;
    let mut cargo_toml_path = path::PathBuf::from(crate_dir_path(src_crate, version));
    cargo_toml_path.push("Cargo.toml");
    {
        let contents = fs::read_to_string(&cargo_toml_path).expect("Could not read cargo.toml!");
        // toml_edit might not be the best tool for this but it works.
        let mut doc = contents
            .parse::<toml_edit::Document>()
            .expect("Invalid toml!");
        doc["package"]["name"] = toml_edit::value(dest_crate);

        let desc_str = doc["package"]["description"]
            .as_str()
            .expect("Package description is not a string???");
        let modified_desc_str = format!("Automated mirror of {} - {}", src_crate, desc_str);
        doc["package"]["description"] = toml_edit::value(modified_desc_str);
        let new_cargo_toml_contents = doc.to_string();

        // Actually write output
        fs::write(cargo_toml_path, new_cargo_toml_contents.as_bytes())
            .expect("Couldn't write to cargo.toml?");
    }
}

/// Prepend our disclaimer to the README.md file of the crate, creating
/// it if necessary.
fn fiddle_readme(src_crate: &str, dest_crate: &str, version: &str) {
    let mut disclaimer_string = format!(r#"
# {dest} - a republish of {src}

This crate is, apart from the name, an exact duplicate of {src}.  It has been produced by an automatic
tool to work around some inconvenience in the upstream crate.

For more information see <https://crates.io/crates/isildur>.

Original README.md file follows:

"#, src=src_crate, dest=dest_crate);

    let crate_dir = crate_dir_path(src_crate, version);
    let mut cargo_toml_path = path::PathBuf::from(crate_dir.clone());
    cargo_toml_path.push("Cargo.toml");
    let contents = fs::read_to_string(&cargo_toml_path).expect("Could not read cargo.toml!");
    // toml_edit might not be the best tool for this but it works.
    let doc = contents
        .parse::<toml_edit::Document>()
        .expect("Invalid toml!");
    let readme_file = doc["package"]["readme"].as_str().unwrap_or("README.md");
    let mut readme_file_path = path::PathBuf::from(&crate_dir);
    readme_file_path.push(readme_file);

    // Output readme with our disclaimer attached.
    // Need to make sure we create the output dir for silly reasons.
    // The readme.md may be in a subdir of `crate_dir` but not actually
    // exist in the crate file.
    let readme_dir = readme_file_path.parent().unwrap_or(path::Path::new(&crate_dir));
    fs::create_dir_all(readme_dir)
        .expect(&format!("Could not create output dir for readme file {:?}", readme_file_path));
    let existing_readme = fs::read_to_string(&readme_file_path).unwrap_or(String::from("No readme file included in crate."));
    disclaimer_string.push_str(&existing_readme);
    fs::write(&readme_file_path, disclaimer_string.as_bytes())
        .expect(&format!("Couldn't write to readme file {:?}", &readme_file_path));
}

fn mirror_crate(src_crate: &str, dest_crate: &str, version: &str) {
    println!(
        "Mirroring {} {} -> {} {}",
        src_crate, version, dest_crate, version
    );
    println!("  Grabbing src crate file");
    fetch_crate(src_crate, version);
    println!("  Heckin' unzipping it");
    extract_crate(src_crate, version);
    println!("  Fiddling name and readme");
    fiddle_cargo_toml(src_crate, dest_crate, version);
    fiddle_readme(src_crate, dest_crate, version);
    println!("  Publishing...");
    println!("  Done!");
}

fn main() {
    const SRC_CRATE: &str = "ring";
    const DEST_CRATE: &str = "gnir";
    const CRATE_INDEX_DIR: &str = "_index";

    let index = crates_index::Index::new(CRATE_INDEX_DIR);
    println!("Fetching crate index...");

    index
        .retrieve_or_update()
        .expect("Could not fetch/update crate index.");
    let src_crate = index
        .crates()
        .find(|c| c.name() == SRC_CRATE)
        .expect("The crate we're trying to mirror does not exist?");
    let dest_crate = index.crates().find(|c| c.name() == DEST_CRATE);

    let src_versions_to_mirror = if let Some(existing_dest) = dest_crate {
        println!("Dest crate exists, filtering out known versions");
        // O(n^2) is just fine if n is small, honest o/`
        // Fiiiiine, it's simpler to do it right anyway.
        let src_version_set: HashSet<&str> =
            src_crate.versions().iter().map(|v| v.version()).collect();
        let versions_to_mirror: Vec<String> = existing_dest.versions().iter()
            .map(|dest_version| dest_version.version()) // We just need the string.
            .filter(|dest_version_str| src_version_set.contains(dest_version_str))
        // If we collect to Vec<&str> then we can't return it 'cause
        // all the &str's point into `existing_dest`, which is dropped
        // at the end of this scope.  We COULD fiddle the order of things
            // to make the ownership work, orrrrrrrrr...
            .map(|v| v.to_owned())
            .collect();
        versions_to_mirror
    } else {
        println!("Dest crate does not exist, mirroring all src crate versions");
        src_crate.versions().iter()
            .map(|v| v.version()) // Just get the string
            .map(|v| v.to_owned())
            .collect()
    };

    src_versions_to_mirror.iter().for_each(|v| {
        mirror_crate(&SRC_CRATE, &DEST_CRATE, v);
        // Sleep for a sec so we don't slam crates.io too hard
        // unlikely, but still polite.
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
}
