// État de l'application
let currentGameId = null;
let pollInterval = null;
let humanPlayerId = null;
let humanAuthToken = null;
let isHumanPlayer = false;

// Éléments DOM
const lobby = document.getElementById('lobby');
const gameView = document.getElementById('gameView');
const createGameForm = document.getElementById('createGameForm');
const gamesList = document.getElementById('gamesList');

// Initialisation
document.addEventListener('DOMContentLoaded', () => {
    setupEventListeners();
    loadGames();
});

function setupEventListeners() {
    // Boutons du lobby
    document.getElementById('createGameBtn').addEventListener('click', showCreateGameForm);
    document.getElementById('joinAsPlayerBtn').addEventListener('click', showJoinAsPlayerDialog);
    document.getElementById('refreshBtn').addEventListener('click', loadGames);
    document.getElementById('cancelCreateBtn').addEventListener('click', hideCreateGameForm);

    // Boutons de contrôle de la partie
    document.getElementById('startGameBtn').addEventListener('click', () => startGame(currentGameId));
    document.getElementById('addBotBtn').addEventListener('click', addTestBot);
    document.getElementById('joinGameDirectBtn').addEventListener('click', showJoinAsPlayerDialog);
    document.getElementById('copyGameIdBtn').addEventListener('click', copyGameId);
    document.getElementById('backToLobbyBtn').addEventListener('click', backToLobby);

    // Toggle sidebar
    document.getElementById('sidebarToggle').addEventListener('click', toggleSidebar);

    // Boutons d'action pour joueur humain
    document.getElementById('foldBtn').addEventListener('click', () => submitAction('fold'));
    document.getElementById('checkBtn').addEventListener('click', () => submitAction('check'));
    document.getElementById('callBtn').addEventListener('click', () => submitAction('call'));
    document.getElementById('raiseBtn').addEventListener('click', submitRaise);
    document.getElementById('allinBtn').addEventListener('click', () => submitAction('allin'));

    // Formulaire de création
    document.getElementById('newGameForm').addEventListener('submit', createGame);
}

function toggleSidebar() {
    const sidebar = document.getElementById('actionLogSidebar');
    sidebar.classList.toggle('collapsed');

    const icon = sidebar.querySelector('.toggle-icon');
    if (sidebar.classList.contains('collapsed')) {
        icon.textContent = '◀'; // Flèche vers la gauche pour ouvrir (sidebar étant à droite)
    } else {
        icon.textContent = '▶'; // Flèche vers la droite pour fermer
    }
}

// === LOBBY ===

async function loadGames() {
    try {
        const response = await fetch('/api/games');
        const data = await response.json();

        displayGames(data.games);
    } catch (error) {
        console.error('Erreur lors du chargement des parties:', error);
        addLogEntry('Erreur de connexion au serveur', 'system');
    }
}

function displayGames(games) {
    if (games.length === 0) {
        gamesList.innerHTML = '<p style="text-align: center; opacity: 0.7;">Aucune partie disponible. Créez-en une !</p>';
        return;
    }

    gamesList.innerHTML = games.map(game => `
        <div class="game-card" onclick="viewGame('${game.game_id}')">
            <h3>${game.name}</h3>
            <div class="game-card-info">
                <span>👥 Joueurs: ${game.player_count}/${game.max_players}</span>
                <span>🎯 Phase: ${game.phase}</span>
                <span>💰 Pot: ${game.pot} jetons</span>
            </div>
        </div>
    `).join('');
}

function showCreateGameForm() {
    createGameForm.style.display = 'flex';
}

function hideCreateGameForm() {
    createGameForm.style.display = 'none';
}

async function createGame(e) {
    e.preventDefault();

    const gameData = {
        name: document.getElementById('gameName').value,
        max_players: parseInt(document.getElementById('maxPlayers').value),
        starting_chips: parseInt(document.getElementById('startingChips').value),
        small_blind: parseInt(document.getElementById('smallBlind').value),
        big_blind: parseInt(document.getElementById('bigBlind').value),
    };

    try {
        const response = await fetch('/api/games', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(gameData)
        });

        const data = await response.json();

        if (response.ok) {
            hideCreateGameForm();
            viewGame(data.game_id);
        } else {
            alert('Erreur: ' + (data.error || 'Impossible de créer la partie'));
        }
    } catch (error) {
        console.error('Erreur:', error);
        alert('Erreur de connexion au serveur');
    }
}

