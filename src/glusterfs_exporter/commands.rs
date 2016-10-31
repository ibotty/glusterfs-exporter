extern crate serde;
extern crate serde_xml;

use glusterfs_exporter::errors::*;
use glusterfs_exporter::types::*;
use std::process::{Command, Output};
use std::io::Read;
use std::str;

fn fail_if_not_successful(cmd_string: &str, cmd: &Output) -> Result<(), GEError> {
    if !cmd.status.success() {
        err!("Failed to run command {} with exitcode {}.",
             cmd_string,
             cmd.status)
    } else {
        Ok(())
    }
}

pub fn gluster_volumes() -> Result<Vec<String>, GEError> {
    let cmd = try!(Command::new("gluster")
        .arg("volume")
        .arg("list")
        .output());
    try!(fail_if_not_successful("volume list", &cmd));
    let out = try!(str::from_utf8(&cmd.stdout));
    Ok(out.lines().map(str::to_string).collect())
}

// This assumes valid xml output as of Glusterfs 3.8
// In detail, it will fail if e.g. a <fop> element appears not inside a <brick>'s
// <cumulativeStats>.
pub fn gluster_vol_info(vol: String) -> Result<VolumeProfileInfo, GEError> {
    let cmd = try!(Command::new("gluster")
        .arg("volume")
        .arg("profile")
        .arg(vol)
        .arg("info")
        .arg("cumulative")
        .arg("--xml")
        .spawn());
    let stdout = try!(cmd.stdout.ok_or("Can't capture stdout"));
    serde_xml::de::from_iter(stdout.bytes()).map_err(From::from)
}
