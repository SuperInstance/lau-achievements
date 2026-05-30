use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// The tier / rarity of a badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BadgeTier {
    Bronze,
    Silver,
    Gold,
    Diamond,
    Legendary,
}

/// The category a badge belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BadgeCategory {
    Building,
    Learning,
    Conservation,
    Social,
    Exploration,
    Teaching,
    GitMastery,
}

// ---------------------------------------------------------------------------
// Core types
// ---------------------------------------------------------------------------

/// A badge definition – the thing a kid can earn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Badge {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,      // emoji
    pub tier: BadgeTier,
    pub category: BadgeCategory,
}

/// A concrete instance of a badge that someone has earned.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Achievement {
    pub badge: Badge,
    pub earned_at_tick: u64,
    pub earned_by: String,
}

/// Tracks which badges are available and which have been earned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementTracker {
    pub earned: Vec<Achievement>,
    pub available: Vec<Badge>,
}

impl AchievementTracker {
    /// Create an empty tracker with no available badges.
    pub fn new() -> Self {
        Self {
            earned: Vec::new(),
            available: Vec::new(),
        }
    }

    /// Create a tracker pre-loaded with the given badges.
    pub fn with_badges(available: Vec<Badge>) -> Self {
        Self {
            earned: Vec::new(),
            available,
        }
    }

    /// Attempt to earn a badge by its `id`.
    ///
    /// Returns `None` if the badge doesn't exist in `available`, or if the
    /// player has already earned it. Otherwise pushes a new `Achievement`
    /// and returns `Some(&Achievement)`.
    pub fn earn(&mut self, badge_id: &str, player: &str, tick: u64) -> Option<&Achievement> {
        // Don't allow duplicates.
        if self.has_badge(badge_id) {
            return None;
        }

        let badge = self.available.iter().find(|b| b.id == badge_id)?.clone();

        self.earned.push(Achievement {
            badge,
            earned_at_tick: tick,
            earned_by: player.to_string(),
        });
        self.earned.last()
    }

    /// Check whether *any* player has earned the badge with the given id.
    pub fn has_badge(&self, badge_id: &str) -> bool {
        self.earned.iter().any(|a| a.badge.id == badge_id)
    }

    /// Return all achievements earned by a specific player.
    pub fn badges_by_player(&self, player: &str) -> Vec<&Achievement> {
        self.earned
            .iter()
            .filter(|a| a.earned_by == player)
            .collect()
    }

    /// Return all achievements whose badge belongs to the given category.
    pub fn badges_by_category(&self, cat: BadgeCategory) -> Vec<&Achievement> {
        self.earned
            .iter()
            .filter(|a| a.badge.category == cat)
            .collect()
    }

    /// Count how many badges of a given tier a specific player has earned.
    pub fn player_tier_count(&self, player: &str, tier: BadgeTier) -> usize {
        self.earned
            .iter()
            .filter(|a| a.earned_by == player && a.badge.tier == tier)
            .count()
    }
}

impl Default for AchievementTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Pre-defined badges  (30+)
// ---------------------------------------------------------------------------