// === VUE DE JEU ===

function viewGame(gameId) {
    currentGameId = gameId;
    lobby.style.display = 'none';
    gameView.style.display = 'block';

    // Afficher les contrôles du header et la barre d'info
    document.getElementById('headerControls').style.display = 'flex';
    document.getElementById('gameInfoBar').style.display = 'flex';

    // Reset l'état précédent pour forcer un re-render complet
    lastGameState = null;
    isShowingWinner = false;

    // Démarrer le polling pour les mises à jour
    startPolling();
}

function backToLobby() {
    stopPolling();
    currentGameId = null;
    humanPlayerId = null;
    humanAuthToken = null;
    isHumanPlayer = false;

    // Reset l'état global
    lastGameState = null;
    isShowingWinner = false;

    // Cacher les contrôles du joueur humain
    document.getElementById('humanPlayerControls').style.display = 'none';

    // Cacher les sections de jeu
    gameView.style.display = 'none';
    lobby.style.display = 'block';

    // Cacher les contrôles du header et la barre d'info
    document.getElementById('headerControls').style.display = 'none';
    document.getElementById('gameInfoBar').style.display = 'none';

    loadGames();
}

function startPolling() {
    // Arrêter tout polling existant pour éviter les doublons
    stopPolling();

    // Mise à jour immédiate
    updateGameState();

    // Puis toutes les 2 secondes
    pollInterval = setInterval(updateGameState, 2000);
}

function stopPolling() {
    if (pollInterval) {
        clearInterval(pollInterval);
        pollInterval = null;
    }
}

async function updateGameState() {
    if (!currentGameId) return;

    // Capturer l'ID attendu pour vérifier la race condition
    const expectedGameId = currentGameId;

    try {
        // Utiliser le player_id du joueur humain si disponible, sinon mode spectateur
        const playerId = humanPlayerId || 'spectator';
        const response = await fetch(`/api/games/${expectedGameId}/state?player_id=${playerId}`);

        if (!response.ok) {
            // La partie n'existe peut-être plus
            if (response.status === 400) {
                // Vérifier si on est toujours sur la même partie avant de quitter
                if (currentGameId === expectedGameId) {
                    addLogEntry('Partie terminée ou introuvable', 'system');
                    backToLobby();
                }
            }
            return;
        }

        const gameState = await response.json();

        // SÉCURITÉ IMPORTANTE: Vérifier que la réponse correspond à la partie active
        // Si l'utilisateur a changé de partie pendant la requête, on ignore cette réponse
        if (currentGameId !== expectedGameId || gameState.game_id !== expectedGameId) {
            console.log(`⚠️ Ignoring state for game ${gameState.game_id} because current game is ${currentGameId}`);
            return;
        }

        renderGameState(gameState);
    } catch (error) {
        console.error('Erreur lors de la mise à jour:', error);
    }
}


let hasDismissedWinnerPopup = false;

