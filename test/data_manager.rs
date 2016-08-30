use expectest::prelude::{be_ok, be_some};

use sql::data_manager::DataManager;

#[test]
fn saves_to_one_row_table() {
    let data_manger = DataManager::default();

    expect!(data_manger.save_to("table_name", vec!["1"]))
        .to(be_ok());
}

#[test]
fn retrievs_data_from_table() {
    let data_manager = DataManager::default();

    drop(data_manager.save_to("table_name", vec!["1", "2"]));
    drop(data_manager.save_to("table_name", vec!["3", "4"]));

    expect!(data_manager.get_row_from("table_name", 0))
        .to(be_some().value(vec!["1", "2"]));
    expect!(data_manager.get_row_from("table_name", 1))
        .to(be_some().value(vec!["3", "4"]));
}

#[test]
fn retrives_range_of_rows_from_table() {
    let data_manager = DataManager::default();

    drop(data_manager.save_to("table_name", vec!["1", "2", "3"]));
    drop(data_manager.save_to("table_name", vec!["4", "5", "6"]));
    drop(data_manager.save_to("table_name", vec!["7", "8", "9"]));
    drop(data_manager.save_to("table_name", vec!["10", "11", "12"]));
    drop(data_manager.save_to("table_name", vec!["13", "14", "15"]));

    expect!(data_manager.get_range("table_name", 1, 3))
        .to(be_some().value(
            vec![
                vec!["4", "5", "6"],
                vec!["7", "8", "9"],
                vec!["10", "11", "12"]
            ]
        ));

    expect!(data_manager.get_range("table_name", 2, 2))
        .to(be_some().value(
            vec![
                vec!["7", "8", "9"],
                vec!["10", "11", "12"]
            ]
        ));
}