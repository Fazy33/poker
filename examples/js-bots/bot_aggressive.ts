import axios, { AxiosError } from 'axios';

const API_BASE = 'https://poker-server-637587888596.europe-west1.run.app/api';

interface GameState {
    game_id: string;
    phase: string;
    pot: number;
    current_bet: number;
    community_cards: string[];
    current_player_id: string | null;
    your_player_id: string;
    your_chips: number;
    your_cards: string[];
    valid_actions: string[];
}

interface Action {
    type: string;
    amount?: number;
}

class AggressiveBot {
    private name: string;
    private gameId: string | null;
    private playerId: string | null;
    private authToken: string | null;

    constructor(name: string, gameId: string | null = null) {
        this.name = name;
        this.gameId = gameId;
        this.playerId = null;
        this.authToken = null;
    }

    async joinGame(): Promise<boolean> {
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
            const err = error as AxiosError;
            console.error(`❌ Erreur join:`, err.message);
            return false;
        }
    }

    async getGameState(): Promise<GameState | null> {
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

    async submitAction(actionType: string, amount?: number): Promise<boolean> {
        if (!this.gameId || !this.playerId) {
            return false;
        }

        const action: Action = { type: actionType };
        if (amount !== undefined) {
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
            const err = error as AxiosError;
            console.error(`❌ Erreur action:`, err.message);
            return false;
        }
    }

    // Stratégie AGRESSIVE: raise souvent, intimidation
    decideAction(state: GameState): Action | null {
        const validActions = state.valid_actions || [];

        if (validActions.length === 0) {
            return null;
        }

        // 60% du temps, raise si possible
        if (validActions.includes('raise') && Math.random() < 0.6) {
            const raiseAmount = Math.max(
                state.current_bet + 20,
                Math.floor(state.your_chips * 0.15) // 15% de nos jetons
            );
            return { type: 'raise', amount: raiseAmount };
        }

        // Sinon check si possible
        if (validActions.includes('check')) {
            return { type: 'check' };
        }

        // Call si on peut
        if (validActions.includes('call')) {
            return { type: 'call' };
        }

        // En dernier recours, fold
        return { type: 'fold' };
    }

    async play(): Promise<void> {
        console.log(`\n🤖 Bot AGRESSIF ${this.name} en action!`);
        console.log('   Stratégie: Raise fréquent (60%), Call/Check sinon\n');

        while (true) {
            try {
                const state = await this.getGameState();

                if (!state) {
                    await this.sleep(2000);
                    continue;
                }

                // Vérifier si la partie est terminée
                if ((state as any).game_finished) {
                    console.log(`\n🏆 PARTIE TERMINÉE !`);
                    if ((state as any).winner_name) {
                        console.log(`   Gagnant: ${(state as any).winner_name}`);
                    }
                    if ((state as any).winner_id === this.playerId) {
                        console.log(`   🎉 FÉLICITATIONS ! Vous avez gagné !`);
                    } else {
                        console.log(`   😞 Vous avez perdu.`);
                    }

                    // Afficher le log des actions
                    const actionLog = (state as any).action_log;
                    if (actionLog && actionLog.length > 0) {
                        console.log(`\n📋 Historique de la partie:\n`);
                        actionLog.forEach((entry: string, i: number) => {
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
                    console.log(`   Actions valides: ${state.valid_actions.join(', ')}`);

                    const action = this.decideAction(state);

                    if (action) {
                        const display = action.amount
                            ? `${action.type} (${action.amount})`
                            : action.type;
                        console.log(`   → Décision: ${display}`);
                        await this.submitAction(action.type, action.amount);
                    }

                    await this.sleep(1000);
                } else {
                    console.log(`⏳ En attente...`);
                }

                await this.sleep(2000);

            } catch (error) {
                const err = error as AxiosError;
                if (err.message.includes('ECONNREFUSED')) {
                    console.log('⚠️  Serveur non disponible, réessai dans 5s...');
                    await this.sleep(5000);
                } else {
                    console.error(`❌ Erreur: ${err.message}`);
                    await this.sleep(5000);
                }
            }
        }
    }

    private sleep(ms: number): Promise<void> {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
}

async function main() {
    const args = process.argv.slice(2);

    if (args.length < 2) {
        console.log('Usage: ts-node bot_aggressive.ts <nom_du_bot> <game_id>');
        console.log('\nExemple:');
        console.log('  ts-node bot_aggressive.ts AggroBot <game_id>');
        process.exit(1);
    }

    const botName = args[0];
    const gameId = args[1];

    const bot = new AggressiveBot(botName, gameId);

    if (!await bot.joinGame()) {
        process.exit(1);
    }

    console.log(`\n⏳ En attente d'autres joueurs...`);

    await bot.play();
}

main().catch(console.error);