function renderGameState(state) {
    // Mettre à jour l'interface du joueur humain si présent
    if (isHumanPlayer) {
        updateHumanPlayerUI(state);
    }

    // Afficher le gagnant de la main précédente si disponible
    if (state.last_hand_winner && !isShowingWinner) {
        showHandWinner(state.last_hand_winner_name, state.last_hand_amount, state.last_hand_description, state.last_hand_cards);
    }

    // Afficher le résultat final si la partie est terminée
    // Ne pas réafficher si l'utilisateur l'a fermé manuellement
    if (state.game_finished && !isShowingWinner && !hasDismissedWinnerPopup) {
        showGameWinner(state.winner_name, state.winner_id, state.pot);
    }

    // Mettre à jour l'en-tête seulement si changé
    if (!lastGameState || lastGameState.phase !== state.phase) {
        document.getElementById('gamePhase').textContent = state.phase;
    }
    if (!lastGameState || lastGameState.pot !== state.pot) {
        document.getElementById('gamePot').textContent = state.pot;
        document.getElementById('potDisplay').textContent = state.pot;
    }
    if (!lastGameState || lastGameState.current_bet !== state.current_bet) {
        document.getElementById('currentBet').textContent = state.current_bet;
    }

    // Cartes communes - ne re-render que si changées
    if (!lastGameState || JSON.stringify(lastGameState.community_cards) !== JSON.stringify(state.community_cards)) {
        renderCommunityCards(state.community_cards);
    }

    // Joueurs - IMPORTANT: ne pas tout re-render à chaque fois
    renderPlayers(state.players, state.current_player_id, state.action_log);

    // Désactiver le bouton start si la partie a démarré
    const startBtn = document.getElementById('startGameBtn');
    if (state.phase !== 'preflop' || state.pot > 0) {
        startBtn.disabled = true;
        startBtn.textContent = '✅ Partie en cours';
    }

    // Afficher le log des actions en temps réel - ne mettre à jour que si changé
    if (state.action_log && state.action_log.length > 0) {
        if (!lastGameState || JSON.stringify(lastGameState.action_log) !== JSON.stringify(state.action_log)) {
            const logContent = document.getElementById('actionLog');
            // Vider le log actuel
            logContent.innerHTML = '';

            // Afficher les actions dans l'ordre inverse (plus récent en haut)
            state.action_log.slice().reverse().forEach(entry => {
                const logEntry = document.createElement('div');
                logEntry.className = 'log-entry action';
                logEntry.textContent = entry;
                logContent.appendChild(logEntry);
            });
        }
    }

    // Sauvegarder l'état actuel
    lastGameState = JSON.parse(JSON.stringify(state));
}

function showHandWinner(winnerName, amount, handDescription, cards) {
    if (!winnerName || isShowingWinner) return;

    isShowingWinner = true;

    // Générer HTML pour les cartes
    let cardsHtml = '';
    if (cards && cards.length > 0) {
        cardsHtml = '<div class="winner-cards">';
        cards.forEach(card => {
            const color = isRedCard(card) ? 'red' : 'black';
            cardsHtml += `<div class="card ${color}">${card}</div>`;
        });
        cardsHtml += '</div>';
    }

    // Créer l'overlay
    const overlay = document.createElement('div');
    overlay.className = 'winner-overlay';
    overlay.innerHTML = `
        <div class="winner-card">
            <h2>🏆 Gagnant de la Main</h2>
            <div class="winner-name">${winnerName}</div>
            ${cardsHtml}
            <div class="winner-amount">+${amount} jetons</div>
            <div class="winner-hand">${handDescription || 'Main gagnante'}</div>
            <div class="countdown">Reprise dans <span id="countdown">5</span>s...</div>
        </div>
    `;

    document.getElementById('gameView').appendChild(overlay);

    // Compte à rebours
    let countdown = 5;
    const countdownEl = document.getElementById('countdown');
    const interval = setInterval(() => {
        countdown--;
        if (countdownEl) countdownEl.textContent = countdown;
        if (countdown <= 0) {
            clearInterval(interval);
            overlay.remove();
            isShowingWinner = false;
        }
    }, 1000);
}


function showGameWinner(winnerName, winnerId, potAmount) {
    if (isShowingWinner) return;
    isShowingWinner = true;

    // Créer l'overlay
    const overlay = document.createElement('div');
    overlay.className = 'winner-overlay';
    overlay.id = 'gameWinnerOverlay';
    overlay.innerHTML = `
        <div class="winner-card" style="border-color: #ff6b6b; box-shadow: 0 0 50px rgba(255, 107, 107, 0.3);">
            <h2>🏆 PARTIE TERMINÉE !</h2>
            <div class="winner-name">${winnerName || 'Inconnu'}</div>
            <div class="winner-hand">A remporté le tournoi !</div>
            <div class="winner-amount">Gains totaux: ${potAmount} jetons</div>
            
            <div class="action-buttons" style="margin-top: 30px; justify-content: center; gap: 10px;">
                <button class="btn btn-primary" onclick="backToLobby()">Retour au Lobby</button>
                <button class="btn btn-secondary" onclick="dismissWinnerPopup()">👀 Voir l'historique</button>
            </div>
        </div>
    `;

    document.getElementById('gameView').appendChild(overlay);
}

