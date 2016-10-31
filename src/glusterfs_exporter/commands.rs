extern crate xml;

use glusterfs_exporter::errors::*;

use std::str;
use std::process::{Command, Output};

use self::xml::attribute::{OwnedAttribute};
use self::xml::name::{OwnedName};
use self::xml::reader::{EventReader, XmlEvent};

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
pub fn gluster_vol_info(vol: String) -> Result<(), GEError> {
    let cmd = try!(Command::new("gluster")
        .arg("volume")
        .arg("profile")
        .arg(vol)
        .arg("info")
        .arg("cumulative")
        .arg("--xml")
        .spawn());
    let stdout = try!(cmd.stdout.ok_or("Can't capture stdout"));
    let parser = EventReader::new(stdout);
    // let cur_stat = MaybeStat(None, None, None);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement{name : OwnedName{ local_name, ..}, attributes, ..}) => {
                info!("parsing elem {}.", local_name);
                match local_name.as_ref() {
                    "brickName" => {
                        // cur_stat = Some(value);
                        debug!("setting brickName to {}", local_name);
                    },
                    _ => info!("somewhere else")
                }
            },
            Ok(XmlEvent::EndElement{name : OwnedName{ local_name, ..}}) => {
                info!("ending elem {}.", local_name);
                match local_name.as_ref() {
                    "brick" => debug!("closing brick."),
                    _ => info!("somewhere else!")
                }
            },
            Ok(_) => break,
            Err(e) => {
                // err!("Can't parse xml output: {}.", e);
                break;
            }
        }
    }
    Ok(())
}
