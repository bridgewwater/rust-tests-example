// use serde_derive::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Directories
///
/// use attribute #[serde(skip)] to skip member
#[skip_serializing_none]
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Directories {
    /// doc
    ///
    #[serde(default)]
    pub doc: String,
}

/// Scripts
///
/// use attribute #[serde(skip)] to skip member
#[skip_serializing_none]
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Scripts {
    /// test
    ///
    #[serde(default)]
    pub test: String,
}

/// Repository
///
/// use attribute #[serde(skip)] to skip member
#[skip_serializing_none]
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    /// r#type
    ///
    #[serde(default)]
    pub r#type: String,
    /// url
    ///
    #[serde(default)]
    pub url: String,
}

/// Author
/// @see https://serde.rs/enum-representations.html#adjacently-tagged
/// use attribute #[serde(skip)] to skip member
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Author {
    Name(String),
    Full {
        /// name
        ///
        #[serde(default)]
        name: String,
        /// email
        ///
        #[serde(default)]
        email: String,
        /// url
        ///
        #[serde(default)]
        url: String,
    },
}

// /// for [Author] default value
// /// @see https://serde.rs/attr-default.html#default-value-for-a-field
// fn default_author() -> Author {
//     Author::Name(String::default())
// }

impl Default for Author {
    #[cfg(not(tarpaulin_include))]
    fn default() -> Self {
        Author::Name(String::default())
    }
}

/// Bugs
///
/// use attribute #[serde(skip)] to skip member
#[skip_serializing_none]
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Bugs {
    /// url
    ///
    #[serde(default)]
    pub url: String,
}

/// NpmConfigDefine
///
/// use attribute #[serde(skip)] to skip member
#[skip_serializing_none]
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NpmConfigDefine {
    /// name
    ///
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub name: String,
    /// version
    ///
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub version: String,
    /// display_name
    ///
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(
    rename(serialize = "displayName", deserialize = "displayName"),
    default
    )]
    pub display_name: String,
    /// description
    ///
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub description: String,
    /// directories
    ///
    #[serde(default)]
    pub directories: Option<Directories>,
    /// scripts
    ///
    #[serde(default)]
    pub scripts: Option<Scripts>,
    /// repository
    ///
    #[serde(default)]
    pub repository: Option<Repository>,
    /// keywords
    ///
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub keywords: Vec<String>,
    /// author
    ///
    #[serde(default)]
    pub author: Option<Author>,
    /// license
    ///
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub license: String,
    /// bugs
    ///
    #[serde(default)]
    pub bugs: Option<Bugs>,
    /// homepage
    ///
    #[serde(skip_serializing_if = "String::is_empty")]
    #[serde(default)]
    pub homepage: String,
}

use anyhow::{anyhow, Result};

fn load_package_json(pkg_json_bytes: &[u8]) -> Result<NpmConfigDefine> {
    let pkg_json_str = String::from_utf8_lossy(pkg_json_bytes);
    // println!("pkg_json_str #{pkg_json_str}");
    let pkg_define: NpmConfigDefine = match serde_json::from_str(&pkg_json_str)
    {
        Err(why) => return Err(anyhow!("couldn't parse package.json: {}\nfrom json:\n{}\n", why, pkg_json_str)),
        Ok(pkg_define) => pkg_define,
    };
    Ok(pkg_define)
}

fn find_cli_package_json_version(pkg_json_bytes: &[u8]) -> Result<String> {
    let pkg_json_str = String::from_utf8_lossy(pkg_json_bytes);
    // println!("pkg_json_str #{pkg_json_str}");
    let pkg_define: NpmConfigDefine = match serde_json::from_str(&pkg_json_str)
    {
        Err(why) => return Err(anyhow!("couldn't parse package.json: {}\nfrom json:\n{}\n", why, pkg_json_str)),
        Ok(pkg_define) => pkg_define,
    };
    Ok(pkg_define.version)
}

#[cfg(test)]
mod test_npm_config_define {
    use super::*;

    #[test]
    fn test_load_package_json() {
        let pkg_json = r###"{
    "version": "0.1.2"
}"###.to_string();
        let pkg_json_bytes = pkg_json.as_bytes();
        let pkg_define = match load_package_json(pkg_json_bytes) {
            Ok(res) => { res }
            Err(why) => { panic!("{}", why) }
        };
        let npm_config_define = pkg_define;
        assert!(npm_config_define.name.is_empty());
        assert_eq!(npm_config_define.version, "0.1.2");
        println!("pkg_define {}", serde_json::to_string(&npm_config_define).unwrap());
    }

    #[test]
    fn test_find_cli_package_json_version() {
        let pkg_json = r###"{
    "version": "0.1.2",
    "author": "mine"
}"###.to_string();
        let pkg_json_bytes = pkg_json.as_bytes();
        let version = find_cli_package_json_version(pkg_json_bytes).unwrap();
        println!("find cli version {}", version);
        assert_eq!(version, "0.1.2");
    }
}

