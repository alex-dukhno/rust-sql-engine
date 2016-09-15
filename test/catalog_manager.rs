use expectest::prelude::{be_true, be_false, be_some, be_equal_to};

use sql::parser::ast::Type;
use sql::catalog_manager::LockBasedCatalogManager;

#[test]
fn adds_table_to_catalog_manger() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table("table_name");

    expect!(catalog_manager.contains_table("table_name"))
        .to(be_true());
}

#[test]
fn does_not_contain_table_that_was_not_add() {
    let catalog_manager = LockBasedCatalogManager::default();

    expect!(catalog_manager.contains_table("table"))
        .to(be_false());
}

#[test]
fn adds_column_to_table() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table("table");

    catalog_manager.add_column_to("table", ("col", Type::Int, None));

    expect!(catalog_manager.contains_column_in("table", "col"))
        .to(be_true());
}

#[test]
fn does_not_contain_column_that_was_not_add() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table("table");

    expect!(catalog_manager.contains_column_in("table", "col"))
        .to(be_false());
}

#[test]
fn column_matches_type() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table("table");

    catalog_manager.add_column_to("table", ("col", Type::Int, None));

    expect!(catalog_manager.match_type("table", 0, Type::Int))
        .to(be_true());
}

#[test]
fn column_does_not_match_type() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table("table");

    catalog_manager.add_column_to("table", ("col", Type::VarChar(10), None));

    expect!(catalog_manager.match_type("table", 0, Type::Int))
        .to(be_false());
}


#[test]
fn get_table_columns() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table("table");

    catalog_manager.add_column_to("table", ("col_1", Type::Int, None));
    catalog_manager.add_column_to("table", ("col_2", Type::Int, None));
    catalog_manager.add_column_to("table", ("col_3", Type::Int, None));

    expect!(catalog_manager.get_table_columns("table"))
        .to(be_equal_to(vec![("col_1".to_owned(), (None, Type::Int)), ("col_2".to_owned(), (None, Type::Int)), ("col_3".to_owned(), (None, Type::Int))]));
}

#[test]
fn gets_column_index_by_name() {
    let catalog_manager = LockBasedCatalogManager::default();

    catalog_manager.add_table("table");

    catalog_manager.add_column_to("table", ("col_1", Type::Int, None));
    catalog_manager.add_column_to("table", ("col_2", Type::Int, None));
    catalog_manager.add_column_to("table", ("col_3", Type::Int, None));

    expect!(catalog_manager.get_column_index("table", "col_1")).to(be_some().value(0));
    expect!(catalog_manager.get_column_index("table", "col_2")).to(be_some().value(1));
    expect!(catalog_manager.get_column_index("table", "col_3")).to(be_some().value(2));
}
