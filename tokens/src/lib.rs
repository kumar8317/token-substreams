use substreams::errors::Error;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_entity_change::pb::entity::EntityChanges;


#[substreams::handlers::map]
fn graph_out(
    erc20_entities: EntityChanges,
    erc721_entities: EntityChanges,
    erc1155_entities: EntityChanges,
    collection_owner_entities: EntityChanges,
) -> Result<EntityChanges, Error>{
    Ok(EntityChanges { 
        entity_changes:[
            erc20_entities.entity_changes,
            erc721_entities.entity_changes,
            erc1155_entities.entity_changes,
            collection_owner_entities.entity_changes
        ]
        .concat()
    })
}

#[substreams::handlers::map]
fn db_out(
    erc20_db: DatabaseChanges,
    erc721_db: DatabaseChanges,
    erc1155_db: DatabaseChanges,
    collection_owner_db: DatabaseChanges,
) -> Result<DatabaseChanges, Error>{
    Ok(DatabaseChanges { 
        table_changes:[
            erc20_db.table_changes,
            erc721_db.table_changes,
            erc1155_db.table_changes,
            collection_owner_db.table_changes
        ]
        .concat()
    })
}