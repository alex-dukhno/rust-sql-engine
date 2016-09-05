use expectest::prelude::be_equal_to;

use sql::data_manager::{DataManager, LockBaseDataManager};

#[test]
fn saves_to_one_row_table() {
    let data_manger = LockBaseDataManager::create();

    drop(data_manger.save_to("table_name", vec!["1"]));

    expect!(data_manger.get_range_till_end("table_name", 0))
        .to(be_equal_to(vec![vec!["1"]]));
}

#[test]
fn retrievs_data_from_table() {
    let data_manager = LockBaseDataManager::create();

    drop(data_manager.save_to("table_name", vec!["1", "2"]));
    drop(data_manager.save_to("table_name", vec!["3", "4"]));

    expect!(data_manager.get_row_from("table_name", 0))
        .to(be_equal_to(vec!["1", "2"]));
    expect!(data_manager.get_row_from("table_name", 1))
        .to(be_equal_to(vec!["3", "4"]));
}

#[test]
fn retrievs_range_of_rows_from_table() {
    let data_manager = LockBaseDataManager::create();

    drop(data_manager.save_to("table_name", vec!["1", "2", "3"]));
    drop(data_manager.save_to("table_name", vec!["4", "5", "6"]));
    drop(data_manager.save_to("table_name", vec!["7", "8", "9"]));
    drop(data_manager.save_to("table_name", vec!["10", "11", "12"]));
    drop(data_manager.save_to("table_name", vec!["13", "14", "15"]));

    expect!(data_manager.get_range("table_name", 1, 3))
        .to(be_equal_to(
            vec![
                vec!["4", "5", "6"],
                vec!["7", "8", "9"],
                vec!["10", "11", "12"]
            ]
        ));

    expect!(data_manager.get_range("table_name", 2, 2))
        .to(be_equal_to(
            vec![
                vec!["7", "8", "9"],
                vec!["10", "11", "12"]
            ]
        ));
}

#[test]
fn retrievs_range_from_index_till_end() {
    let data_manager = LockBaseDataManager::create();

    drop(data_manager.save_to("table_name", vec!["1", "2", "3"]));
    drop(data_manager.save_to("table_name", vec!["4", "5", "6"]));
    drop(data_manager.save_to("table_name", vec!["7", "8", "9"]));
    drop(data_manager.save_to("table_name", vec!["10", "11", "12"]));
    drop(data_manager.save_to("table_name", vec!["13", "14", "15"]));

    expect!(data_manager.get_range_till_end("table_name", 2))
        .to(be_equal_to(
            vec![
                vec!["7", "8", "9"],
                vec!["10", "11", "12"],
                vec!["13", "14", "15"]
            ]
        ));
}