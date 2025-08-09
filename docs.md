
A DAO-governed village insurance dApp on Stellar: DAO admins propose & vote policies, villagers subscribe and pay weekly premiums, investors fund the pool and earn a cut, and claims are handled by DAO votes with credit-score consequences.

---

## Actors

* **System Admins (us)** — create initial DAO members manually (off-chain step).
* **DAO Members** — village admins; can propose policies, vote on proposals (add/remove/ban DAO members, approve/deny policies/claims).
* **Users (Villagers)** — sign up, subscribe to policies, pay weekly premiums, request claims, have a credit score.
* **Investors** — add funds to the pool, withdraw after lock-in, earn share of premiums.

---

## High-level Rules & Constraints

* Initial DAO members are created manually by system operators.
* Any change to DAO membership (add/remove/ban) requires a DAO vote proposal and quorum rules handled by contract.
* Policies are immutable once created. They can be **archived** (stop new subscriptions) or **deleted** (after archival and another vote). No in-place editing.
* Users must be approved (optional DAO gating) before policy activation — flow configurable per policy.
* Weekly premium payments are enforced; missing payments can forfeit policy after configurable grace period.
* Claims require 1 image upload (image stored off-chain, hash stored on-chain). Claim payout only after DAO vote success.
* Claim denial triggers credit score slashing algorithm (policy-level parameter).
* Investors' deposits are locked for a policy-configured lock-in period before they can withdraw.
* Claim eligibility has a cooldown from policy start (policy param).

---

## Voting Mechanism

### Vote Types and Quorum Rules

1. **Policy Proposals**
   * Requires DAO member vote
   * Quorum requirement: Base quorum (default: 3 members)
   * Success criteria: More than 50% yes votes AND quorum met
   * Time limit: 7 days for voting period

2. **Claim Approvals**
   * Requires DAO member vote
   * Quorum varies by claim amount:
     * Small claims (≤100 XLM): Base quorum
     * Medium claims (≤1000 XLM): Base quorum × 1.5
     * Large claims (>1000 XLM): Base quorum × 2
   * Success criteria: More than 60% yes votes AND quorum met
   * Time limit: 3 days for voting period

### Vote Lifecycle

1. **Creation**
   * Proposal created with type, description, and execution data
   * Voting period starts immediately
   * All active DAO members notified

2. **Voting Period**
   * Members cast votes with weights
   * Running totals updated
   * Cannot change vote once cast

3. **Resolution**
   * Automatically resolved when:
     * Quorum met AND time remains (early resolution)
     * OR voting period ends
   * If passed: Execute proposal action
   * If failed: Mark as rejected

4. **Execution**
   * Successful votes trigger automatic execution
   * Failed votes may impact credit scores (for claims)
   * Results immutably recorded on-chain

---

## Contract Structures

### Core Data Structures

