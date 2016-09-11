use expectest::prelude::be_equal_to;

use sql::data_manager::LockBaseDataManager;

#[test]
fn saves_to_one_row_table() {
    let data_manger = LockBaseDataManager::default();

    drop(data_manger.save_to("table_name", vec!["1".to_owned()]));

    expect!(data_manger.get_range_till_end("table_name", 0))
        .to(be_equal_to(vec![vec!["1"]]));
}

#[test]
fn retrievs_data_from_table() {
    let data_manager = LockBaseDataManager::default();

    drop(data_manager.save_to("table_name", vec!["1".to_owned(), "2".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["3".to_owned(), "4".to_owned()]));

    expect!(data_manager.get_row_from("table_name", 0))
        .to(be_equal_to(vec!["1", "2"]));
    expect!(data_manager.get_row_from("table_name", 1))
        .to(be_equal_to(vec!["3", "4"]));
}

#[test]
fn retrievs_range_of_rows_from_table() {
    let data_manager = LockBaseDataManager::default();

    drop(data_manager.save_to("table_name", vec!["1".to_owned(), "2".to_owned(), "3".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["4".to_owned(), "5".to_owned(), "6".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["7".to_owned(), "8".to_owned(), "9".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["10".to_owned(), "11".to_owned(), "12".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["13".to_owned(), "14".to_owned(), "15".to_owned()]));

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
    let data_manager = LockBaseDataManager::default();

    drop(data_manager.save_to("table_name", vec!["1".to_owned(), "2".to_owned(), "3".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["4".to_owned(), "5".to_owned(), "6".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["7".to_owned(), "8".to_owned(), "9".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["10".to_owned(), "11".to_owned(), "12".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["13".to_owned(), "14".to_owned(), "15".to_owned()]));

    expect!(data_manager.get_range_till_end("table_name", 2))
        .to(be_equal_to(
            vec![
                vec!["7", "8", "9"],
                vec!["10", "11", "12"],
                vec!["13", "14", "15"]
            ]
        ));
}

#[test]
fn retrievs_by_not_equal_predicate_on_column() {
    let data_manager = LockBaseDataManager::default();

    drop(data_manager.save_to("table_name", vec!["10".to_owned(), "11".to_owned(), "12".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["1".to_owned(), "2".to_owned(), "3".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["7".to_owned(), "8".to_owned(), "9".to_owned()]));

    expect!(data_manager.get_not_equal("table_name", 0, &("1".to_owned())))
        .to(be_equal_to(
            vec![
                vec!["10", "11", "12"],
                vec!["7", "8", "9"]
            ]
        ));
}

#[test]
fn retrives_by_column_index() {
    let data_manager = LockBaseDataManager::default();

    drop(data_manager.save_to("table_name", vec!["10".to_owned(), "11".to_owned(), "12".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["1".to_owned(), "2".to_owned(), "3".to_owned()]));
    drop(data_manager.save_to("table_name", vec!["7".to_owned(), "8".to_owned(), "9".to_owned()]));

    expect!(data_manager.get_range_till_end_for_column("table_name", 0))
        .to(be_equal_to(
            vec![
                vec!["10"],
                vec!["1"],
                vec!["7"]
            ]
        ));
}