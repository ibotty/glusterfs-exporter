#[derive(Deserialize, Debug)]
#[serde(rename="cliOutput")]
pub struct CliOutput {
    #[serde(rename="opRet")]
    pub ret: u8,

    #[serde(rename="opErrno")]
    pub errno: u8,

    // This is a workaround for https://github.com/serde-rs/xml/issues/32
    #[serde(rename="opErrstr")]
    errstr: serde_xml::value::Element,

    #[serde(rename="volProfile")]
    pub vol_profile: VolProfile,
}

#[derive(Deserialize, Debug)]
pub struct VolProfile {
    // pub volname: VolName,
    pub volname: String,

    #[serde(rename="profileOp")]
    pub profile_op: u64,

    #[serde(rename="brick")]
    pub bricks: Vec<Brick>,
}

#[derive(Deserialize, Debug)]
pub struct Brick {
    #[serde(rename="brickName")]
    pub brick_name: String,

    #[serde(rename="cumulativeStats")]
    pub cumulative_stats: CumulativeStats,
}

#[derive(Deserialize, Debug)]
pub struct CumulativeStats {
    #[serde(rename="blockStats")]
    pub block_stats: BlockStats,

    #[serde(rename="fopStats")]
    pub fop_stats: FopStats,

    pub duration: u64,

    #[serde(rename="totalRead")]
    pub total_read: u64,

    #[serde(rename="totalWrite")]
    pub total_write: u64
}

#[derive(Deserialize, Debug)]
pub struct BlockStats {
    #[serde(rename="block")]
    pub blocks: Vec<BlockStat>,
}

#[derive(Deserialize, Debug)]
pub struct BlockStat {
    pub size: u64,
    pub reads: u64,
    pub writes: u64,
}

#[derive(Deserialize, Debug)]
pub struct FopStats {
    #[serde(rename="fop")]
    pub fops: Vec<FopStat>,
}

#[derive(Deserialize, Debug)]
pub struct FopStat {
    pub name: String,
    pub hits: u64,

    #[serde(rename="avgLatency")]
    pub avg_latency: f64,

    #[serde(rename="minLatency")]
    pub min_latency: f64,

    #[serde(rename="maxLatency")]
    pub max_latency: f64,
}