```rust
#[contracttype]
pub enum DataKey {
    User(Address),
    Policy(u64),
    Subscription(u64),
    Claim(u64),
    DAOProposal(u64),
    DAOVote(DAOVoteKey),
    InvestorPool(Address),
    CreditScore(Address),
    VoteWeight(Address),
    ProposalConfig(ProposalType),
}

#[contracttype]
pub struct User {
    pub address: Address,
    pub credit_score: u32,
    pub status: UserStatus,
    pub join_date: u64,
    pub is_dao_member: bool,
    pub name: String,
}

#[contracttype]
pub enum UserStatus {
    Pending,
    Active,
    Banned,
}

#[contracttype]
pub struct Policy {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub params: PolicyParams,
    pub status: PolicyStatus,
    pub created_at: u64,
    pub creator: Address,
}

#[contracttype]
pub struct PolicyParams {
    pub max_claim_amount: i128,
    pub interest_rate: u32,          // basis points (e.g., 500 = 5%)
    pub premium_amount: i128,
    pub premium_currency: String,     // "XLM"
    pub claim_cooldown_days: u32,
    pub investor_lock_in_days: u32,
    pub requires_dao_approval: bool,
    pub credit_slash_on_reject: u32,
}

#[contracttype]
pub enum PolicyStatus {
    Pending,
    Active,
    Archived,
    Deleted,
}

#[contracttype]
pub struct Subscription {
    pub id: u64,
    pub policy_id: u64,
    pub subscriber: Address,
    pub start_date: u64,
    pub status: SubscriptionStatus,
    pub last_payment_date: u64,
    pub next_payment_due: u64,
}

#[contracttype]
pub struct Claim {
    pub id: u64,
    pub subscription_id: u64,
    pub claimer: Address,
    pub amount: i128,
    pub image_hash: BytesN<32>,
    pub description: String,
    pub status: ClaimStatus,
    pub created_at: u64,
}

#[contracttype]
pub enum ProposalType {
    PolicyCreation,      // Create new insurance policy
    PolicyArchival,      // Stop new subscriptions
    PolicyDeletion,      // Remove archived policy
    UserApproval,        // Approve new user registration
    UserBan,            // Ban user for violations
    GovernanceUpdate,    // Change DAO rules/parameters
    FinancialDecision,   // Treasury/pool management
    ClaimResolution,     // Vote on insurance claim
    EmergencyAction,     // Time-critical actions
    MembershipChange,    // Add/remove DAO member
}

#[contracttype]
pub enum ProposalStatus {
    Open,               // Accepting votes
    Passed,            // Approved and executed
    Failed,            // Rejected due to votes
    Expired,           // Time limit reached
    Executing,         // In process of execution
    Invalid,           // Found to be invalid
}

#[contracttype]
pub struct DAOProposal {
    pub id: u64,
    pub proposer: Address,
    pub proposal_type: ProposalType,
    pub title: String,
    pub description: String,
    pub status: ProposalStatus,
    pub votes_yes: u32,
    pub votes_no: u32,
    pub end_time: u64,
    pub quorum_required: u32,       // Required vote weight
    pub execution_params: Vec<u8>,  // Type-specific parameters
    pub min_voting_power: u32,     // Minimum power to vote
}

#[contracttype]
pub struct InvestorDeposit {
    pub investor: Address,
    pub amount: i128,
    pub deposit_date: u64,
    pub unlock_date: u64,
    pub yield_earned: i128,
}
```

### Contract Interface

```rust
pub trait VillageInsuranceContract {
    // User Management
    fn register_user(env: Env, user: Address) -> Result<bool, Error>;
    fn update_user_status(env: Env, user: Address, new_status: UserStatus) -> Result<bool, Error>;
    
    // DAO Operations
    fn create_proposal(env: Env, proposal: DAOProposal) -> Result<u64, Error>;
    fn vote_proposal(env: Env, proposal_id: u64, voter: Address, vote: bool) -> Result<bool, Error>;
    
    // Policy Management
    fn create_policy(env: Env, policy: Policy) -> Result<u64, Error>;
    fn update_policy_status(env: Env, policy_id: u64, new_status: PolicyStatus) -> Result<bool, Error>;
    
    // Subscription Management
    fn subscribe_to_policy(env: Env, policy_id: u64, subscriber: Address) -> Result<u64, Error>;
    fn pay_premium(env: Env, subscription_id: u64, amount: i128) -> Result<bool, Error>;
    
    // Claims Management
    fn submit_claim(env: Env, subscription_id: u64, amount: i128, image_hash: BytesN<32>, description: String) -> Result<u64, Error>;
    fn vote_claim(env: Env, claim_id: u64, voter: Address, vote: bool) -> Result<bool, Error>;
    
    // Investment Management
    fn deposit_investment(env: Env, investor: Address, amount: i128) -> Result<bool, Error>;
    fn withdraw_investment(env: Env, investor: Address, amount: i128) -> Result<bool, Error>;
    
    // View Functions
    fn get_policy(env: Env, policy_id: u64) -> Result<Policy, Error>;
    fn get_user(env: Env, address: Address) -> Result<User, Error>;
    fn get_subscription(env: Env, sub_id: u64) -> Result<Subscription, Error>;
    fn get_pool_stats(env: Env) -> Result<PoolStats, Error>;
}
```
---

## SDK Integration and Smart Contract Interaction

### TypeScript SDK Types

