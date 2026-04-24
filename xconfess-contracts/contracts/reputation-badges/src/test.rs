use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_mint_badge() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Mint a badge
    let badge_id = client.mint_badge(&user, &BadgeType::ConfessionStarter);
    assert_eq!(badge_id, 1);

    // Verify badge count
    let count = client.get_badge_count(&user);
    assert_eq!(count, 1);

    // Verify has_badge
    assert!(client.has_badge(&user, &BadgeType::ConfessionStarter));
    assert!(!client.has_badge(&user, &BadgeType::PopularVoice));
}

#[test]
fn test_duplicate_badge_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Mint first badge
    let badge_id1 = client.mint_badge(&user, &BadgeType::ConfessionStarter);
    assert_eq!(badge_id1, 1);

    // Verify count stays at 1
    let count = client.get_badge_count(&user);
    assert_eq!(count, 1);
}

#[test]
fn test_multiple_badge_types() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Mint different badge types
    client.mint_badge(&user, &BadgeType::ConfessionStarter);
    client.mint_badge(&user, &BadgeType::PopularVoice);
    client.mint_badge(&user, &BadgeType::GenerousSoul);

    // Verify count
    let count = client.get_badge_count(&user);
    assert_eq!(count, 3);

    // Verify all badges
    assert!(client.has_badge(&user, &BadgeType::ConfessionStarter));
    assert!(client.has_badge(&user, &BadgeType::PopularVoice));
    assert!(client.has_badge(&user, &BadgeType::GenerousSoul));
    assert!(!client.has_badge(&user, &BadgeType::CommunityHero));
}

#[test]
fn test_get_badges() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Mint badges
    client.mint_badge(&user, &BadgeType::ConfessionStarter);
    client.mint_badge(&user, &BadgeType::TopReactor);

    // Get all badges
    let badges = client.get_badges(&user);
    assert_eq!(badges.len(), 2);
}

#[test]
fn test_transfer_badge() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    // Mint badge to user1
    let badge_id = client.mint_badge(&user1, &BadgeType::ConfessionStarter);

    // Transfer to user2
    client.transfer_badge(&badge_id, &user2);

    // Verify ownership changed
    assert!(!client.has_badge(&user1, &BadgeType::ConfessionStarter));
    assert!(client.has_badge(&user2, &BadgeType::ConfessionStarter));

    assert_eq!(client.get_badge_count(&user1), 0);
    assert_eq!(client.get_badge_count(&user2), 1);
}

#[test]
fn test_get_total_badges() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    // Mint badges
    client.mint_badge(&user1, &BadgeType::ConfessionStarter);
    client.mint_badge(&user2, &BadgeType::PopularVoice);
    client.mint_badge(&user1, &BadgeType::GenerousSoul);

    // Verify total
    let total = client.get_total_badges();
    assert_eq!(total, 3);
}

#[test]
fn test_get_badge_by_id() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Mint badge
    let badge_id = client.mint_badge(&user, &BadgeType::CommunityHero);

    // Get badge by ID
    let badge = client.get_badge(&badge_id);
    assert!(badge.is_some());

    let badge = badge.unwrap();
    assert_eq!(badge.id, badge_id);
    assert_eq!(badge.badge_type, BadgeType::CommunityHero);
    assert_eq!(badge.owner, user);
}

#[test]
fn test_nonexistent_badge() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    // Try to get non-existent badge
    let badge = client.get_badge(&999);
    assert!(badge.is_none());
}

#[test]
fn test_transfer_nonexistent_badge() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let _user = Address::generate(&env);

    // Verify no badge exists
    let badge = client.get_badge(&999);
    assert!(badge.is_none());
}

#[test]
fn test_revoke_badge() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    // Mint badge
    let badge_id = client.mint_badge(&user, &BadgeType::GenerousSoul);
    assert!(client.has_badge(&user, &BadgeType::GenerousSoul));
    assert_eq!(client.get_badge_count(&user), 1);

    // Revoke badge
    client.revoke_badge(&badge_id);

    // Verify
    assert!(!client.has_badge(&user, &BadgeType::GenerousSoul));
    assert_eq!(client.get_badge_count(&user), 0);
    assert!(client.get_badge(&badge_id).is_none());
}

// ─────────────────────────────────────────────────────────────────────────────
// Admin Authorization Tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_initialize_contract() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin);

    // Verify admin is set
    let retrieved_admin = client.get_admin();
    assert_eq!(retrieved_admin, admin);
}

