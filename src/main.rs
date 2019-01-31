use crates_index;
use std::collections::HashSet;

fn mirror_crate(src_crate: &str, dest_crate: &str, version: &str) {
    println!("Mirroring {} {} -> {} {}", src_crate, version, dest_crate, version);
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
    let src_crate = index.crates().find(|c| c.name() == SRC_CRATE)
        .expect("The crate we're trying to mirror does not exist?");
    let dest_crate = index.crates().find(|c| c.name() == DEST_CRATE);

    let src_versions_to_mirror = if let Some(existing_dest) = dest_crate {
        println!("Dest crate exists, filtering out known versions");
        // O(n^2) is just fine if n is small, honest o/`
        // Fiiiiine, it's simpler to do it right anyway.
        let src_version_set: HashSet<&str> = src_crate.versions().iter().map(|v| v.version()).collect();
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

    src_versions_to_mirror.iter()
        .for_each(|v| mirror_crate(&SRC_CRATE, &DEST_CRATE, v));
}