function dismissWinnerPopup() {
    const overlay = document.getElementById('gameWinnerOverlay');
    if (overlay) {
        overlay.remove();
    }
    isShowingWinner = false;
    hasDismissedWinnerPopup = true;

    // Ouvrir la sidebar d'historique automatiquement
    const sidebar = document.getElementById('actionLogSidebar');
    if (sidebar.classList.contains('collapsed')) {
        toggleSidebar();
    }
}

function renderCommunityCards(cards) {
    const container = document.getElementById('communityCards');

    if (cards.length === 0) {
        container.innerHTML = '<div style="color: rgba(255,255,255,0.5);">En attente...</div>';
        return;
    }

    container.innerHTML = cards.map(card => {
        const color = isRedCard(card) ? 'red' : 'black';
        return `<div class="card ${color}">${card}</div>`;
    }).join('');
}

function renderPlayers(players, currentPlayerId, actionLog) {
    const container = document.getElementById('playersContainer');

    // Si le nombre de joueurs a changé, on doit tout re-créer
    const existingPlayers = container.querySelectorAll('.player');
    if (existingPlayers.length !== players.length) {
        container.innerHTML = '';
        players.forEach((player, index) => {
            const playerDiv = createPlayerElement(player, currentPlayerId, actionLog);
            playerDiv.dataset.playerId = player.id;
            container.appendChild(playerDiv);
        });
    } else {
        // Sinon, on met juste à jour les joueurs existants
        players.forEach((player, index) => {
            let playerDiv = container.querySelector(`[data-player-id="${player.id}"]`);
            if (!playerDiv) {
                // Si le joueur n'existe pas, le créer
                playerDiv = createPlayerElement(player, currentPlayerId, actionLog);
                playerDiv.dataset.playerId = player.id;
                container.appendChild(playerDiv);
            } else {
                // Mettre à jour seulement ce qui a changé
                updatePlayerElement(playerDiv, player, currentPlayerId, actionLog);
            }
        });
    }
}

function createPlayerElement(player, currentPlayerId, actionLog) {
    const div = document.createElement('div');
    div.className = 'player';
    updatePlayerElement(div, player, currentPlayerId, actionLog);
    return div;
}

function updatePlayerElement(playerDiv, player, currentPlayerId, actionLog) {
    const isActive = player.id === currentPlayerId;
    const isFolded = player.status === 'Folded';

    // Mettre à jour les classes (toujours safe)
    playerDiv.className = 'player';
    if (isActive) playerDiv.classList.add('active');
    if (isFolded) playerDiv.classList.add('folded');

    // Générer le HTML de contenu
    let cardsHtml = '';
    if (player.cards && player.cards.length > 0) {
        cardsHtml = `
            <div class="player-cards">
                ${player.cards.map(card => {
            const color = isRedCard(card) ? 'red' : 'black';
            return `<div class="card ${color}">${card}</div>`;
        }).join('')}
            </div>
        `;
    } else if (!isFolded && player.status !== 'SittingOut') {
        cardsHtml = `
            <div class="player-cards">
                <div class="card-back">🂠</div>
                <div class="card-back">🂠</div>
            </div>
        `;
    }

    // Récupérer la dernière action
    const lastAction = getPlayerLastAction(player.name, actionLog);

    const newHtml = `
        <div class="player-last-action">${lastAction}</div>
        <div class="player-name">${player.name}</div>
        <div class="player-chips">💰 ${player.chips} jetons</div>
        ${player.current_bet > 0 ? `<div class="player-bet">📊 Mise: ${player.current_bet}</div>` : ''}
        <div class="player-status ${isFolded ? 'status-folded' : ''}">${player.status}</div>
        ${cardsHtml}
    `;

    // Ne mettre à jour le HTML que si changé pour éviter les glitchs d'animation
    if (playerDiv._lastHtml !== newHtml) {
        playerDiv.innerHTML = newHtml;
        playerDiv._lastHtml = newHtml;
    }
}

