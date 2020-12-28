use std::path;

/// Reference to a specific asset
// Right now assets can't be mixed with one another, since polymorphic interactions in Rust are _difficult_ (for good reason! raw pointer casting is dangerous)
// But eventually the plan is to unify them.
// Asset references should be able to be constructed from a path and converted back into that path (with a little help)
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct AssetReference {
    asset_location: String,
}

impl AssetReference {
    fn into(self, mut asset_root: path::PathBuf) -> path::PathBuf {
        asset_root.push(self.asset_location);
        asset_root
    }
}

impl From<&'static str> for AssetReference {
    fn from(loc: &'static str) -> Self {
        AssetReference::from(loc.to_string())
    }
}

impl From<String> for AssetReference {
    fn from(loc: String) -> Self {
        AssetReference {
            asset_location: loc
        }
    }
}