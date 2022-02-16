pub mod helloworld {
    pub mod v1 {
        include!("helloworld.v1.rs");
    }
}
pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
}
