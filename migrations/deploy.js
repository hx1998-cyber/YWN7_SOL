const anchor = require('@project-serum/anchor');

async function main() {
    // 设置 Anchor 配置
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);

    // 加载程序 IDL
    const programId = new anchor.web3.PublicKey('YOUR_PROGRAM_ID');
    const program = new anchor.Program(idl, programId, provider);

    // 获取必要的账户和初始化数据
    const tx = await program.rpc.startNewRound({
        accounts: {
            bettingGame: bettingGamePublicKey,
            admin: provider.wallet.publicKey,
        },
    });

    console.log("Transaction signature", tx);
}

main().then(() => console.log('Deployment complete!')).catch(err => console.error(err));