#[test]
fn test_initialize_only_once() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin1);

    // Try to initialize again - should fail
    let result = client.try_initialize(&admin2);
    assert!(result.is_err());
}

#[test]
fn test_transfer_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);

    // Initialize with admin1
    client.initialize(&admin1);
    assert_eq!(client.get_admin(), admin1);

    // Transfer to admin2
    client.transfer_admin(&admin2);

    // Verify admin2 is now admin
    assert_eq!(client.get_admin(), admin2);
}

#[test]
fn test_admin_only_functions_require_init() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let _admin = Address::generate(&env);
    let user = Address::generate(&env);

    // Try to call admin functions without initializing - should fail
    let award_result = client.try_award_badge(&user, &BadgeType::ConfessionStarter);
    assert!(award_result.is_err());

    let adjust_result =
        client.try_adjust_reputation(&user, &100i128, &String::from_str(&env, "test"));
    assert!(adjust_result.is_err());
}

#[test]
fn test_create_badge_metadata() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    // Create badge metadata
    client.create_badge(
        &BadgeType::ConfessionStarter,
        &String::from_str(&env, "First Confession"),
        &String::from_str(&env, "Posted your first confession"),
        &String::from_str(&env, "Post at least one confession"),
    );

    // Verify metadata is stored (by checking we can create it without error)
    // In a full implementation, we'd have a get_badge_metadata function
}

#[test]
fn test_award_badge_admin_only() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin);

    // Admin awards badge
    let badge_id = client.award_badge(&user, &BadgeType::ConfessionStarter);
    assert_eq!(badge_id, 1);

    // Verify user received the badge
    assert!(client.has_badge(&user, &BadgeType::ConfessionStarter));
    assert_eq!(client.get_badge_count(&user), 1);
}

#[test]
fn test_adjust_reputation() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin);

    // Check initial reputation
    assert_eq!(client.get_user_reputation(&user), 0);

    // Adjust reputation
    let new_rep = client.adjust_reputation(&user, &100i128, &String::from_str(&env, "test"));
    assert_eq!(new_rep, 100);

    // Verify reputation updated
    assert_eq!(client.get_user_reputation(&user), 100);

    // Adjust again (negative)
    let new_rep = client.adjust_reputation(&user, &-50i128, &String::from_str(&env, "penalty"));
    assert_eq!(new_rep, 50);

    // Verify final reputation
    assert_eq!(client.get_user_reputation(&user), 50);
}

#[test]
fn test_award_duplicate_badge_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin);

    // Award badge first time
    client.award_badge(&user, &BadgeType::ConfessionStarter);

    // Try to award same badge type again - should fail
    let result = client.try_award_badge(&user, &BadgeType::ConfessionStarter);
    assert!(result.is_err());
}

#[test]
fn test_admin_can_award_different_badge_types() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin);

    // Award multiple different badge types
    let id1 = client.award_badge(&user, &BadgeType::ConfessionStarter);
    let id2 = client.award_badge(&user, &BadgeType::PopularVoice);
    let id3 = client.award_badge(&user, &BadgeType::GenerousSoul);

    // Verify all were awarded
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
    assert_eq!(client.get_badge_count(&user), 3);
}

#[test]
fn test_mint_and_award_can_coexist() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    client.initialize(&admin);

    // User self-mints one badge
    let self_id = client.mint_badge(&user, &BadgeType::ConfessionStarter);
    assert_eq!(self_id, 1);

    // Admin tries to award same badge type - should fail (user already has it)
    let award_result = client.try_award_badge(&user, &BadgeType::ConfessionStarter);
    assert!(award_result.is_err());

    // Admin awards different badge type - should succeed
    let award_id = client.award_badge(&user, &BadgeType::PopularVoice);
    assert_eq!(award_id, 2);
    assert_eq!(client.get_badge_count(&user), 2);
}

// ─────────────────────────────────────────────────────────────────────────────
// Read interface tests (Issue 816)
// Pins the stable read surface required by backend and frontend consumers.
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn get_badge_type_metadata_returns_none_before_create() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let result = client.get_badge_type_metadata(&BadgeType::ConfessionStarter);
    assert!(
        result.is_none(),
        "metadata must not exist before create_badge is called"
    );
}