function isRedCard(card) {
    return card.includes('♥') || card.includes('♦');
}

function getPlayerLastAction(playerName, actionLog) {
    if (!actionLog || !playerName) return '';

    // Chercher la dernière action de ce joueur dans les logs (parcourir à l'envers)
    // Format attendu: "[phase] PlayerName -> ACTION ..."

    for (let i = actionLog.length - 1; i >= 0; i--) {
        const log = actionLog[i];

        // Vérifier si la ligne concerne ce joueur
        // On cherche " PlayerName -> " pour être sûr
        const pattern = ` ${playerName} -> `;
        const index = log.indexOf(pattern);

        if (index !== -1) {
            // Extraire l'action après la flèche
            let actionPart = log.substring(index + pattern.length);

            // L'action peut être "CALL (50)", "RAISE (100)", "FOLD", etc.
            // On veut juste le premier mot ou "ALL-IN"

            // Nettoyer pour affichage
            if (actionPart.includes('FOLD')) return 'FOLD';
            if (actionPart.includes('CHECK')) return 'CHECK';
            if (actionPart.includes('CALL')) return 'CALL';
            if (actionPart.includes('RAISE')) return 'RAISE';
            if (actionPart.includes('ALL-IN')) return 'ALL-IN';

            // Fallback: retourner le début de la chaîne
            return actionPart.split(' ')[0];
        }

        // Support pour l'ancien format au cas où (legacy)
        if (log.startsWith(playerName + ' ')) {
            const parts = log.substring(playerName.length + 1).split(' ');
            let action = parts[0];
            if (action.endsWith('s')) action = action.slice(0, -1);
            return action.toUpperCase();
        }
    }

    return '';
}

// === LOG DES ACTIONS ===

function addLogEntry(message, type = 'action') {
    const logContent = document.getElementById('actionLog');
    const entry = document.createElement('div');
    entry.className = `log-entry ${type}`;

    const timestamp = new Date().toLocaleTimeString('fr-FR');
    entry.textContent = `[${timestamp}] ${message}`;

    logContent.insertBefore(entry, logContent.firstChild);

    // Garder seulement les 50 dernières entrées
    while (logContent.children.length > 50) {
        logContent.removeChild(logContent.lastChild);
    }
}

// === CONTRÔLES DE PARTIE ===

// Démarrer une partie
async function startGame(gameId) {
    if (!gameId) {
        alert('Aucune partie sélectionnée');
        return;
    }

    const startBtn = document.getElementById('startGameBtn');

    // Désactiver immédiatement pour éviter les double-clics
    if (startBtn.disabled) {
        console.log('⚠️ Bouton déjà cliqué, ignoring');
        return;
    }

    startBtn.disabled = true;
    startBtn.textContent = '⏳ Démarrage...';

    try {
        console.log('🎮 Envoi de la demande de démarrage pour:', gameId);
        const response = await fetch(`/api/games/${gameId}/start`, {
            method: 'POST'
        });

        const data = await response.json();

        if (response.ok) {
            addLogEntry('🎮 Partie démarrée !', 'system');
            startBtn.textContent = '✅ Partie en cours';
        } else {
            alert('Erreur: ' + (data.error || 'Impossible de démarrer la partie'));
            // Réactiver si erreur
            startBtn.disabled = false;
            startBtn.textContent = '🎮 Démarrer la Partie';
        }
    } catch (error) {
        console.error('Erreur:', error);
        alert('Erreur de connexion au serveur');
        // Réactiver si erreur
        startBtn.disabled = false;
        startBtn.textContent = '🎮 Démarrer la Partie';
    }
}

