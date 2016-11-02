# Prometheus glusterfs exporter

This exporter is meant to be started next to a gluster peer (not neccessarily
serving bricks!).

It will only work with volumes that have profiling enabled.

     gluster volume profile <volname> start

The exporter will _not_ set up profiling, but you can run the following
command to enable profiling on all volumes.

     gluster volume list | xargs -I {} -n1 gluster volume profile {} start


## Building `glusterfs_exporter`

Can be done with cargo and a recent-ish rust.

```
cargo install
```
