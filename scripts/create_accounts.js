const anchor = require('@project-serum/anchor');
const { Keypair } = anchor.web3;

async function createAccount() {
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);

    // 创建一个新的密钥对
    const newAccount = Keypair.generate();
    console.log("New Account:", newAccount.publicKey.toBase58());

    // 转账一些 SOL 到新的账户
    const transaction = new anchor.web3.Transaction().add(
        anchor.web3.SystemProgram.transfer({
            fromPubkey: provider.wallet.publicKey,
            toPubkey: newAccount.publicKey,
            lamports: anchor.web3.LAMPORTS_PER_SOL, // 1 SOL
        })
    );

    await provider.sendAndConfirm(transaction, [provider.wallet.payer]);

    console.log("Account created and funded!");
}

createAccount().catch(err => console.error(err));