```typescript
// SDK types matching Soroban contract structures
interface Policy {
    id: string;
    title: string;
    description: string;
    params: {
        maxClaimAmount: string;  // i128 as string
        interestRate: number;    // basis points
        premiumAmount: string;   // i128 as string
        premiumCurrency: string;
        claimCooldownDays: number;
        investorLockInDays: number;
        requiresDaoApproval: boolean;
        creditSlashOnReject: number;
    };
    status: 'pending' | 'active' | 'archived' | 'deleted';
    createdAt: string;
    creator: string;  // Stellar address
}

interface Claim {
    id: string;
    subscriptionId: string;
    claimer: string;  // Stellar address
    amount: string;   // i128 as string
    imageHash: string;
    description: string;
    status: 'open' | 'voting' | 'approved' | 'rejected';
    votes: {
        yes: number;
        no: number;
    };
    createdAt: string;
}

interface PoolStats {
    totalPool: string;          // i128 as string
    totalInvested: string;      // i128 as string
    yieldRate: number;          // basis points
    investorShares: Array<{
        investor: string;       // Stellar address
        sharePercent: number;   // basis points
    }>;
}
```

### SDK Methods and Responses

```typescript
class VillageInsuranceSDK {
    constructor(private contract: Contract) {}

    // User Management
    async getUser(address: string): Promise<User> {
        const result = await this.contract.call("get_user", address);
        return {
            address: result.address,
            name: result.name,
            creditScore: Number(result.credit_score),
            status: this.mapUserStatus(result.status),
            joinDate: new Date(Number(result.join_date) * 1000).toISOString(),
            isDaoMember: result.is_dao_member
        };
    }

    // Policy Management
    async createPolicy(params: PolicyCreateParams): Promise<string> {
        const result = await this.contract.call(
            "create_policy",
            this.encodePolicyParams(params)
        );
        return result.toString();
    }

    async getPolicies(): Promise<Policy[]> {
        const result = await this.contract.call("get_policies");
        return this.decodePolicies(result);
    }

    // Subscription Management
    async subscribeToPolicy(policyId: string): Promise<string> {
        const result = await this.contract.call(
            "subscribe_to_policy",
            policyId,
            this.env.sender()
        );
        return result.toString();
    }

    // Claims Management
    async submitClaim(params: ClaimSubmitParams): Promise<string> {
        const result = await this.contract.call(
            "submit_claim",
            params.subscriptionId,
            params.amount,
            params.imageHash,
            params.description
        );
        return result.toString();
    }

    // Investment Management
    async depositInvestment(amount: string): Promise<boolean> {
        return await this.contract.call(
            "deposit_investment",
            this.env.sender(),
            amount
        );
    }

    // Pool Statistics
    async getPoolStats(): Promise<PoolStats> {
        const result = await this.contract.call("get_pool_stats");
        return this.decodePoolStats(result);
    }

    // Image Upload (Off-chain)
    async uploadImage(file: File): Promise<{hash: string, url: string}> {
        // Implementation using IPFS or other decentralized storage
        // Returns hash for on-chain storage and URL for frontend display
    }
}

---

## Example Contract Responses

### Policy Response

```json
{
    "id": "1",
    "title": "Crop Loss - Monsoon",
    "description": "Cover for failed monsoon crop",
    "params": {
        "maxClaimAmount": "5000000000",     // 5000 XLM
        "interestRate": 500,                // 5.00%
        "premiumAmount": "20000000",        // 20 XLM
        "premiumCurrency": "XLM",
        "claimCooldownDays": 14,
        "investorLockInDays": 30,
        "requiresDaoApproval": true,
        "creditSlashOnReject": 10
    },
    "status": "active",
    "createdAt": "1751328000000",          // Unix timestamp
    "creator": "GCYK...KLMN"               // Stellar address
}
```

### Subscription Response

```json
{
    "id": "1",
    "policyId": "1",
    "subscriber": "GABC...WXYZ",           // Stellar address
    "startDate": "1751328000000",
    "status": "active",
    "lastPaymentDate": "1751328000000",
    "nextPaymentDue": "1751932800000"      // One week later
}
```

### Claim Response

```json
{
    "id": "1",
    "subscriptionId": "1",
    "claimer": "GABC...WXYZ",
    "amount": "1000000000",                // 1000 XLM
    "imageHash": "Qm...",                  // IPFS hash
    "description": "Crop damage due to insufficient rainfall",
    "status": "voting",
    "votes": {
        "yes": 3,
        "no": 1
    },
    "createdAt": "1751328000000"
}
```

### Pool Stats Response

```json
{
    "totalPool": "100000000000",           // 100,000 XLM
    "totalInvested": "95000000000",        // 95,000 XLM
    "yieldRate": 750,                      // 7.50%
    "investorShares": [
        {
            "investor": "GDEF...OPQR",
            "sharePercent": 2500            // 25.00%
        },
        {
            "investor": "GHIJ...STUV",
            "sharePercent": 7500            // 75.00%
        }
    ]
}
```

## Contract Testing

### Unit Tests

```rust
#[test]
fn test_policy_lifecycle() {
    let env = Env::default();
    let contract = VillageInsuranceContract::new(&env);
    
    // Test policy creation
    let policy = Policy {
        title: "Test Policy".into(),
        description: "Test Description".into(),
        params: PolicyParams {
            max_claim_amount: 1000_0000000i128,  // 1000 XLM
            interest_rate: 500,                   // 5%
            premium_amount: 1_0000000i128,        // 1 XLM
            premium_currency: "XLM".into(),
            claim_cooldown_days: 14,
            investor_lock_in_days: 30,
            requires_dao_approval: true,
            credit_slash_on_reject: 10,
        },
        status: PolicyStatus::Pending,
        created_at: env.ledger().timestamp(),
        creator: Address::generate(&env),
    };
    
    let policy_id = contract.create_policy(&env, policy).unwrap();
    
    // Test subscription
    let subscriber = Address::generate(&env);
    let sub_id = contract.subscribe_to_policy(&env, policy_id, subscriber.clone()).unwrap();
    
    // Test premium payment
    assert!(contract.pay_premium(&env, sub_id, 1_0000000i128).unwrap());
    
    // Test claim submission
    let claim_id = contract.submit_claim(
        &env,
        sub_id,
        500_0000000i128,  // 500 XLM claim
        BytesN::from_array(&env, &[0; 32]),
        "Test claim".into(),
    ).unwrap();
    
    // Test claim voting
    let dao_member = Address::generate(&env);
    assert!(contract.vote_claim(&env, claim_id, dao_member, true).unwrap());
}

