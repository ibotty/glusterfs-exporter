extern crate serde;
extern crate serde_xml;

use glusterfs_exporter::errors::*;
use glusterfs_exporter::types::*;
use glusterfs_exporter::metrics::*;
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
pub fn gluster_vol_info(vol: &str) -> Result<VolProfile, GEError> {
    let cmd = try!(Command::new("gluster")
        .arg("volume")
        .arg("profile")
        .arg(vol)
        .arg("info")
        .arg("cumulative")
        .arg("--xml")
        .spawn());
    let stdout = try!(cmd.stdout.ok_or("Can't capture stdout"));
    serde_xml::de::from_iter(stdout.bytes())
        .map_err(From::from)
        .map(|cli: CliOutput| cli.vol_profile)
}

pub fn collect_stats() -> Result<(), GEError> {
    for vol in try!(gluster_volumes()).iter() {
        info!("collection profile info for volume {}.", vol);
        let VolProfile{profile_op, bricks, ..} = try!(gluster_vol_info(vol));
        PROFILE_OP_GAUGE.with_label_values(&[vol]).set(profile_op as f64);

        for Brick{brick_name, cumulative_stats} in bricks {
            let brick = &brick_name;
            debug!("collecting info for brick {}.", brick_name);
            BLOCKSTATS_TOTAL_COUNTER.with_label_values(&[vol, brick, "read"]).set(cumulative_stats.total_read as f64);
            BLOCKSTATS_TOTAL_COUNTER.with_label_values(&[vol, brick, "write"]).set(cumulative_stats.total_write as f64);
            for blockstat in cumulative_stats.block_stats.blocks {
                let size_bucket = &format!("{}", blockstat.size);
                BLOCKSTATS_COUNTER.with_label_values(&[vol, brick, "read", size_bucket]).set(blockstat.reads as f64);
                BLOCKSTATS_COUNTER.with_label_values(&[vol, brick, "write", size_bucket]).set(blockstat.writes as f64);
            }
            for fopstat in cumulative_stats.fop_stats.fops {
                let fop_name = &fopstat.name;
                FOPSTATS_AVGLATENCY.with_label_values(&[vol, brick, fop_name]).set(fopstat.avg_latency);
                FOPSTATS_MINLATENCY.with_label_values(&[vol, brick, fop_name]).set(fopstat.min_latency);
                FOPSTATS_MAXLATENCY.with_label_values(&[vol, brick, fop_name]).set(fopstat.max_latency);
                FOPSTATS_COUNTER.with_label_values(&[vol, brick, fop_name]).set(fopstat.hits as f64);
            }
        }
    }
    Ok(())
}
