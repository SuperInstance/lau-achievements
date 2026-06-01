# lau-achievements

> Badge and achievement system for the PLATO/Lau learning game — 33 built-in badges across 7 categories and 5 tiers, with per-player tracking and duplicate prevention.

## What This Does

This crate provides a complete achievement system for educational games:

- **33 pre-built badges** across 7 categories (Building, Learning, Conservation, Social, Exploration, Teaching, GitMastery) and 5 tiers (Bronze → Legendary)
- **Per-player tracking** — who earned what, when (in game ticks)
- **Query filters** — by player, by category, by tier
- **Duplicate prevention** — each badge can only be earned once globally
- **Full serde support** — JSON round-trips for persistence

If you're building a game where players earn badges for completing activities, this handles the bookkeeping.

## The Key Idea

Gamification through collectible badges. Each badge is a `Badge` definition (id, name, description, emoji icon, tier, category). When a player earns one, an `Achievement` record is created linking the badge to the player and the game tick. The `AchievementTracker` manages the full lifecycle — registration, earning, querying — and prevents double-awards.

```
┌────────────────────┐
│  AchievementTracker │
│                    │
│  available: [      │   earn("first-block", "alice", 100)
│    🧱 First Block  │ ──────────────────────────────────→ ✅
│    🏛️ Architect    │
│    📘 First Lesson │   earn("first-block", "alice", 200)
│    ...             │ ──────────────────────────────────→ ❌ duplicate
│  ]                 │
│                    │   earn("first-block", "bob", 150)
│  earned: [         │ ──────────────────────────────────→ ❌ already claimed
│    🧱 alice @ 100  │
│  ]                 │
└────────────────────┘
```

### Badge Tiers

| Tier | Rarity | Color Concept |
|------|--------|---------------|
| 🥉 Bronze | Common | First steps, beginner actions |
| 🥈 Silver | Uncommon | Repeated effort, moderate goals |
| 🥇 Gold | Rare | Significant accomplishments |
| 💎 Diamond | Epic | Mastery-level achievements |
| 🌟 Legendary | Mythic | Extraordinary, rare achievements |

### Badge Categories

| Category | Focus |
|----------|-------|
| Building | Construction, architecture, world creation |
| Learning | Lessons, quizzes, coding exercises |
| Conservation | Ecosystem balance, environmental actions |
| Social | Teamwork, sharing, community building |
| Exploration | Biomes, mapping, distance traveled |
| Teaching | Mentoring, tutorials, course creation |
| GitMastery | Version control skills (commits, branches, merges) |

## Install

```bash
cargo add lau-achievements
```

**Dependencies:** `serde` (with `derive`)

**Dev dependencies:** `serde_json`

## Quick Start

```rust
use lau_achievements::*;

// 1. Use the built-in badge set (33 badges)
let badges = default_badges();
let mut tracker = AchievementTracker::with_badges(badges);

// 2. Alice earns her first badge
let achievement = tracker.earn("first-block", "alice", 100).unwrap();
println!("{} earned {}! {}", achievement.earned_by, achievement.badge.name, achievement.badge.icon);
// Output: alice earned First Block! 🧱

// 3. Can't earn the same badge twice (globally unique)
assert!(tracker.earn("first-block", "alice", 200).is_none());
assert!(tracker.earn("first-block", "bob", 150).is_none());

// 4. Bob earns different badges
tracker.earn("architect", "bob", 500);       // Silver, Building
tracker.earn("first-lesson", "bob", 300);    // Bronze, Learning

// 5. Query by player
let bob_badges = tracker.badges_by_player("bob");
assert_eq!(bob_badges.len(), 2);

// 6. Query by category
let building = tracker.badges_by_category(BadgeCategory::Building);
let learning = tracker.badges_by_category(BadgeCategory::Learning);

// 7. Count by tier
let bob_bronze = tracker.player_tier_count("bob", BadgeTier::Bronze);
let bob_silver = tracker.player_tier_count("bob", BadgeTier::Silver);

// 8. Check if a badge has been earned
assert!(tracker.has_badge("first-block"));
assert!(!tracker.has_badge("git-wizard"));
```

### Custom Badges

```rust
use lau_achievements::*;

let custom_badge = Badge {
    id: "custom-1".into(),
    name: "Speed Runner".into(),
    description: "Complete a level in under 60 seconds.".into(),
    icon: "⚡".into(),
    tier: BadgeTier::Gold,
    category: BadgeCategory::Exploration,
};

let mut tracker = AchievementTracker::with_badges(vec![custom_badge]);
tracker.earn("custom-1", "alice", 999);
```

## API Reference

### `Badge`
A badge definition — the template for an achievement.

| Field | Type | Description |
|-------|------|-------------|
| `id` | `String` | Unique identifier (e.g., `"first-block"`) |
| `name` | `String` | Human-readable name |
| `description` | `String` | What you did to earn it |
| `icon` | `String` | Emoji icon (e.g., `"🧱"`) |
| `tier` | `BadgeTier` | Rarity level |
| `category` | `BadgeCategory` | Domain category |

### `BadgeTier` (enum)
`Bronze`, `Silver`, `Gold`, `Diamond`, `Legendary`

