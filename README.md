# YWN7_SOL
Multi-token betting game smart contract---meme in pump names YWN7
# BettingGame Smart Contract

This smart contract is a decentralized betting game based on the Solana blockchain. Users can place bets using tokens like USDT and compete with other players. The contract supports dynamic betting and automatically selects winners based on the number of players in each round.

## Features Overview
1. **Betting Functionality**: Players can place a fixed amount of USDT as a bet and join the current round of the game.
2. **Round Control**: Each round supports up to 10 players. After the round ends, the winner is selected and rewards are distributed.
3. **Prize Pool Management**: The contract stores players' bet funds in a prize pool. The winner receives 70% of the prize pool, while 30% is sent to the treasury.
4. **Random Winner Selection**: The contract uses system time (Unix timestamp) to ensure the selection of a winner is random.
5. **Round Reset**: After each round, the administrator can start a new game round.

## Tech Stack
- **Solana Blockchain**: Built on the Solana blockchain using the Anchor framework.
- **Anchor Framework**: A framework that simplifies Solana program development, making it easier for developers to build and deploy smart contracts.
- **USDT Token Support**: Supports Solana-based USDT tokens as the betting medium.

## Security
- The contract includes standard security checks such as ensuring players haven't placed a bet already, ensuring the number of players per round, and ensuring the fixed betting amount for each round.
- With the security and convenience provided by the Anchor framework, the contract is reliable and secure.

## Deployment Instructions
1. Use **Anchor CLI** tools to build and deploy the program.
2. Deploy on Solana mainnet or testnet.
3. Interact with the smart contract using the `place_bet` and `start_new_round` methods.

## Use Case
- **Players**: Players can use Solana wallets like Phantom or Sollet to transfer USDT to the contract address for betting.
- **Admins**: Admins can control the start and end of game rounds and manage the prize pool.

---

If you want to contribute to or participate in this project, feel free to visit our GitHub repository to fork it or submit an issue.
# BettingGame 智能合约

该智能合约是一个基于 Solana 区块链的去中心化博彩游戏，用户可以使用 USDT 等代币参与游戏，下注并与其他玩家竞争。合约支持动态下注，并根据每轮游戏中的玩家数量自动选择赢家。

## 功能概述
1. **下注功能**：玩家可以将固定金额的 USDT 进行投注，并加入当前轮次的游戏。
2. **回合控制**：每轮最多支持 10 名玩家下注，回合结束后会选出赢家并进行奖励分配。
3. **奖池管理**：合约会将玩家的下注资金存入奖池，赢家将获得 70% 的奖池资金，剩余 30% 将转入国库。
4. **随机选择赢家**：合约使用系统时间（Unix 时间戳）来确保赢家的选择具有一定的随机性。
5. **回合重置**：每轮游戏结束后，管理员可以启动新的游戏回合。

## 技术栈
- **Solana Blockchain**：基于 Solana 区块链实现，采用 Anchor 框架开发。
- **Anchor Framework**：简化 Solana 程序开发的框架，帮助开发者更容易地构建和部署智能合约。
- **USDT 代币支持**：支持基于 Solana 的 USDT 代币作为下注工具。

## 安全性
- 该合约使用了标准的安全机制，如检查玩家是否已经下注、确保每轮游戏的玩家数量、确保每轮游戏下注金额固定等。
- 通过 Anchor 框架提供的安全性和便利性，确保了合约的可靠性。

## 部署方式
1. 使用 **Anchor CLI** 工具构建并部署程序。
2. 在 Solana 主网或测试网上进行部署。
3. 可以通过智能合约的 `place_bet` 和 `start_new_round` 方法进行互动。

## 使用案例
- **玩家参与**：玩家可以通过 Solana 钱包（如 Phantom、Sollet）将 USDT 转移到合约地址进行投注。
- **管理员操作**：管理员可以控制游戏回合的开始与结束，并管理奖池。

---

如果你想参与或贡献本项目，欢迎访问我们的 GitHub 仓库进行 Fork 或提 Issue。