/// Return the canonical list of all built-in badges.
pub fn default_badges() -> Vec<Badge> {
    vec![
        // ── Building ──────────────────────────────────────────────────
        Badge {
            id: "first-block".into(),
            name: "First Block".into(),
            description: "Place your very first block in the world.".into(),
            icon: "🧱".into(),
            tier: BadgeTier::Bronze,
            category: BadgeCategory::Building,
        },
        Badge {
            id: "architect".into(),
            name: "Architect".into(),
            description: "Build a structure with at least 10 different block types.".into(),
            icon: "🏛️".into(),
            tier: BadgeTier::Silver,
            category: BadgeCategory::Building,
        },
        Badge {
            id: "world-builder".into(),
            name: "World Builder".into(),
            description: "Fill a 100×100 area with your own creations.".into(),
            icon: "🌍".into(),
            tier: BadgeTier::Gold,
            category: BadgeCategory::Building,
        },
        Badge {
            id: "master-builder".into(),
            name: "Master Builder".into(),
            description: "Create a multi-floor building with working elevators.".into(),
            icon: "🏗️".into(),
            tier: BadgeTier::Diamond,
            category: BadgeCategory::Building,
        },
        Badge {
            id: "redstone-engineer".into(),
            name: "Redstone Engineer".into(),
            description: "Build a working circuit with logic gates.".into(),
            icon: "🔌".into(),
            tier: BadgeTier::Gold,
            category: BadgeCategory::Building,
        },
        // ── Learning ──────────────────────────────────────────────────
        Badge {
            id: "first-lesson".into(),
            name: "First Lesson".into(),
            description: "Complete your first learning module.".into(),
            icon: "📘".into(),
            tier: BadgeTier::Bronze,
            category: BadgeCategory::Learning,
        },
        Badge {
            id: "knowledge-seeker".into(),
            name: "Knowledge Seeker".into(),
            description: "Complete 10 lessons across any subject.".into(),
            icon: "📚".into(),
            tier: BadgeTier::Silver,
            category: BadgeCategory::Learning,
        },
        Badge {
            id: "scholar".into(),
            name: "Scholar".into(),
            description: "Score 100% on any advanced quiz.".into(),
            icon: "🎓".into(),
            tier: BadgeTier::Gold,
            category: BadgeCategory::Learning,
        },
        Badge {
            id: "quiz-champion".into(),
            name: "Quiz Champion".into(),
            description: "Win 5 quiz challenges in a row.".into(),
            icon: "🏆".into(),
            tier: BadgeTier::Diamond,
            category: BadgeCategory::Learning,
        },
        Badge {
            id: "code-master".into(),
            name: "Code Master".into(),
            description: "Write 100 lines of runnable code.".into(),
            icon: "💻".into(),
            tier: BadgeTier::Gold,
            category: BadgeCategory::Learning,
        },
        // ── Conservation ──────────────────────────────────────────────
        Badge {
            id: "balance-keeper".into(),
            name: "Balance Keeper".into(),
            description: "Maintain a balanced ecosystem for 7 days.".into(),
            icon: "⚖️".into(),
            tier: BadgeTier::Silver,
            category: BadgeCategory::Conservation,
        },
        Badge {
            id: "conservation-master".into(),
            name: "Conservation Master".into(),
            description: "Restore a depleted biome to full health.".into(),
            icon: "🌿".into(),
            tier: BadgeTier::Gold,
            category: BadgeCategory::Conservation,
        },
        Badge {
            id: "perfect-balance".into(),
            name: "Perfect Balance".into(),
            description: "Achieve a self-sustaining 10-species ecosystem.".into(),
            icon: "♻️".into(),
            tier: BadgeTier::Diamond,
            category: BadgeCategory::Conservation,
        },
        Badge {
            id: "tree-planter".into(),
            name: "Tree Planter".into(),
            description: "Plant 50 trees in a single session.".into(),
            icon: "🌳".into(),
            tier: BadgeTier::Bronze,
            category: BadgeCategory::Conservation,
        },
        Badge {
            id: "ocean-guardian".into(),
            name: "Ocean Guardian".into(),
            description: "Clean up 100 pieces of ocean trash.".into(),
            icon: "🐠".into(),
            tier: BadgeTier::Silver,
            category: BadgeCategory::Conservation,
        },
        // ── Social ────────────────────────────────────────────────────
        Badge {
            id: "team-player".into(),
            name: "Team Player".into(),
            description: "Complete a project with at least one friend.".into(),
            icon: "🤝".into(),
            tier: BadgeTier::Bronze,
            category: BadgeCategory::Social,
        },
        Badge {
            id: "world-sharer".into(),
            name: "World Sharer".into(),
            description: "Share your world with 5 other players.".into(),
            icon: "🌐".into(),
            tier: BadgeTier::Silver,
            category: BadgeCategory::Social,
        },
        Badge {
            id: "community-builder".into(),
            name: "Community Builder".into(),
            description: "Host an event with 10+ participants.".into(),
            icon: "🏘️".into(),
            tier: BadgeTier::Gold,
            category: BadgeCategory::Social,
        },
        Badge {
            id: "mentor".into(),
            name: "Mentor".into(),
            description: "Help 3 new players get their first badge.".into(),
            icon: "🌟".into(),
            tier: BadgeTier::Diamond,
            category: BadgeCategory::Social,
        },
        Badge {
            id: "collaborator".into(),
            name: "Collaborator".into(),
            description: "Contribute to 3 group builds.".into(),
            icon: "👥".into(),
            tier: BadgeTier::Bronze,
            category: BadgeCategory::Social,
        },
        // ── Exploration ───────────────────────────────────────────────
        Badge {
            id: "trailblazer".into(),
            name: "Trailblazer".into(),
            description: "Visit 5 distinct biomes.".into(),
            icon: "🥾".into(),
            tier: BadgeTier::Bronze,
            category: BadgeCategory::Exploration,
        },
        Badge {
            id: "cartographer".into(),
            name: "Cartographer".into(),
            description: "Map 10,000 blocks of terrain.".into(),
            icon: "🗺️".into(),
            tier: BadgeTier::Silver,
            category: BadgeCategory::Exploration,
        },
        Badge {
            id: "deep-diver".into(),
            name: "Deep Diver".into(),
            description: "Reach the deepest layer of the world.".into(),
            icon: "🏊".into(),
            tier: BadgeTier::Gold,
            category: BadgeCategory::Exploration,
        },
        Badge {
            id: "globetrotter".into(),
            name: "Globetrotter".into(),
            description: "Travel 100,000 blocks from spawn.".into(),
            icon: "✈️".into(),
            tier: BadgeTier::Diamond,
            category: BadgeCategory::Exploration,
        },
        // ── Teaching ──────────────────────────────────────────────────
        Badge {
            id: "apprentice-teacher".into(),
            name: "Apprentice Teacher".into(),
            description: "Teach one skill to another player.".into(),
            icon: "📝".into(),
            tier: BadgeTier::Bronze,
            category: BadgeCategory::Teaching,
        },
        Badge {
            id: "guide".into(),
            name: "Guide".into(),
            description: "Run a tutorial session for 3 players.".into(),
            icon: "🧭".into(),
            tier: BadgeTier::Silver,
            category: BadgeCategory::Teaching,
        },
        Badge {
            id: "master-teacher".into(),
            name: "Master Teacher".into(),
            description: "Create a learning course used by 20 players.".into(),
            icon: "📖".into(),
            tier: BadgeTier::Gold,
            category: BadgeCategory::Teaching,
        },
        Badge {
            id: "sage".into(),
            name: "Sage".into(),
            description: "Mentor 5 players to their first gold badge.".into(),
            icon: "🧙".into(),
            tier: BadgeTier::Legendary,
            category: BadgeCategory::Teaching,
        },
        // ── Git Mastery ───────────────────────────────────────────────
        Badge {
            id: "first-save".into(),
            name: "First Save".into(),
            description: "Make your first git commit.".into(),
            icon: "💾".into(),
            tier: BadgeTier::Bronze,
            category: BadgeCategory::GitMastery,
        },
        Badge {
            id: "branch-explorer".into(),
            name: "Branch Explorer".into(),
            description: "Create and switch between 3 branches.".into(),
            icon: "🌿".into(),
            tier: BadgeTier::Silver,
            category: BadgeCategory::GitMastery,
        },
        Badge {
            id: "merge-master".into(),
            name: "Merge Master".into(),
            description: "Successfully merge 5 pull requests.".into(),
            icon: "🔀".into(),
            tier: BadgeTier::Gold,
            category: BadgeCategory::GitMastery,
        },
        Badge {
            id: "git-wizard".into(),
            name: "Git Wizard".into(),
            description: "Resolve a complex merge conflict with 10+ files.".into(),
            icon: "⚡".into(),
            tier: BadgeTier::Diamond,
            category: BadgeCategory::GitMastery,
        },
        Badge {
            id: "history-explorer".into(),
            name: "History Explorer".into(),
            description: "Use git log, blame, and bisect to find a bug.".into(),
            icon: "🔍".into(),
            tier: BadgeTier::Bronze,
            category: BadgeCategory::GitMastery,
        },
    ]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ── Basic construction ────────────────────────────────────────────

    #[test]
    fn test_tracker_new_is_empty() {
        let t = AchievementTracker::new();
        assert!(t.earned.is_empty());
        assert!(t.available.is_empty());
    }

    #[test]
    fn test_tracker_with_badges() {
        let badges = vec![Badge {
            id: "test".into(),
            name: "Test".into(),
            description: "".into(),
            icon: "🔬".into(),
            tier: BadgeTier::Bronze,
            category: BadgeCategory::Learning,
        }];
        let t = AchievementTracker::with_badges(badges);
        assert_eq!(t.available.len(), 1);
        assert!(t.earned.is_empty());
    }

    #[test]
    fn test_tracker_default() {
        let t: AchievementTracker = Default::default();
        assert!(t.earned.is_empty());
    }

    // ── Earning badges ────────────────────────────────────────────────

    #[test]
    fn test_earn_success() {
        let badges = default_badges();
        let mut t = AchievementTracker::with_badges(badges);
        let ach = t.earn("first-block", "alice", 42);
        assert!(ach.is_some());
        assert_eq!(ach.unwrap().earned_by, "alice");
        assert_eq!(ach.unwrap().earned_at_tick, 42);
        assert_eq!(ach.unwrap().badge.id, "first-block");
    }

    #[test]
    fn test_earn_unknown_badge() {
        let mut t = AchievementTracker::new();
        assert!(t.earn("does-not-exist", "alice", 0).is_none());
    }

    #[test]
    fn test_earn_no_duplicates() {
        let badges = default_badges();
        let mut t = AchievementTracker::with_badges(badges);
        assert!(t.earn("first-block", "alice", 1).is_some());
        assert!(t.earn("first-block", "alice", 2).is_none());
    }

    #[test]
    fn test_earn_different_players_same_badge() {
        let badges = default_badges();
        let mut t = AchievementTracker::with_badges(badges);
        assert!(t.earn("first-block", "alice", 1).is_some());
        assert!(t.earn("first-block", "bob", 2).is_none()); // badge already earned
    }

    // ── has_badge ─────────────────────────────────────────────────────

    #[test]
    fn test_has_badge_true() {
        let badges = default_badges();
        let mut t = AchievementTracker::with_badges(badges);
        t.earn("first-block", "alice", 1);
        assert!(t.has_badge("first-block"));
    }

    #[test]
    fn test_has_badge_false() {
        let t = AchievementTracker::new();
        assert!(!t.has_badge("nothing"));
    }

    // ── badges_by_player ──────────────────────────────────────────────

    #[test]
    fn test_badges_by_player() {
        let badges = default_badges();
        let mut t = AchievementTracker::with_badges(badges);
        t.earn("first-block", "alice", 1);
        t.earn("tree-planter", "alice", 2);
        t.earn("first-lesson", "bob", 3);

        let alice_badges = t.badges_by_player("alice");
        assert_eq!(alice_badges.len(), 2);

        let bob_badges = t.badges_by_player("bob");
        assert_eq!(bob_badges.len(), 1);

        let nobody = t.badges_by_player("charlie");
        assert!(nobody.is_empty());
    }

    // ── badges_by_category ────────────────────────────────────────────

    #[test]
    fn test_badges_by_category() {
        let badges = default_badges();
        let mut t = AchievementTracker::with_badges(badges);
        t.earn("first-block", "alice", 1); // Building
        t.earn("tree-planter", "alice", 1); // Conservation

        let building = t.badges_by_category(BadgeCategory::Building);
        assert_eq!(building.len(), 1);
        assert_eq!(building[0].badge.id, "first-block");

        let learning = t.badges_by_category(BadgeCategory::Learning);
        assert!(learning.is_empty());
    }

    // ── player_tier_count ─────────────────────────────────────────────

    #[test]
    fn test_player_tier_count() {
        let badges = default_badges();
        let mut t = AchievementTracker::with_badges(badges);
        t.earn("first-block", "alice", 1); // Bronze
        t.earn("tree-planter", "alice", 2); // Bronze
        t.earn("architect", "alice", 3); // Silver

        assert_eq!(t.player_tier_count("alice", BadgeTier::Bronze), 2);
        assert_eq!(t.player_tier_count("alice", BadgeTier::Silver), 1);
        assert_eq!(t.player_tier_count("alice", BadgeTier::Gold), 0);
        assert_eq!(t.player_tier_count("bob", BadgeTier::Bronze), 0);
    }

    // ── default_badges ────────────────────────────────────────────────

    #[test]
    fn test_default_badges_count() {
        let badges = default_badges();
        assert!(badges.len() >= 30, "expected 30+, got {}", badges.len());
    }

    #[test]
    fn test_default_badges_all_categories_represented() {
        let badges = default_badges();
        for cat in &[
            BadgeCategory::Building,
            BadgeCategory::Learning,
            BadgeCategory::Conservation,
            BadgeCategory::Social,
            BadgeCategory::Exploration,
            BadgeCategory::Teaching,
            BadgeCategory::GitMastery,
        ] {
            assert!(
                badges.iter().any(|b| &b.category == cat),
                "missing category {cat:?}"
            );
        }
    }

    #[test]
    fn test_default_badges_no_duplicate_ids() {
        let badges = default_badges();
        let mut ids: Vec<&str> = badges.iter().map(|b| b.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), badges.len(), "duplicate badge ids found");
    }

    #[test]
    fn test_default_badges_all_tiers_represented() {
        let badges = default_badges();
        for tier in &[
            BadgeTier::Bronze,
            BadgeTier::Silver,
            BadgeTier::Gold,
            BadgeTier::Diamond,
            BadgeTier::Legendary,
        ] {
            assert!(
                badges.iter().any(|b| &b.tier == tier),
                "missing tier {tier:?}"
            );
        }
    }

    // ── Serde round-trip ──────────────────────────────────────────────

    #[test]
    fn test_serde_badge_roundtrip() {
        let b = Badge {
            id: "test-id".into(),
            name: "Test Badge".into(),
            description: "A description".into(),
            icon: "🎖️".into(),
            tier: BadgeTier::Gold,
            category: BadgeCategory::Building,
        };
        let json = serde_json::to_string(&b).unwrap();
        let back: Badge = serde_json::from_str(&json).unwrap();
        assert_eq!(b, back);
    }

    #[test]
    fn test_serde_achievement_roundtrip() {
        let a = Achievement {
            badge: Badge {
                id: "test-id".into(),
                name: "Test".into(),
                description: "".into(),
                icon: "🔬".into(),
                tier: BadgeTier::Diamond,
                category: BadgeCategory::GitMastery,
            },
            earned_at_tick: 99,
            earned_by: "charlie".into(),
        };
        let json = serde_json::to_string(&a).unwrap();
        let back: Achievement = serde_json::from_str(&json).unwrap();
        assert_eq!(a, back);
    }

    #[test]
    fn test_serde_tracker_roundtrip() {
        let badges = default_badges();
        let mut t = AchievementTracker::with_badges(badges);
        t.earn("first-block", "alice", 1);
        let json = serde_json::to_string(&t).unwrap();
        let back: AchievementTracker = serde_json::from_str(&json).unwrap();
        assert_eq!(back.earned.len(), 1);
        assert_eq!(back.earned[0].badge.id, "first-block");
        assert_eq!(back.available.len(), t.available.len());
    }

    // ── Edge cases ────────────────────────────────────────────────────

    #[test]
    fn test_earn_on_empty_tracker() {
        let mut t = AchievementTracker::new();
        assert!(t.earn("anything", "alice", 0).is_none());
    }

    #[test]
    fn test_multiple_players_independent() {
        let mut t = AchievementTracker::with_badges(default_badges());
        // Players can each earn their own distinct badges.
        t.earn("first-block", "alice", 1);
        t.earn("tree-planter", "bob", 2);
        assert_eq!(t.badges_by_player("alice").len(), 1);
        assert_eq!(t.badges_by_player("bob").len(), 1);
    }

    #[test]
    fn test_player_tier_count_only_counts_player() {
        let mut t = AchievementTracker::with_badges(default_badges());
        t.earn("first-block", "alice", 1); // Bronze, alice
        t.earn("tree-planter", "bob", 2); // Bronze, bob
        t.earn("apprentice-teacher", "alice", 3); // Bronze, alice
        assert_eq!(t.player_tier_count("alice", BadgeTier::Bronze), 2);
        assert_eq!(t.player_tier_count("bob", BadgeTier::Bronze), 1);
    }

    #[test]
    fn test_badges_by_category_empty_when_nothing_earned() {
        let t = AchievementTracker::with_badges(default_badges());
        assert!(t.badges_by_category(BadgeCategory::Building).is_empty());
    }

    #[test]
    fn test_has_badge_false_on_empty_tracker() {
        let t = AchievementTracker::new();
        assert!(!t.has_badge("first-block"));
    }
}
