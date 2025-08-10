mod mocks;
use std::time::{Duration, SystemTime};

use backend::model::history::History;
use mocks::history_service::history_service_mock;

#[test]
fn saving_history_creates_history_entries() {
    // given
    let mut history_service = history_service_mock(3);
    assert_eq!(history_service.repo.users.get(&1).unwrap().beer_count, 42);

    // when
    let histories = history_service.historize_drinks(&mut ());

    // then
    assert!(histories.is_ok());
    let histories: Vec<_> = histories.unwrap();

    assert_eq!(histories[0].beer_count, 42);
    assert_eq!(histories[0].shot_count, 43);
    assert_eq!(histories[0].other_count, 44);
    assert_eq!(histories[0].water_count, 45);

    assert_eq!(histories[1].beer_count, 42);
    assert_eq!(histories[1].shot_count, 43);
    assert_eq!(histories[1].other_count, 44);
    assert_eq!(histories[1].water_count, 45);

    assert_eq!(histories[2].beer_count, 42);
    assert_eq!(histories[2].shot_count, 43);
    assert_eq!(histories[2].other_count, 44);
    assert_eq!(histories[2].water_count, 45);
}

#[test]
fn saving_history_resets_drink_counts() {
    // given
    let mut history_service = history_service_mock(3);
    assert_eq!(history_service.repo.users.get(&1).unwrap().beer_count, 42);

    // when
    let _histories = history_service.historize_drinks(&mut ());

    // then
    assert_eq!(history_service.repo.users.get(&1).unwrap().beer_count, 0);
    assert_eq!(history_service.repo.users.get(&1).unwrap().shot_count, 0);
    assert_eq!(history_service.repo.users.get(&1).unwrap().other_count, 0);
    assert_eq!(history_service.repo.users.get(&1).unwrap().water_count, 0);

    assert_eq!(history_service.repo.users.get(&2).unwrap().beer_count, 0);
    assert_eq!(history_service.repo.users.get(&2).unwrap().shot_count, 0);
    assert_eq!(history_service.repo.users.get(&2).unwrap().other_count, 0);
    assert_eq!(history_service.repo.users.get(&2).unwrap().water_count, 0);

    assert_eq!(history_service.repo.users.get(&3).unwrap().beer_count, 0);
    assert_eq!(history_service.repo.users.get(&3).unwrap().shot_count, 0);
    assert_eq!(history_service.repo.users.get(&3).unwrap().other_count, 0);
    assert_eq!(history_service.repo.users.get(&3).unwrap().water_count, 0);
}

#[test]
fn save_history_from_csv() {
    // given
    // 2042_01_01 = 2272143600000;
    let history_csv = r#"history_id,user_name,timestamp,beer_count,shot_count,other_count,water_count1
    1,some-name,2272143600000,42,43,44,45
    2,other name,2272143600000,24,34,44,54"#;

    // when
    let mut history_service = history_service_mock(0);
    assert!(history_service.load_histories(&mut ()).unwrap().is_empty());
    let _inserted_histories = history_service.histories_from_csv(history_csv, &mut ());

    // then
    let histories = history_service.load_histories(&mut ()).unwrap();
    assert_eq!(histories.len(), 2);

    let expected_history = History {
        history_id: 1,
        user_name: "some-name".to_string(),
        timestamp: SystemTime::UNIX_EPOCH + Duration::from_millis(2_272_143_600_000),
        beer_count: 42,
        shot_count: 43,
        other_count: 44,
        water_count: 45,
    };
    assert_eq!(histories[0], expected_history);
}
