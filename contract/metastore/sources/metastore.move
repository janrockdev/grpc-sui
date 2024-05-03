// Inspired by:
// Copyright (c) 2022, Sui Foundation
// SPDX-License-Identifier: Apache-2.0

module sui_intro_unit_two::metastore {

    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};
    use sui::transfer;

    struct WrappableMetastore has key, store {
        id: UID,
        timestamp: vector<u8>,
        lineage_id: vector<u8>,
        hash: vector<u8>,
        derived_from: vector<u8>,
        status: vector<u8>
    }

    struct Folder has key {
        id: UID,
        metastore: WrappableMetastore,
        intended_address: address
    }

    struct AuditCap has key {
        id: UID
    }

    const ENotIntendedAddress: u64 = 1;

    fun init(ctx: &mut TxContext) {
        transfer::transfer(AuditCap {
            id: object::new(ctx)
        }, tx_context::sender(ctx))
    }

    public entry fun add_auditor(_: &AuditCap, new_auditor_address: address, ctx: &mut TxContext){
        transfer::transfer(
            AuditCap {
                id: object::new(ctx)
            },
        new_auditor_address
        )
    }
    
    public entry fun create_wrappable_metastore_object(_: &AuditCap, timestamp: vector<u8>, lineage_id: vector<u8>, hash: vector<u8>, derived_from: vector<u8>, status: vector<u8>, ctx: &mut TxContext) {
        let wrappableMetastore = WrappableMetastore {
            id: object::new(ctx),
            timestamp,
            lineage_id,
            hash,
            derived_from,
            status,
        };
        transfer::transfer(wrappableMetastore, tx_context::sender(ctx))
    }

    public entry fun view_score(metastoreObject: &WrappableMetastore): vector<u8>{
        metastoreObject.status
    }

    public entry fun update_score(_: &AuditCap, metastoreObject: &mut WrappableMetastore, state: vector<u8>){
        metastoreObject.status = state
    }

    public entry fun delete_metastore(_: &AuditCap, metastoreObject: WrappableMetastore){
        let WrappableMetastore {id, timestamp: _, lineage_id: _, hash: _, derived_from: _, status: _} = metastoreObject;
        object::delete(id);
    }

    public entry fun request_metastore(metastore: WrappableMetastore, intended_address: address, ctx: &mut TxContext){
        let folderObject = Folder {
            id: object::new(ctx),
            metastore,
            intended_address
        };
        transfer::transfer(folderObject, intended_address)
    }

    public entry fun unpack_wrapped_metastore(folder: Folder, ctx: &mut TxContext){
        assert!(folder.intended_address == tx_context::sender(ctx), ENotIntendedAddress);
        let Folder {
            id,
            metastore,
            intended_address:_,
        } = folder;
        transfer::transfer(metastore, tx_context::sender(ctx));
        object::delete(id)
    }

}
