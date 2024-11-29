const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

describe('betting-game', () => {
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);

    const program = anchor.workspace.BettingGame;

    it('Starts a new round', async () => {
        const bettingGame = anchor.web3.Keypair.generate();
        
        // 执行游戏逻辑
        await program.rpc.startNewRound({
            accounts: {
                bettingGame: bettingGame.publicKey,
                admin: provider.wallet.publicKey,
            },
            signers: [bettingGame],
        });

        console.log("Betting Game started!");
    });

    it('Allows a player to place a bet', async () => {
        const player = anchor.web3.Keypair.generate();

        // 下注
        await program.rpc.placeBet(new anchor.BN(1000), {
            accounts: {
                bettingGame: bettingGame.publicKey,
                player: player.publicKey,
                playerToken: playerTokenAccount.publicKey,
                pool: poolTokenAccount.publicKey,
                tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
            },
            signers: [player],
        });

        console.log("Bet placed!");
    });
});