// Ajouter un bot de test
async function addTestBot() {
    if (!currentGameId) {
        alert('Aucune partie sélectionnée');
        return;
    }

    const botNames = ['BotAlpha', 'BotBeta', 'BotGamma', 'BotDelta', 'BotOmega'];
    const randomName = botNames[Math.floor(Math.random() * botNames.length)] + Math.floor(Math.random() * 1000);

    try {
        const response = await fetch(`/api/games/${currentGameId}/join`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ bot_name: randomName })
        });

        const data = await response.json();

        if (response.ok) {
            addLogEntry(`🤖 Bot "${randomName}" ajouté à la partie`, 'system');
        } else {
            alert('Erreur: ' + (data.error || 'Impossible d\'ajouter le bot'));
        }
    } catch (error) {
        console.error('Erreur:', error);
        alert('Erreur de connexion au serveur');
    }
}

// Copier le Game ID dans le presse-papier
async function copyGameId() {
    if (!currentGameId) {
        alert('Aucune partie sélectionnée');
        return;
    }

    try {
        await navigator.clipboard.writeText(currentGameId);
        const btn = document.getElementById('copyGameIdBtn');
        const originalText = btn.textContent;
        btn.textContent = '✅ Copié !';
        setTimeout(() => {
            btn.textContent = originalText;
        }, 2000);

        addLogEntry(`📋 Game ID copié: ${currentGameId}`, 'system');
    } catch (error) {
        // Fallback pour les navigateurs plus anciens
        const textArea = document.createElement('textarea');
        textArea.value = currentGameId;
        document.body.appendChild(textArea);
        textArea.select();
        document.execCommand('copy');
        document.body.removeChild(textArea);
        alert('Game ID copié: ' + currentGameId);
    }
}

// === FONCTIONS JOUEUR HUMAIN ===

// Afficher dialogue pour rejoindre comme joueur
function showJoinAsPlayerDialog() {
    const gameId = prompt('Entrez le Game ID de la partie à rejoindre:');
    if (!gameId) return;

    const playerName = prompt('Entrez votre nom:');
    if (!playerName) return;

    joinAsHumanPlayer(gameId, playerName);
}

// Rejoindre une partie en tant que joueur humain
async function joinAsHumanPlayer(gameId, playerName) {
    try {
        const response = await fetch(`/api/games/${gameId}/join`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                bot_name: playerName,
                player_type: 'human'
            })
        });

        const data = await response.json();

        if (response.ok) {
            humanPlayerId = data.player_id;
            humanAuthToken = data.auth_token;
            isHumanPlayer = true;
            currentGameId = gameId;

            console.log('✅ Rejoint en tant que joueur humain:', data);

            // Afficher la vue de jeu
            viewGame(gameId);

            // Afficher les contrôles du joueur humain
            document.getElementById('humanPlayerControls').style.display = 'grid';

            addLogEntry(`🎮 Vous avez rejoint la partie en tant que ${playerName}`, 'system');
        } else {
            alert('Erreur: ' + (data.error || 'Impossible de rejoindre la partie'));
        }
    } catch (error) {
        console.error('Erreur:', error);
        alert('Erreur de connexion au serveur');
    }
}