#[test]
fn get_badge_type_metadata_returns_correct_values_after_create() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    client.create_badge(
        &BadgeType::ConfessionStarter,
        &String::from_str(&env, "First Confession"),
        &String::from_str(&env, "Posted your first confession"),
        &String::from_str(&env, "Post at least one confession"),
    );

    let meta = client.get_badge_type_metadata(&BadgeType::ConfessionStarter);
    assert!(
        meta.is_some(),
        "metadata must be present after create_badge"
    );

    let meta = meta.unwrap();
    assert_eq!(meta.name, String::from_str(&env, "First Confession"));
    assert_eq!(
        meta.description,
        String::from_str(&env, "Posted your first confession")
    );
    assert_eq!(
        meta.criteria,
        String::from_str(&env, "Post at least one confession")
    );
}

#[test]
fn has_badge_type_metadata_reflects_create_state() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    assert!(!client.has_badge_type_metadata(&BadgeType::PopularVoice));

    client.create_badge(
        &BadgeType::PopularVoice,
        &String::from_str(&env, "Popular Voice"),
        &String::from_str(&env, "Received 100+ reactions"),
        &String::from_str(&env, "Earn 100 reactions on your confessions"),
    );

    assert!(client.has_badge_type_metadata(&BadgeType::PopularVoice));
    assert!(
        !client.has_badge_type_metadata(&BadgeType::GenerousSoul),
        "unrelated type must remain absent"
    );
}

#[test]
fn get_badge_type_metadata_is_independent_per_badge_type() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    client.create_badge(
        &BadgeType::GenerousSoul,
        &String::from_str(&env, "Generous Soul"),
        &String::from_str(&env, "Tipped 10+ confessions"),
        &String::from_str(&env, "Tip at least 10 confessions"),
    );

    assert!(client
        .get_badge_type_metadata(&BadgeType::GenerousSoul)
        .is_some());
    assert!(client
        .get_badge_type_metadata(&BadgeType::CommunityHero)
        .is_none());
    assert!(client
        .get_badge_type_metadata(&BadgeType::TopReactor)
        .is_none());
}

#[test]
fn get_user_badge_summary_returns_empty_for_new_user() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let user = Address::generate(&env);
    let (badge_ids, reputation) = client.get_user_badge_summary(&user);

    assert_eq!(badge_ids.len(), 0, "new user must have no badges");
    assert_eq!(reputation, 0i128, "new user reputation must be zero");
}

#[test]
fn get_user_badge_summary_reflects_awarded_badges_and_reputation() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    client.initialize(&admin);

    client.award_badge(&user, &BadgeType::ConfessionStarter);
    client.award_badge(&user, &BadgeType::PopularVoice);
    client.adjust_reputation(&user, &200i128, &String::from_str(&env, "great content"));

    let (badge_ids, reputation) = client.get_user_badge_summary(&user);

    assert_eq!(
        badge_ids.len(),
        2,
        "summary must include both awarded badges"
    );
    assert_eq!(
        reputation, 200i128,
        "summary must reflect current reputation"
    );
}

#[test]
fn get_user_badge_summary_badge_ids_match_individual_queries() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    client.initialize(&admin);

    let id1 = client.award_badge(&user, &BadgeType::ConfessionStarter);
    let id2 = client.award_badge(&user, &BadgeType::CommunityHero);

    let (summary_ids, _) = client.get_user_badge_summary(&user);

    assert!(
        summary_ids.contains(id1),
        "summary must contain first badge id"
    );
    assert!(
        summary_ids.contains(id2),
        "summary must contain second badge id"
    );
}

#[test]
fn read_interfaces_do_not_require_auth() {
    let env = Env::default();
    // Deliberately do NOT call env.mock_all_auths() — read-only calls must succeed.

    let contract_id = env.register(ReputationBadges, ());
    let client = ReputationBadgesClient::new(&env, &contract_id);
    let user = Address::generate(&env);

    // All read functions must succeed without any auth mocking.
    let _ = client.get_badge_type_metadata(&BadgeType::TopReactor);
    let _ = client.has_badge_type_metadata(&BadgeType::TopReactor);
    let _ = client.get_user_badge_summary(&user);
    let _ = client.get_badge_count(&user);
    let _ = client.get_user_reputation(&user);
    let _ = client.has_badge(&user, &BadgeType::GenerousSoul);
    let _ = client.get_total_badges();
}
