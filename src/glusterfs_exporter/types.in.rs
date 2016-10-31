#[derive(Deserialize, Debug)]
#[serde(rename="cliOutput")]
pub struct VolumeProfileInfo {
    #[serde(rename="opErrno")]
    errno: u8,
    #[serde(rename="opErrstr")]
    errstr: String,
    volname: String,
    #[serde(rename="brick")]
    bricks: Vec<BrickStats>,
}

#[derive(Deserialize, Debug)]
pub struct BrickStats {
    #[serde(rename="brickName")]
    brick_name: String,
}
