# lau-achievements

> Badge and achievement system for the PLATO/Lau learning game — 30+ built-in badges across 7 categories, with per-player tracking.

## What This Does

This crate provides a complete achievement system for educational games. Define badges with tiers (Bronze → Legendary), categories (Building, Learning, Conservation, etc.), and emoji icons. Track which players have earned what, filter by player/category/tier, and prevent duplicate awards. Ships with 30+ pre-built badges ready to use.

## The Key Idea

Gamification through collectible badges. Each badge is a `Badge` definition (id, name, description, emoji, tier, category). When a player earns one, an `Achievement` record is created linking the badge to the player and the game tick. The `AchievementTracker` manages the full lifecycle — registration, earning, querying — and prevents double-awards.

## Install

```bash
cargo add lau-achievements
```

## Quick Start

```rust
use lau_achievements::*;

// Use the built-in badge set (30+ badges)
let badges = default_badges();
let mut tracker = AchievementTracker::with_badges(badges);

// Alice earns her first badge
let achievement = tracker.earn("first-block", "alice", 100).unwrap();
println!("{} earned {}! {}", achievement.earned_by, achievement.badge.name, achievement.badge.icon);
// Output: alice earned First Block! 🧱

// Can't earn the same badge twice
assert!(tracker.earn("first-block", "alice", 200).is_none());

// Bob earns badges too
tracker.earn("first-block", "bob", 150);
tracker.earn("architect", "bob", 500);

// Query by player
let bob_badges = tracker.badges_by_player("bob");
assert_eq!(bob_badges.len(), 2);

// Query by category
let building = tracker.badges_by_category(BadgeCategory::Building);

// Count by tier
let bob_bronze = tracker.player_tier_count("bob", BadgeTier::Bronze);
```

## API Reference

### `Badge`
A badge definition. Fields: `id: String`, `name: String`, `description: String`, `icon: String` (emoji), `tier: BadgeTier`, `category: BadgeCategory`.

### `BadgeTier`
Badge rarity: `Bronze`, `Silver`, `Gold`, `Diamond`, `Legendary`.

### `BadgeCategory`
Badge domain: `Building`, `Learning`, `Conservation`, `Social`, `Exploration`, `Teaching`, `GitMastery`.

### `Achievement`
An earned badge instance. Fields: `badge: Badge`, `earned_at_tick: u64`, `earned_by: String`.

### `AchievementTracker`
- `new() -> Self` — Empty tracker, no available badges.
- `with_badges(available: Vec<Badge>) -> Self` — Pre-loaded with badge definitions.
- `earn(&mut self, badge_id: &str, player: &str, tick: u64) -> Option<&Achievement>` — Award a badge. Returns `None` if badge not found or already earned.
- `has_badge(&self, badge_id: &str) -> bool` — Check if anyone has earned this badge.
- `badges_by_player(&self, player: &str) -> Vec<&Achievement>` — All achievements for a player.
- `badges_by_category(&self, cat: BadgeCategory) -> Vec<&Achievement>` — Filter by category.
- `player_tier_count(&self, player: &str, tier: BadgeTier) -> usize` — Count badges of a tier for a player.

### `default_badges() -> Vec<Badge>`
Returns 30+ built-in badges across all 7 categories and 5 tiers.

## How It Works

Badge lookup is O(n) linear scan over the `available` list. Earning appends to a `Vec<Achievement>`. Duplicate prevention checks `earned` by badge ID. The system is deliberately simple — no database, no indices — designed for the hundreds-of-badges scale of an educational game.

## Testing

**24 tests** covering: earning badges, duplicate prevention, player/category/tier queries, default badge list integrity, serde round-trips.

## License

MIT
