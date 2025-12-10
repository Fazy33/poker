const axios = require('axios');

const API_BASE = 'http://localhost:8080/api';

class ConservativeBot {
    constructor(name, gameId = null) {
        this.name = name;
        this.gameId = gameId;
        this.playerId = null;
        this.authToken = null;
    }

    async createGame() {
        try {
            const response = await axios.post(`${API_BASE}/games`, {
                name: `Partie de ${this.name}`,
                max_players: 4,
                starting_chips: 1000,
                small_blind: 10,
                big_blind: 20
            });
            this.gameId = response.data.game_id;
            console.log(`✅ Partie créée: ${this.gameId}`);
            return true;
        } catch (error) {
            console.error(`❌ Erreur création:`, error.message);
            return false;
        }
    }

    async joinGame() {
        if (!this.gameId) {
            console.error('❌ Aucune partie spécifiée');
            return false;
        }

        try {
            const response = await axios.post(`${API_BASE}/games/${this.gameId}/join`, {
                bot_name: this.name
            });
            this.playerId = response.data.player_id;
            this.authToken = response.data.auth_token;
            console.log(`✅ Rejoint la partie en position ${response.data.position}`);
            console.log(`   Player ID: ${this.playerId}`);
            return true;
        } catch (error) {
            console.error(`❌ Erreur join:`, error.message);
            return false;
        }
    }

    async getGameState() {
        if (!this.gameId || !this.playerId) {
            return null;
        }

        try {
            const response = await axios.get(`${API_BASE}/games/${this.gameId}/state`, {
                params: { player_id: this.playerId }
            });
            return response.data;
        } catch (error) {
            return null;
        }
    }

    async submitAction(actionType, amount = null) {
        if (!this.gameId || !this.playerId) {
            return false;
        }

        const action = { type: actionType };
        if (amount !== null) {
            action.amount = amount;
        }

        try {
            const response = await axios.post(`${API_BASE}/games/${this.gameId}/action`, {
                auth_token: this.authToken,
                action: action
            });

            if (response.data.success) {
                console.log(`✅ Action ${actionType} effectuée`);
                return true;
            } else {
                console.log(`❌ Action refusée: ${response.data.error}`);
                return false;
            }
        } catch (error) {
            console.error(`❌ Erreur action:`, error.message);
            return false;
        }
    }

    // Stratégie CONSERVATIVE: joue prudemment
    decideAction(state) {
        const validActions = state.valid_actions || [];

        if (validActions.length === 0) {
            return null;
        }

        // Toujours check si possible
        if (validActions.includes('check')) {
            return { type: 'check' };
        }

        // Call seulement si la mise est raisonnable (< 10% de nos jetons)
        if (validActions.includes('call')) {
            // Montant déjà misé par ce joueur dans ce tour
            const currentBet = state.players?.find(p => p.id === this.playerId)?.current_bet || 0;
            // Montant total requis pour rester dans la partie
            const totalRequired = state.current_bet || 0;
            // Montant supplémentaire à ajouter
            const callAmount = totalRequired - currentBet;
            const maxCall = (state.your_chips || 0) * 0.1;

            if (callAmount <= maxCall) {
                return { type: 'call' };
            }
        }

        // Sinon fold
        return { type: 'fold' };
    }

    async play() {
        console.log(`\n🤖 Bot CONSERVATEUR ${this.name} en action!`);
        console.log('   Stratégie: Check si possible, Call prudemment, sinon Fold\n');

        while (true) {
            try {
                const state = await this.getGameState();

                if (!state) {
                    await new Promise(resolve => setTimeout(resolve, 2000));
                    continue;
                }

                // Vérifier si la partie est terminée
                if (state.game_finished) {
                    console.log(`\n🏆 PARTIE TERMINÉE !`);
                    if (state.winner_name) {
                        console.log(`   Gagnant: ${state.winner_name}`);
                    }
                    if (state.winner_id === this.playerId) {
                        console.log(`   🎉 FÉLICITATIONS ! Vous avez gagné !`);
                    } else {
                        console.log(`   😞 Vous avez perdu.`);
                    }

                    // Afficher le log des actions
                    if (state.action_log && state.action_log.length > 0) {
                        console.log(`\n📋 Historique de la partie:\n`);
                        state.action_log.forEach((entry, i) => {
                            console.log(`  ${i + 1}. ${entry}`);
                        });
                    }

                    console.log('');
                    process.exit(0);
                }

                console.log(`\n📊 État du jeu:`);
                console.log(`   Phase: ${state.phase}`);
                console.log(`   Pot: ${state.pot}`);
                console.log(`   Vos jetons: ${state.your_chips || '?'}`);
                console.log(`   Vos cartes: ${(state.your_cards || []).join(', ')}`);

                if (state.current_player_id === this.playerId) {
                    console.log(`\n🎯 C'est notre tour!`);
                    console.log(`   Actions valides: ${(state.valid_actions || []).join(', ')}`);

                    const action = this.decideAction(state);

                    if (action) {
                        const display = action.amount
                            ? `${action.type} (${action.amount})`
                            : action.type;
                        console.log(`   → Décision: ${display}`);
                        await this.submitAction(action.type, action.amount);
                    }

                    await new Promise(resolve => setTimeout(resolve, 1000));
                } else {
                    console.log(`⏳ En attente...`);
                }

                await new Promise(resolve => setTimeout(resolve, 2000));

            } catch (error) {
                if (error.message.includes('ECONNREFUSED')) {
                    console.log('⚠️  Serveur non disponible, réessai dans 5s...');
                    await new Promise(resolve => setTimeout(resolve, 5000));
                } else {
                    console.error(`❌ Erreur: ${error.message}`);
                    await new Promise(resolve => setTimeout(resolve, 5000));
                }
            }
        }
    }
}

async function main() {
    const args = process.argv.slice(2);

    if (args.length < 1) {
        console.log('Usage: node bot_conservative.js <nom_du_bot> [game_id]');
        console.log('\nExemples:');
        console.log('  node bot_conservative.js ConservBot');
        console.log('  node bot_conservative.js ConservBot <game_id>');
        process.exit(1);
    }

    const botName = args[0];
    const gameId = args[1] || null;

    const bot = new ConservativeBot(botName, gameId);

    if (!gameId) {
        if (!await bot.createGame()) {
            process.exit(1);
        }
        console.log(`\n💡 Pour rejoindre cette partie avec un autre bot:`);
        console.log(`   node bot_aggressive.js AggroBot ${bot.gameId}`);
    }

    if (!await bot.joinGame()) {
        process.exit(1);
    }

    console.log(`\n⏳ En attente d'autres joueurs...`);
    console.log(`   Démarrez la partie avec:`);
    console.log(`   curl -X POST http://localhost:8080/api/games/${bot.gameId}/start`);

    await bot.play();
}

main().catch(console.error);