### `BadgeCategory` (enum)
`Building`, `Learning`, `Conservation`, `Social`, `Exploration`, `Teaching`, `GitMastery`

### `Achievement`
An earned badge instance — links a badge to a player and moment.

| Field | Type | Description |
|-------|------|-------------|
| `badge` | `Badge` | The badge definition (owned copy) |
| `earned_at_tick` | `u64` | Game tick when earned |
| `earned_by` | `String` | Player identifier |

### `AchievementTracker`
Manages available badges and earned achievements.

| Method | Signature | Description |
|--------|-----------|-------------|
| `new` | `() -> Self` | Empty tracker, no available badges |
| `with_badges` | `(Vec<Badge>) -> Self` | Pre-loaded with badge definitions |
| `earn` | `(&mut self, badge_id: &str, player: &str, tick: u64) -> Option<&Achievement>` | Award a badge. Returns `None` if not found or already earned. |
| `has_badge` | `(&self, badge_id: &str) -> bool` | Check if anyone has earned this badge |
| `badges_by_player` | `(&self, player: &str) -> Vec<&Achievement>` | All achievements for a specific player |
| `badges_by_category` | `(&self, cat: BadgeCategory) -> Vec<&Achievement>` | Filter earned achievements by category |
| `player_tier_count` | `(&self, player: &str, tier: BadgeTier) -> usize` | Count badges of a specific tier for a player |

Implements `Default` (same as `new()`).

### `default_badges() -> Vec<Badge>`
Returns 33 built-in badges. All 7 categories and all 5 tiers are represented. No duplicate IDs.

**Category breakdown:**
- Building (5): First Block 🧱, Architect 🏛️, World Builder 🌍, Master Builder 🏗️, Redstone Engineer 🔌
- Learning (5): First Lesson 📘, Knowledge Seeker 📚, Scholar 🎓, Quiz Champion 🏆, Code Master 💻
- Conservation (5): Balance Keeper ⚖️, Conservation Master 🌿, Perfect Balance ♻️, Tree Planter 🌳, Ocean Guardian 🐠
- Social (5): Team Player 🤝, World Sharer 🌐, Community Builder 🏘️, Mentor 🌟, Collaborator 👥
- Exploration (4): Trailblazer 🥾, Cartographer 🗺️, Deep Diver 🏊, Globetrotter ✈️
- Teaching (4): Apprentice Teacher 📝, Guide 🧭, Master Teacher 📖, Sage 🧙
- GitMastery (5): First Save 💾, Branch Explorer 🌿, Merge Master 🔀, Git Wizard ⚡, History Explorer 🔍

## How It Works

### Badge Lookup
Badge lookup is O(n) linear scan over the `available` list. At the scale of an educational game (hundreds of badges max), this is fast enough and avoids the complexity of a HashMap.

### Earning Flow
1. Check if the badge ID exists in `available` → `None` if not found
2. Check if the badge has already been earned (via `has_badge`) → `None` if duplicate
3. Clone the badge definition, create an `Achievement` with player and tick
4. Append to `earned` vector, return a reference

### Duplicate Prevention
Duplicates are checked **globally** by badge ID, not per player. Once any player earns a badge, no one else can earn it. This is by design — some badges represent unique world firsts. If you want per-player duplicates, use a separate tracker per player.

### Serialization
All types derive `Serialize`/`Deserialize`. Achievement trackers can be persisted as JSON:

```rust
let json = serde_json::to_string(&tracker).unwrap();
let restored: AchievementTracker = serde_json::from_str(&json).unwrap();
```

## The Math

### Badge Distribution
The default set has 33 badges distributed as:
- **By tier**: Bronze (9), Silver (8), Gold (8), Diamond (5), Legendary (1) — heavier at lower tiers, encouraging progression
- **By category**: 7 categories with 4–5 badges each — balanced coverage

### Deduplication as Set Membership
The `has_badge` check is a set membership test: `badge_id ∈ {a.badge.id for a in earned}`. This ensures each badge is awarded at most once, modeling achievements as a mathematical set rather than a multiset.

### Query Complexity
All queries are O(n) where n is the number of earned achievements:
- `badges_by_player`: filters `earned` by player name
- `badges_by_category`: filters `earned` by badge category
- `player_tier_count`: filters `earned` by player AND tier

At the expected scale (<1000 achievements), these are effectively instantaneous.

## Testing

**24 tests** covering:
- Tracker construction: `new()`, `with_badges()`, `Default` trait
- Earning badges: success, unknown badge, duplicate prevention, cross-player duplicate prevention
- `has_badge`: positive and negative cases
- `badges_by_player`: multi-player tracking, empty results
- `badges_by_category`: filtering, empty results
- `player_tier_count`: multi-tier counting, player isolation
- `default_badges()`: count (≥30), all 7 categories present, all 5 tiers present, no duplicate IDs
- Serde round-trips: `Badge`, `Achievement`, `AchievementTracker`
- Edge cases: earning on empty tracker, independent player tracking, tier counting isolation

```bash
cargo test    # Run all 24 tests
```

## License

MIT