#[test]
fn test_investment_flow() {
    let env = Env::default();
    let contract = VillageInsuranceContract::new(&env);
    
    let investor = Address::generate(&env);
    let amount = 10000_0000000i128;  // 10,000 XLM
    
    // Test deposit
    assert!(contract.deposit_investment(&env, investor.clone(), amount).unwrap());
    
    // Test pool stats
    let stats = contract.get_pool_stats(&env).unwrap();
    assert_eq!(stats.total_pool, amount);
    
    // Try early withdrawal (should fail)
    env.ledger().set_timestamp(0);
    assert!(contract.withdraw_investment(&env, investor.clone(), amount).is_err());
    
    // Test withdrawal after lock-in
    env.ledger().set_timestamp(31 * 24 * 60 * 60);  // 31 days
    assert!(contract.withdraw_investment(&env, investor.clone(), amount).unwrap());
}
```

### SDK Tests

```typescript
describe('VillageInsuranceSDK', () => {
    let sdk: VillageInsuranceSDK;
    let mockContract: MockSorobanContract;
    
    beforeEach(() => {
        mockContract = new MockSorobanContract();
        sdk = new VillageInsuranceSDK(mockContract);
    });
    
    describe('Policy Management', () => {
        it('should create policy', async () => {
            const policy = {
                title: "Test Policy",
                description: "Test Description",
                params: {
                    maxClaimAmount: "1000000000",
                    interestRate: 500,
                    premiumAmount: "10000000",
                    premiumCurrency: "XLM",
                    claimCooldownDays: 14,
                    investorLockInDays: 30,
                    requiresDaoApproval: true,
                    creditSlashOnReject: 10
                }
            };
            
            mockContract.setNextResponse({
                id: "1",
                status: "pending"
            });
            
            const result = await sdk.createPolicy(policy);
            expect(result).toBe("1");
        });
        
        it('should get policies', async () => {
            mockContract.setNextResponse([{
                id: "1",
                title: "Test Policy",
                status: "active"
            }]);
            
            const policies = await sdk.getPolicies();
            expect(policies).toHaveLength(1);
            expect(policies[0].id).toBe("1");
        });
    });
    
    describe('Claims Management', () => {
        it('should submit claim', async () => {
            const claim = {
                subscriptionId: "1",
                amount: "500000000",
                imageHash: "Qm...",
                description: "Test claim"
            };
            
            mockContract.setNextResponse({
                id: "1",
                status: "open"
            });
            
            const result = await sdk.submitClaim(claim);
            expect(result).toBe("1");
        });
    });
});
```

### Integration Tests

```typescript
describe('VillageInsurance Integration', () => {
    let contract: Contract;
    let sdk: VillageInsuranceSDK;
    
    beforeAll(async () => {
        // Deploy test contract
        contract = await SorobanContract.deploy({
            networkPassphrase: Networks.TESTNET,
            source: await loadContractWasm('village_insurance.wasm')
        });
        
        sdk = new VillageInsuranceSDK(contract);
    });
    
    it('should complete full policy lifecycle', async () => {
        // Create policy
        const policyId = await sdk.createPolicy({
            title: "Test Policy",
            description: "Integration test policy",
            params: {
                maxClaimAmount: "1000000000",
                interestRate: 500,
                premiumAmount: "10000000",
                premiumCurrency: "XLM",
                claimCooldownDays: 14,
                investorLockInDays: 30,
                requiresDaoApproval: true,
                creditSlashOnReject: 10
            }
        });
        
        // Subscribe
        const subId = await sdk.subscribeToPolicy(policyId);
        
        // Pay premium
        await sdk.payPremium(subId, "10000000");
        
        // Submit claim
        const claimId = await sdk.submitClaim({
            subscriptionId: subId,
            amount: "500000000",
            imageHash: "Qm...",
            description: "Integration test claim"
        });
        
        // Vote on claim
        await sdk.voteClaim(claimId, true);
        
        // Verify claim status
        const claim = (await sdk.getClaims())[0];
        expect(claim.status).toBe("approved");
    });
});

