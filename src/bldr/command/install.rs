// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Installs a bldr package from a [repo](../repo).
//!
//! # Examples
//!
//! ```bash
//! $ bldr install redis -u http://bldr.co:9633
//! ```
//!
//! Will install the `redis` package from the repository at `http://bldr.co:9633`.
//!
//! ```bash
//! $ bldr install redis -u http://bldr.co:9633 -d adam
//! ```
//!
//! Will do the same, but choose the `adam` origin, rather than the default `bldr`.
//!
//! ```bash
//! $ bldr install redis -u http://bldr.co:9633 -v 3.0.1
//! ```
//!
//! Will install the `3.0.1` version of redis.
//!
//! ```bash
//! $ bldr install redis -u http://bldr.co:9633 -v 3.0.1 -r 20150911204047
//! ```
//!
//! Will install the `20150911204047` release of the `3.0.1` version of `redis.
//!
//! ```bash
//! $ bldr install redis -u http://bldr.co:9633 -d adam -v 3.0.1 -r 20150911204047
//! ```
//!
//! The same as the last, but from the `adam` origin as well.
//!
//! # Internals
//!
//! * Download the artifact
//! * Verify it is un-altered
//! * Unpack it
//!

use std::fs;
use std::str::FromStr;

use fs::PACKAGE_CACHE;
use error::BldrResult;
use package::{Package, PackageIdent};
use repo::{self, data_object};

static LOGKEY: &'static str = "CI";

/// Given a package name and a base url, downloads the package
/// to `/opt/bldr/cache/pkgs`. Returns the filename in the cache as a String
///
/// # Failures
///
/// * Fails if it cannot create `/opt/bldr/cache/pkgs`
/// * Fails if it cannot download the package from the upstream
pub fn from_url(repo: &str, ident: &PackageIdent) -> BldrResult<data_object::Package> {
    let package = try!(repo::client::show_package(repo, ident));
    try!(fs::create_dir_all(PACKAGE_CACHE));
    let mut installed: Vec<PackageIdent> = vec![];
    for dep in &package.deps {
        let ident = try!(PackageIdent::from_str(&dep.to_string()));
        try!(install(repo, &ident, &mut installed));
    }
    try!(install(repo, &ident, &mut installed));
    Ok(package)
}

fn install(repo: &str, package: &PackageIdent, acc: &mut Vec<PackageIdent>) -> BldrResult<()> {
    if acc.contains(&package) {
        return Ok(());
    }
    let archive = try!(repo::client::fetch_package(repo, package, PACKAGE_CACHE));
    let package = try!(archive.ident());
    let deps = try!(archive.deps());
    try!(archive.unpack());
    outputln!("Installed {}", package);
    acc.push(package);
    for dep in deps {
        try!(install(repo, &dep, acc))
    }
    Ok(())
}
