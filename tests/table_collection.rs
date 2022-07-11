use tskit_rust_tutorial::tskit;

#[test]
fn simple_table_collection_creation_with_newtype() {
    #[allow(unused_mut)]
    {
        // ANCHOR: create_table_collection_with_newtype
        let sequence_length = tskit::Position::from(100.0);
        let mut tables = tskit::TableCollection::new(sequence_length).unwrap();
        // In tskit, the various newtypes can be compared to
        // the low-level types they wrap.
        assert_eq!(tables.sequence_length(), 100.0);
        // ANCHOR_END: create_table_collection_with_newtype
    }
}

#[test]
fn simple_table_collection_creation() {
    #[allow(unused_mut)]
    // ANCHOR: create_table_collection
    let mut tables = tskit::TableCollection::new(100.0).unwrap();
    // ANCHOR_END: create_table_collection
    assert_eq!(tables.sequence_length(), 100.0);
}

#[test]
fn add_node_without_metadata() {
    {
        let mut tables = tskit::TableCollection::new(100.0).unwrap();
        // ANCHOR: add_node_without_metadata
        let node_id = tables
            .add_node(
                0,                         // Node flags
                tskit::Time::from(0.0),    // Birth time
                tskit::PopulationId::NULL, // Population id
                tskit::IndividualId::NULL, // Individual id
            )
            .unwrap();
        assert_eq!(node_id, 0);
        // ANCHOR_END: add_node_without_metadata
    }
    {
        let mut tables = tskit::TableCollection::new(100.0).unwrap();
        // ANCHOR: add_node_without_metadata_using_into
        let node_id = tables.add_node(0, 0.0, -1, -1).unwrap();
        // ANCHOR_END: add_node_without_metadata_using_into
        assert_eq!(node_id, 0);
    }
}

#[test]
fn add_node_handle_error() {
    // ANCHOR: integrity_check
    let mut tables = tskit::TableCollection::new(100.0).unwrap();
    // Everything about this edge is wrong...
    tables.add_edge(-1.0, 11.0, 0, 1).unwrap();
    // ...and we can catch that here
    match tables.check_integrity(tskit::TableIntegrityCheckFlags::default()) {
        Ok(_) => (),
        // tskit::TskitError can be formatted into the same
        // error messages that tskit-c/tskit-python give.
        Err(e) => println!("{}", e),
    }
    // ANCHOR_END: integrity_check
    assert!(tables
        .check_integrity(tskit::TableIntegrityCheckFlags::default())
        .is_err());
}