## Contract Security & Deployment

### Security Considerations

1. **Access Control**
   - All state-changing functions must verify caller permissions using `env.sender()`
   - DAO operations must verify membership status
   - Premium payments must come from subscription owner
   - Claim voting restricted to active DAO members

2. **Token Safety**
   - Use `i128` for all token amounts to handle large values
   - Implement safe math operations using Soroban's built-in checked arithmetic
   - Validate all token transfers with proper error handling
   - Lock investor deposits for the specified period

3. **State Management**
   - Use proper storage types from `soroban_sdk::storage`
   - Clear expired data to optimize storage usage
   - Maintain proper indices for efficient lookups
   - Handle race conditions in claim processing

### Deployment Flow

#### Contract Preparation

```bash
# Build optimized release
cargo build --target wasm32-unknown-unknown --release

# Generate contract bindings
soroban contract bindings typescript \
    --wasm target/wasm32-unknown-unknown/release/village_insurance.wasm \
    --output ts/src/contracts/
```

#### Contract Deployment

```typescript
import { Contract } from 'stellar-sdk';
const contract = new Contract(server, contractId);

// Initialize contract with initial DAO members
const initResult = await contract.call(
    "initialize",
    initialDaoMembers,
    quorumPercent,
    baseTokenAddress
);
```

#### Contract Upgrade Strategy

* Implement version control in contract state
* Add upgrade authorization checks
* Follow proper upgrade path for data migration
* Test upgrades on testnet before mainnet

### Error Handling

