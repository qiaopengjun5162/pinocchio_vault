// @generated
pub mod pinocchio_vault {
    // @@protoc_insertion_point(attribute:pinocchio_vault.v1)
    pub mod v1 {
        include!("pinocchio_vault.v1.rs");
        // @@protoc_insertion_point(pinocchio_vault.v1)
    }
}
pub mod sf {
    pub mod solana {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.solana.type.v1)
            pub mod v1 {
                include!("sf.solana.type.v1.rs");
                // @@protoc_insertion_point(sf.solana.type.v1)
            }
        }
    }
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod sink {
            pub mod database {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.database.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.database.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.database.v1)
                }
            }
        }
        pub mod solana {
            // @@protoc_insertion_point(attribute:sf.substreams.solana.v1)
            pub mod v1 {
                include!("sf.substreams.solana.v1.rs");
                // @@protoc_insertion_point(sf.substreams.solana.v1)
            }
        }
    }
}
