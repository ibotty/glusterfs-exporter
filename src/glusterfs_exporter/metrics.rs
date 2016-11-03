use prometheus::*;

lazy_static! {
    pub static ref EXPORTER_FAILURE_COUNTER: Counter = register_counter!("glusterfs_failures", "Failures internal to glusterfs_exporter").unwrap();

    pub static ref PROFILE_OP_GAUGE: GaugeVec = register_gauge_vec!("glusterfs_profile_ops", "Glusterfs volume profile Ops", &["volume"]).unwrap();
    pub static ref BLOCKSTATS_COUNTER: GaugeVec = register_gauge_vec!("glusterfs_blockstats", "Glusterfs volume block stats", &["volume", "brick", "optype", "size_bucket"]).unwrap();
    pub static ref BLOCKSTATS_TOTAL_COUNTER: GaugeVec = register_gauge_vec!("glusterfs_blockstats_total", "Glusterfs volume total block stats", &["volume", "brick", "optype"]).unwrap();
    pub static ref FOPSTATS_AVGLATENCY: GaugeVec = register_gauge_vec!("glusterfs_fop_avglatency", "Glusterfs volume file operation avg latency", &["volume", "brick", "fop_name"]).unwrap();
    pub static ref FOPSTATS_MINLATENCY: GaugeVec = register_gauge_vec!("glusterfs_fop_minlatency", "Glusterfs volume file operation min latency", &["volume", "brick", "fop_name"]).unwrap();
    pub static ref FOPSTATS_MAXLATENCY: GaugeVec = register_gauge_vec!("glusterfs_fop_maxlatency", "Glusterfs volume file operation max latency", &["volume", "brick", "fop_name"]).unwrap();
    pub static ref FOPSTATS_COUNTER: GaugeVec = register_gauge_vec!("glusterfs_fop_hits", "Glusterfs volume file operation hits", &["volume", "brick", "fop_name"]).unwrap();
}
