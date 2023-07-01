pub mod rpkm66 {
    pub mod auth {
        pub mod auth {
            pub mod v1 {
                include!("rpkm66.auth.auth.v1.rs");
            }
        }
    }
    pub mod backend {
        pub mod baan {
            pub mod v1 {
                include!("rpkm66.backend.baan.v1.rs");
            }
        }
        pub mod checkin {
            pub mod v1 {
                include!("rpkm66.backend.checkin.v1.rs");
            }
        }
        pub mod event {
            pub mod v1 {
                include!("rpkm66.backend.event.v1.rs");
            }
        }
        pub mod group {
            pub mod v1 {
                include!("rpkm66.backend.group.v1.rs");
            }
        }
        pub mod user {
            pub mod v1 {
                include!("rpkm66.backend.user.v1.rs");
            }
        }
    }
    pub mod file {
        pub mod file {
            pub mod v1 {
                include!("rpkm66.file.file.v1.rs");
            }
        }
    }
}
