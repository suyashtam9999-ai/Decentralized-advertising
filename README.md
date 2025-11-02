# Decentralized Advertising

## Project Description

The Decentralized Advertising Platform is a revolutionary ad network that eliminates intermediaries and creates a direct connection between advertisers and viewers. Unlike traditional advertising models where platforms capture most of the value, this system ensures that users who view advertisements are directly rewarded for their attention and time.

Advertisers can create advertisement campaigns by depositing a budget and setting a reward amount per view. Users can then view these advertisements and automatically receive tokens as compensation. All transactions are transparent, immutable, and executed through smart contracts on the Stellar blockchain, ensuring fair distribution of rewards without the need for trusted intermediaries.

The platform tracks advertisement budgets, view counts, and user earnings in real-time, providing complete transparency for both advertisers and users. When an advertisement's budget is exhausted, it automatically becomes inactive, preventing any overspending.

---

## Project Vision

Our vision is to democratize the advertising industry by creating a fair, transparent, and user-centric ecosystem where:

- **Users Own Their Attention**: People should be compensated fairly for the time they spend viewing advertisements, not just the platforms hosting them.

- **Advertisers Get Real Engagement**: Direct interaction with viewers ensures genuine attention rather than bot-driven metrics and fraudulent impressions.

- **Transparency by Design**: Blockchain technology ensures all transactions, views, and rewards are publicly verifiable and cannot be manipulated.

- **Elimination of Middlemen**: By removing intermediaries, we reduce costs for advertisers while increasing rewards for users, creating a win-win situation.

- **Global Accessibility**: Anyone with internet access can participate in the network, either as an advertiser or a viewer, regardless of geographical location or traditional banking access.

We envision a future where the advertising industry operates on principles of fairness, transparency, and mutual benefit, powered by decentralized technology that puts control back in the hands of users.

---

## Key Features

### 1. **Create Advertisement Campaigns**
- Advertisers can create advertisement campaigns by specifying:
  - Reward amount per view
  - Total campaign budget
- Each ad receives a unique ID for tracking
- Authentication required to ensure only legitimate advertisers can create campaigns

### 2. **User Reward System**
- Users earn tokens automatically when viewing advertisements
- Real-time reward distribution through smart contracts
- All earnings are tracked transparently on the blockchain
- Users can view their total earnings and number of ads viewed

### 3. **Budget Management**
- Automatic tracking of remaining advertisement budget
- Ads automatically deactivate when budget is exhausted
- Prevents overspending and ensures budget compliance
- Transparent budget utilization visible to all parties

### 4. **View Tracking & Analytics**
- Total view count for each advertisement
- User engagement statistics
- Active/inactive status for campaigns
- Historical data for performance analysis

### 5. **Decentralized & Trustless**
- No central authority controls the platform
- Smart contract execution ensures fairness
- Immutable records on Stellar blockchain
- Transparent operations that anyone can verify

### 6. **Secure Authentication**
- Address-based authentication for advertisers and viewers
- Protection against unauthorized access
- Cryptographic verification of all transactions

---

## Future Scope

### Phase 1: Enhanced Features
- **Ad Categories**: Implement different advertisement categories (video, banner, interactive)
- **Targeted Advertising**: Allow advertisers to specify demographics and user preferences
- **Rating System**: Enable users to rate advertisements for quality control
- **Advertiser Dashboard**: Create comprehensive analytics dashboard for campaign performance

### Phase 2: Advanced Functionality
- **Auction-Based Ad Placement**: Implement bidding system for premium ad slots
- **Staking Mechanism**: Allow users to stake tokens for higher rewards
- **NFT Integration**: Convert high-performing ads into collectible NFTs
- **Multi-Token Support**: Accept various cryptocurrencies for ad payments

### Phase 3: Ecosystem Expansion
- **Mobile Application**: Develop native mobile apps for iOS and Android
- **Browser Extension**: Create browser plugins for seamless ad viewing
- **Publisher Integration**: Enable websites to integrate the ad network
- **Reputation System**: Implement reputation scores for advertisers and viewers

### Phase 4: Governance & Community
- **DAO Implementation**: Transition to decentralized governance model
- **Community Voting**: Allow token holders to vote on platform decisions
- **Reward Pools**: Create community-funded bonus reward pools
- **Content Moderation**: Implement decentralized content review system