```rust
#[derive(Error)]
pub enum ContractError {
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Invalid amount")]
    InvalidAmount,
    
    #[error("Policy not found")]
    PolicyNotFound,
    
    #[error("Insufficient balance")]
    InsufficientBalance,
    
    #[error("Lock period not ended")]
    LockPeriodNotEnded,
    
    #[error("Invalid state transition")]
    InvalidStateTransition,
}

impl VillageInsuranceContract {
    fn ensure_dao_member(&self, env: &Env, address: Address) -> Result<(), ContractError> {
        let user = self.get_user(address)?;
        if !user.is_dao_member {
            return Err(ContractError::Unauthorized);
        }
        Ok(())
    }
    
    fn ensure_valid_amount(&self, amount: i128) -> Result<(), ContractError> {
        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }
        Ok(())
    }
}
```
---

## Pages & Components (AI-generator targets)

Keep components granular and prop-driven so they can be auto-generated. Use corporate-friendly naming.

### Pages

1. **Landing / Dashboard** — high-level pool stats, pending DAO votes, my subscriptions, investor snapshot.
2. **DAO Console** — proposals list, create proposal modal, voting panel, member management.
3. **Policies Catalog** — list of policies (active/archived), policy detail page with subscribe CTA.
4. **Policy Editor (Propose)** — form to create policy (DAO-only). After submit it becomes a proposal.
5. **User Profile** — credit score, active subscriptions, claim history, banned status.
6. **Subscription Flow** — subscribe wizard, payment confirmation, start date.
7. **Claim Flow** — upload image, fill claim form, submit; status tracker with vote progress.
8. **Investor Dashboard** — deposit, withdraw (with lock-in barn), yield dashboard, transaction history.
9. **Admin Audit Trail** — immutable list of contract events (proposal created, voted, payout executed).

### Reusable Components

* `ProposalCard` — shows summary, time left to vote, progress bar (yes/no %), action buttons.
* `PolicyCard` — key params, subscribe button, archive badge.
* `SubscriptionCard` — next due date, pay button, forfeiture status.
* `ClaimCard` — image thumbnail, votes, vote buttons (enabled for DAO members only).
* `CreditScorePill` — numeric + trend sparkline.
* `PoolTicker` — shows `totalPool`, `yieldRate`, `availableToWithdraw`.
* `TxStatusToast` — shows transient transaction statuses from SDK responses.

---

## Interaction & UX details

* **Optimistic UI:** show local optimistic state on actions (vote, pay premium) with explicit fallback when `sdk` returns failed status.
* **Polling vs Events:** UI should poll `sdk.get*` endpoints every 15s for voting widgets and premium due lists. Provide manual refresh control.
* **Image upload:** show compressed preview, compute hash client-side, upload to off-chain storage, then call `sdk.requestClaim({imageHash})`.
* **Payment UX:** integrate a simulated wallet modal that returns `payerPubKey` and `txId`. For real deployment replace with Stellar wallet flow.
* **Deadlines & timers:** calculate absolute dates from contract fields; always display absolute date/time and relative time (e.g., "Voting ends: Aug 11, 2025 — in 3d 4h").

---

## Policy lifecycle (UI flow)

1. DAO member opens **Policy Editor** → fills params → submits → `sdk.createPolicy()` → creates a **Policy Proposal**.
2. Policy appears in DAO Console as a proposal with status open. DAO votes via `sdk.voteProposal()` until quorum/time elapses.
3. If **passed** → `policy.status` becomes `active`. If **failed** → `status` is `rejected` and UI marks it.
4. Users can `subscribe` to active policies → `sdk.subscribePolicy()` → subscription created and first premium due depending on `startDate`.
5. Policy can be **archived** via DAO proposal — archived prevents new subscriptions, existing subs continue.
6. Policy can be **deleted** only if archived and then passed in DAO vote.

---

## Claim flow (UI + contract hooks)

1. User files claim via **Claim Flow**: uploads image → `sdk.uploadImage()` returns `imageHash` → `sdk.requestClaim()`.
2. Claim appears in DAO Console under claims; voting begins (`sdk.voteClaim()`).
3. Voting UI shows real-time yes/no counts from `sdk.getClaims()`.
4. If `sdk.voteClaim()` results `approved`: UI triggers `sdk.executePayout()` and shows `TxStatusToast`. On success, `sdk.adjustCredit()` may be called to *increase* score slightly.
5. If `rejected`: show reasoning metadata (if provided) and call `sdk.adjustCredit({delta: -policy.params.creditSlash})`.
6. Edge case: simultaneous claims against same subscription -> show conflict and disallow payout if double-spend risk — SDK must return conflict flag.

