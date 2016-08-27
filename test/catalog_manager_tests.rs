use expectest::prelude::{be_true, be_false};

use sql::parser::ast::Type;
use sql::catalog_manager::{CatalogManager, LockBasedCatalogManager, Table, Column};

#[test]
fn add_table_to_catalog_manger() {
    let catalog_manager: LockBasedCatalogManager = CatalogManager::create();

    let table = Table::new("table_name");

    catalog_manager.add_table(table);

    expect!(catalog_manager.contains_table("table_name"))
        .to(be_true());
}

#[test]
fn does_not_contain_table_that_was_not_add() {
    let catalog_manager: LockBasedCatalogManager = CatalogManager::create();

    expect!(catalog_manager.contains_table("table"))
        .to(be_false());
}

#[test]
fn add_column_to_table() {
    let catalog_manager: LockBasedCatalogManager = CatalogManager::create();

    let table = Table::new("table");

    catalog_manager.add_table(table);

    catalog_manager.add_column_to("table", Column::new("col", Type::Int));

    expect!(catalog_manager.contains_column_in("table", "col"))
        .to(be_true());
}

#[test]
fn does_not_contain_column_that_was_not_add() {
    let catalog_manager: LockBasedCatalogManager = CatalogManager::create();

    let table = Table::new("table");

    catalog_manager.add_table(table);

    expect!(catalog_manager.contains_column_in("table", "col"))
        .to(be_false());
}

#[test]
fn column_matches_type() {
    let catalog_manager: LockBasedCatalogManager = CatalogManager::create();

    let table = Table::new("table");

    catalog_manager.add_table(table);

    catalog_manager.add_column_to("table", Column::new("col", Type::Int));

    expect!(catalog_manager.match_type("table", 0, Type::Int))
        .to(be_true());
}

#[test]
fn column_does_not_match_type() {
    let catalog_manager: LockBasedCatalogManager = CatalogManager::create();

    let table = Table::new("table");

    catalog_manager.add_table(table);

    catalog_manager.add_column_to("table", Column::new("col", Type::Varchar));

    expect!(catalog_manager.match_type("table", 0, Type::Int))
        .to(be_false());
}