// Mettre à jour l'interface du joueur humain
function updateHumanPlayerUI(state) {
    const controls = document.getElementById('humanPlayerControls');
    if (!controls) return;

    // S'assurer que les contrôles sont visibles
    if (controls.style.display !== 'grid') {
        controls.style.display = 'grid';
    }

    // Récupérer les données du joueur humain
    // Afficher les cartes du joueur (seulement si changées pour éviter animation en boucle)
    const cardsContainer = document.getElementById('humanPlayerCards');
    if (state.your_cards && state.your_cards.length > 0) {
        const newCardsHtml = state.your_cards.map(card => {
            const color = isRedCard(card) ? 'red' : 'black';
            return `<div class="card ${color}">${card}</div>`;
        }).join('');

        if (cardsContainer.innerHTML !== newCardsHtml) {
            cardsContainer.innerHTML = newCardsHtml;
        }
    } else {
        cardsContainer.innerHTML = '';
    }

    // Mettre à jour les jetons
    document.getElementById('humanPlayerChips').textContent = `💰 ${state.your_chips || 0} jetons`;

    // Gérer les boutons d'action
    const isMyTurn = state.current_player_id === humanPlayerId;
    const actionButtons = document.querySelectorAll('.action-buttons button, .raise-controls');
    const actionMessage = document.getElementById('actionMessage');

    if (isMyTurn && state.valid_actions) {
        // Activer les boutons selon les actions valides
        actionMessage.textContent = '🎯 C\'est votre tour !';
        actionMessage.style.color = '#4CAF50';

        // Cacher tous les boutons d'abord
        document.getElementById('checkBtn').style.display = 'none';
        document.getElementById('callBtn').style.display = 'none';
        document.querySelector('.raise-controls').style.display = 'none';

        state.valid_actions.forEach(action => {
            if (action === 'check') {
                const btn = document.getElementById('checkBtn');
                btn.style.display = 'inline-block';
                btn.disabled = false;
            } else if (action === 'call') {
                const callAmount = state.current_bet - (state.players.find(p => p.id === humanPlayerId)?.current_bet || 0);
                document.getElementById('callAmount').textContent = callAmount;
                const btn = document.getElementById('callBtn');
                btn.style.display = 'inline-block';
                btn.disabled = false;
            } else if (action === 'raise') {
                document.querySelector('.raise-controls').style.display = 'flex';
                // Set minimum raise amount
                const minRaise = state.current_bet || 20; // big blind or current bet
                const input = document.getElementById('raiseAmount');
                input.min = minRaise;
                input.placeholder = `Min: ${minRaise}`;
                document.getElementById('raiseBtn').disabled = false;
            }
        });

        // Fold et All-in toujours disponibles
        document.getElementById('foldBtn').disabled = false;
        document.getElementById('allinBtn').disabled = false;
    } else {
        actionMessage.textContent = isHumanPlayer ? '⏳ En attente de votre tour...' : '';
        actionMessage.style.color = '#999';

        // Désactiver tous les boutons
        actionButtons.forEach(btn => {
            if (btn.tagName === 'BUTTON') {
                btn.disabled = true;
            }
        });
    }
}

// Soumettre une action
async function submitAction(actionType) {
    if (!humanAuthToken || !currentGameId) {
        console.error('Pas de token ou game ID');
        return;
    }

    let action;
    switch (actionType) {
        case 'fold':
            action = { type: 'fold' };
            break;
        case 'check':
            action = { type: 'check' };
            break;
        case 'call':
            action = { type: 'call' };
            break;
        case 'allin':
            action = { type: 'allin' };
            break;
        default:
            console.error('Action inconnue:', actionType);
            return;
    }

    try {
        const response = await fetch(`/api/games/${currentGameId}/action`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                auth_token: humanAuthToken,
                action: action
            })
        });

        const data = await response.json();

        if (response.ok && data.success) {
            console.log('✅ Action soumise:', actionType);
            // Rafraîchir l'état immédiatement
            setTimeout(updateGameState, 200);
        } else {
            alert('Erreur: ' + (data.error || 'Action refusée'));
        }
    } catch (error) {
        console.error('Erreur lors de la soumission:', error);
        alert('Erreur de connexion au serveur');
    }
}

// Soumettre un raise
async function submitRaise() {
    const amount = parseInt(document.getElementById('raiseAmount').value);
    if (!amount || amount <= 0) {
        alert('Veuillez entrer un montant valide');
        return;
    }

    if (!humanAuthToken || !currentGameId) {
        console.error('Pas de token ou game ID');
        return;
    }

    try {
        const response = await fetch(`/api/games/${currentGameId}/action`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                auth_token: humanAuthToken,
                action: { type: 'raise', amount: amount }
            })
        });

        const data = await response.json();

        if (response.ok && data.success) {
            console.log('✅ Raise soumis:', amount);
            document.getElementById('raiseAmount').value = '';
            // Rafraîchir l'état immédiatement
            setTimeout(updateGameState, 200);
        } else {
            alert('Erreur: ' + (data.error || 'Raise refusé'));
        }
    } catch (error) {
        console.error('Erreur lors de la soumission:', error);
        alert('Erreur de connexion au serveur');
    }
}

// === UTILITAIRES ===

// Exposer pour le debug dans la console
window.startGame = startGame;
window.viewGame = viewGame;
window.addTestBot = addTestBot;
