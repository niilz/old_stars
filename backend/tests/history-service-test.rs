mod mocks;
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
