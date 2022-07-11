use tskit_rust_tutorial::tskit;

#[test]
fn test_simple_table_collection_creation() {
    #[allow(unused_mut)]
    // ANCHOR: create_table_collection
    let mut tables = tskit::TableCollection::new(100.0).unwrap();
    assert_eq!(tables.sequence_length(), 100.0);
    // ANCHOR_END: create_table_collection
}

#[test]
fn test_add_node_without_metadata() {
    {
        // ANCHOR: add_node_without_metadata
        let mut tables = tskit::TableCollection::new(100.0).unwrap();
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