---

## Investor flow

* Deposit: `sdk.depositPool({investorPubKey, amount})` → deposit locked per `investorLockInDays`. UI shows countdown and expected yield.
* Earn: Investors earn a portion of premiums; `sdk.getPoolStats()` gives `yieldRate` and individual share.
* Withdraw: `sdk.withdrawInvestment({depositId})` allowed only after lock-in ends. UI must prevent early withdraw and show penalty (if any).

---

## DAO member management

* Add/Remove/Ban: each action is a proposal (`sdk.proposeDaoChange()`), requiring voting. UI presents a clear modal summarizing effects (e.g., "Banning removes voting rights immediately after pass").
* Ban user: similar flow; invokes `sdk.banUser()`.

---

## Credit score system

* Displayed prominently on user profile and in decision UIs.
* API: `sdk.getUser()` returns `creditScore`. Changes are via `sdk.adjustCredit()` after claim outcomes or violations.

---

## Edge cases and validation rules (must be in generator tests)

* Claim before cooldown -> block and show cooldown expires on date.
* Investor withdrawal during lock-in -> UI disallows and shows time-left.
* Missing weekly premium: after `gracePeriodDays` mark `subscription` as `forfeited` and alert user.
* DAO voter tries to vote twice → SDK must return error; UI handles dedupe locally.
* Image upload failure -> allow retry, auto-capture EXIF stripped version.
* Network errors / SDK timeouts -> show clear CTA to retry and log to audit trail.

---

## Mocks & Test harness

Because SDK returns dummy data, the AI generator must create fixtures and test harness:

* Mock `sdk` module with deterministic responses for all hook functions above.
* Seed scenario JSONs: `initialDaoMembers`, `policies` (one active, one archived), `subscriptions` (one active with dueNext soon), `claims` (one open voting), `poolStats`.
* Provide a control panel in dev build to flip mock responses (pass/fail proposal, slow responses, txn failures).

---

## Accessibility & Localization

* All dates must show timezone and allow user to switch to local view. Default to Asia/Kolkata.
* All interactive elements keyboard accessible; alt text for uploaded images.
* Strings must be externalized for localization (en-IN default).

---

## Security notes for frontend dev

* Never store private keys in localStorage. Use ephemeral wallet connectors in memory.
* Verify `imageHash` matches uploaded file before calling `sdk.requestClaim()`.
* Treat all SDK-returned data as potentially malicious and sanitize before rendering.

---

## Acceptance criteria (what AI generator must produce)

1. Fully wired pages listed above.
2. Each page calls the appropriate `sdk.*` hook and handles success/fail states.
3. Voting UI with live progress bar and deterministic mock responses.
4. Claim creation uses `sdk.uploadImage()` then `sdk.requestClaim()` and shows vote lifecycle.
5. Investor deposit/withdraw UI respects lock-in and shows earnings calculation from `sdk.getPoolStats()`.
6. A dev-only mock-control panel for simulating contract state and failures.

---

## Sample component prop contracts (small examples)

* `PolicyCard` props:

  * `{policyId, title, description, params, status, onSubscribe(policyId)}`

* `ProposalCard` props:

  * `{proposalId, title, summary, votes:{yes, no}, timeLeft, onVote(vote)}`

* `ClaimCard` props:

  * `{claimId, imageUrl, description, status, votes, onVote}`

---

## Minimal styling guidance

* Clean, modular UI: neutral palette, clear accent color for DAO actions.
* Use clear affordances for DAO-only actions (lock icons, role badges).
* Compact tables for audit trail; cards for summaries.

---

## Final notes (tell it like it is)

* Build the frontend as a state machine: proposals → voting → resolution; subscriptions → payments → claims. Keep on-chain interactions minimal and idempotent.
* The SDK is currently mocked — design components so they can switch from dummy to live with a single adapter swap.
* Assume the Stellar contract enforces finality; keep UI optimistic but always reconcile with `sdk` authoritative reads.

---