### Phase 5: Cross-Chain & Scalability
- **Cross-Chain Bridge**: Enable interoperability with other blockchain networks
- **Layer 2 Solutions**: Implement scaling solutions for higher transaction throughput
- **AI Integration**: Use machine learning for better ad matching and fraud detection
- **Global Marketplace**: Create international advertising marketplace with multi-language support

---

## Smart Contract Functions

### Core Functions

1. **`create_ad(advertiser: Address, reward_per_view: u64, total_budget: u64) -> u64`**
   - Creates a new advertisement campaign
   - Returns unique advertisement ID
   - Requires advertiser authentication

2. **`view_ad(ad_id: u64, viewer: Address)`**
   - Processes advertisement view
   - Distributes rewards to viewer
   - Updates campaign statistics
   - Requires viewer authentication

3. **`get_ad(ad_id: u64) -> Advertisement`**
   - Retrieves advertisement details
   - Returns campaign statistics and status
   - Public read-only function

4. **`get_user_rewards(user: Address) -> UserRewards`**
   - Fetches user's total earnings
   - Returns view history
   - Public read-only function

---

## Data Structures

### Advertisement
```rust
{
    ad_id: u64,
    advertiser: Address,
    reward_per_view: u64,
    total_budget: u64,
    remaining_budget: u64,
    total_views: u64,
    is_active: bool,
}
```

### UserRewards
```rust
{
    user: Address,
    total_earned: u64,
    ads_viewed: u64,
}
```

---

## Getting Started

### Prerequisites
- Rust toolchain (latest stable version)
- Soroban CLI
- Stellar account with testnet tokens

### Installation
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Soroban CLI
cargo install --locked soroban-cli

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Building the Contract
```bash
# Clone the repository
git clone <repository-url>
cd decentralized-advertising

# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Optimize the WASM (optional)
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/decentralized_ad.wasm
```

### Deploying to Stellar
```bash
# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/decentralized_ad.wasm \
  --source <YOUR_SECRET_KEY> \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"
```

### Interacting with the Contract
```bash
# Create an advertisement
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <ADVERTISER_SECRET_KEY> \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- create_ad \
  --advertiser <ADVERTISER_ADDRESS> \
  --reward_per_view 100 \
  --total_budget 10000

# View an advertisement
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <VIEWER_SECRET_KEY> \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- view_ad \
  --ad_id 1 \
  --viewer <VIEWER_ADDRESS>
```

---

## Use Cases

### For Advertisers
- Launch targeted campaigns with transparent ROI tracking
- Pay only for verified views
- Access global audience without intermediaries
- Real-time campaign analytics

### For Users
- Earn cryptocurrency by viewing advertisements
- Full control over which ads to view
- Transparent reward distribution
- Passive income opportunity

### For Publishers
- Integrate decentralized ad network into websites/apps
- Share revenue with users directly
- No dependency on centralized ad networks
- Censorship-resistant platform

---

## Security Considerations

- All transactions require cryptographic authentication
- Smart contract code is immutable once deployed
- Budget constraints prevent overspending
- Transparent on-chain verification of all operations
- Regular security audits recommended before mainnet deployment

---

## Contributing

We welcome contributions from the community! Here's how you can help:

1. **Report Bugs**: Open an issue describing the bug and how to reproduce it
2. **Suggest Features**: Share your ideas for new features or improvements
3. **Submit Pull Requests**: Fork the repo, make changes, and submit a PR
4. **Improve Documentation**: Help us make the docs clearer and more comprehensive
5. **Test the Platform**: Use the testnet version and provide feedback

### Development Guidelines
- Follow Rust best practices and conventions
- Add tests for new functionality
- Update documentation for any changes
- Ensure code passes all existing tests

---

## License

This project is open-source and available under the MIT License.

---

## Support & Community

- **Documentation**: [docs.example.com](https://docs.example.com)
- **Discord**: Join our community server
- **Twitter**: Follow us for updates
- **Email**: support@example.com

---

## Acknowledgments

Built on Stellar's Soroban smart contract platform. Special thanks to the Stellar Development Foundation and the open-source community.

---
## Contract Details
<img width="1366" height="768" alt="image" src="https://github.com/user-attachments/assets/5c66d281-9914-4aa4-bb86-23e42a514ce0" />





