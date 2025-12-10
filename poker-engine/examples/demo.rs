use poker_engine::{GameState, PlayerAction, Card, Rank, Suit, Hand};

fn main() {
    println!("=== Démonstration du Moteur de Poker ===\n");

    // Démonstration 1: Évaluation de mains
    demo_hand_evaluation();

    println!("\n{}\n", "=".repeat(50));

    // Démonstration 2: Partie complète
    demo_full_game();
}

fn demo_hand_evaluation() {
    println!("📊 Démonstration de l'évaluation des mains\n");

    // Quinte Flush Royale
    let royal_flush = vec![
        Card::new(Rank::Ace, Suit::Spades),
        Card::new(Rank::King, Suit::Spades),
        Card::new(Rank::Queen, Suit::Spades),
        Card::new(Rank::Jack, Suit::Spades),
        Card::new(Rank::Ten, Suit::Spades),
    ];
    let hand = Hand::evaluate(&royal_flush);
    println!("🃏 Main: {} {} {} {} {}", 
        royal_flush[0], royal_flush[1], royal_flush[2], royal_flush[3], royal_flush[4]);
    println!("   Rang: {:?}\n", hand.rank);

    // Carré
    let four_kind = vec![
        Card::new(Rank::King, Suit::Spades),
        Card::new(Rank::King, Suit::Hearts),
        Card::new(Rank::King, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Ace, Suit::Spades),
    ];
    let hand = Hand::evaluate(&four_kind);
    println!("🃏 Main: {} {} {} {} {}", 
        four_kind[0], four_kind[1], four_kind[2], four_kind[3], four_kind[4]);
    println!("   Rang: {:?}\n", hand.rank);

    // Double paire
    let two_pair = vec![
        Card::new(Rank::Ace, Suit::Spades),
        Card::new(Rank::Ace, Suit::Hearts),
        Card::new(Rank::King, Suit::Diamonds),
        Card::new(Rank::King, Suit::Clubs),
        Card::new(Rank::Queen, Suit::Spades),
    ];
    let hand = Hand::evaluate(&two_pair);
    println!("🃏 Main: {} {} {} {} {}", 
        two_pair[0], two_pair[1], two_pair[2], two_pair[3], two_pair[4]);
    println!("   Rang: {:?}", hand.rank);
}

fn demo_full_game() {
    println!("🎮 Démonstration d'une partie complète\n");

    // Créer une partie avec 3 joueurs
    let players = vec![
        ("alice".to_string(), "Alice".to_string()),
        ("bob".to_string(), "Bob".to_string()),
        ("charlie".to_string(), "Charlie".to_string()),
    ];

    let mut game = GameState::new(players, 1000, 10, 20);
    
    println!("Joueurs:");
    for player in &game.players {
        println!("  - {} ({}): {} jetons", player.name, player.id, player.chips);
    }

    println!("\n🎲 Démarrage d'une nouvelle main...\n");
    game.start_new_hand();

    println!("Blinds postées:");
    println!("  Small Blind: {}", game.small_blind);
    println!("  Big Blind: {}", game.big_blind);
    println!("  Pot initial: {}\n", game.pot);

    println!("Phase: {:?}", game.phase);
    println!("Joueur actuel: {}\n", game.players[game.current_player].name);

    // Afficher les cartes privées (normalement cachées)
    println!("Cartes privées (pour la démo):");
    for player in &game.players {
        if !player.hole_cards.is_empty() {
            println!("  {}: {} {}", 
                player.name, 
                player.hole_cards[0], 
                player.hole_cards[1]
            );
        }
    }

    println!("\n🎯 Tour de mise Pre-Flop:");
    
    // Charlie (après BB) suit
    let current_player_id = game.players[game.current_player].id.clone();
    println!("  {} suit", game.players[game.current_player].name);
    game.execute_action(&current_player_id, PlayerAction::Call).unwrap();

    // Alice (SB) relance
    let current_player_id = game.players[game.current_player].id.clone();
    println!("  {} relance de 40", game.players[game.current_player].name);
    game.execute_action(&current_player_id, PlayerAction::Raise(40)).unwrap();

    // Bob (BB) suit
    let current_player_id = game.players[game.current_player].id.clone();
    println!("  {} suit", game.players[game.current_player].name);
    game.execute_action(&current_player_id, PlayerAction::Call).unwrap();

    // Charlie suit
    let current_player_id = game.players[game.current_player].id.clone();
    println!("  {} suit", game.players[game.current_player].name);
    game.execute_action(&current_player_id, PlayerAction::Call).unwrap();

    println!("\n💰 Pot: {}", game.pot);
    println!("Phase: {:?}", game.phase);

    if !game.community_cards.is_empty() {
        print!("Cartes communes: ");
        for card in &game.community_cards {
            print!("{} ", card);
        }
        println!();
    }

    println!("\n✅ Moteur de poker fonctionnel!");
    println!("   - Gestion des joueurs ✓");
    println!("   - Distribution des cartes ✓");
    println!("   - Blinds automatiques ✓");
    println!("   - Actions des joueurs ✓");
    println!("   - Progression des phases ✓");
}
