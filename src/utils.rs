//! Private library utilities that are not exposed to clients since we don't
//! want to make semver guarantees about them

/// Private helper to walk a dir and also follow symbolic links.
#[cfg(any(feature = "parsing", feature = "plist-load"))]
pub fn walk_dir<P: AsRef<std::path::Path>>(folder: P) -> walkdir::WalkDir {
    walkdir::WalkDir::new(folder).follow_links(true)
}

#[cfg(all(test, feature = "parsing"))]
pub mod testdata {
    use std::sync::LazyLock;

    use crate::parsing::SyntaxSet;

    /// The [`SyntaxSet`] loaded from the `testdata/Packages` folder
    ///
    /// Shared here to avoid re-doing a particularly costly construction in various tests
    pub static PACKAGES_SYN_SET: LazyLock<SyntaxSet> =
        LazyLock::new(|| SyntaxSet::load_from_folder("testdata/Packages").unwrap());
}
