export type Address = string

export interface PolicyParams {
  maxClaimAmount: string // i128 as string
  interestRate: number // basis points
  premiumAmount: string // i128 as string
  premiumCurrency: string
  claimCooldownDays: number
  investorLockInDays: number
  requiresDaoApproval: boolean
  creditSlashOnReject: number
}

export type PolicyStatus = 'pending' | 'active' | 'archived' | 'deleted'

export interface Policy {
  id: string
  title: string
  description: string
  params: PolicyParams
  status: PolicyStatus
  createdAt: string // ISO
  creator: Address
}

export interface Claim {
  id: string
  subscriptionId: string
  claimer: Address
  amount: string // i128
  imageHash: string
  description: string
  status: 'open' | 'voting' | 'pending' | 'approved' | 'rejected'
  votes: { yes: number; no: number }
  createdAt: string // ISO
  // optional rejection reason
  reason?: string
  conflict?: boolean
}

export interface PoolStats {
  totalPool: string
  totalInvested: string
  yieldRate: number
  investorShares: Array<{ investor: Address; sharePercent: number }>
}

export interface User {
  address: Address
  name: string
  creditScore: number
  status: 'active' | 'banned'
  joinDate: string // ISO
  isDaoMember: boolean
}

export interface Subscription {
  id: string
  policyId: string
  subscriber: Address
  startDate: string // ISO
  status: 'active' | 'forfeited' | 'ended'
  lastPaymentDate?: string // ISO
  nextPaymentDue?: string // ISO
  gracePeriodDays?: number
}

export interface Proposal {
  id: string
  title: string
  summary: string
  kind:
    | 'policy_create'
    | 'policy_archive'
    | 'policy_delete'
    | 'dao_add_member'
    | 'dao_remove_member'
    | 'dao_ban_user'
    | 'claim_resolution'
    | 'pool_config'
  refId?: string // links to policy/claim/user
  createdAt: string // ISO
  endsAt: string // ISO
  status: 'open' | 'passed' | 'failed' | 'rejected'
  votes: { yes: number; no: number }
}

export interface Deposit {
  id: string
  investor: Address
  amount: string // i128 string
  createdAt: string // ISO
  lockEndsAt: string // ISO
  status: 'locked' | 'withdrawn'
}

export interface TxResult {
  ok: boolean
  txId?: string
  error?: string
}

export type Vote = 'yes' | 'no'

export interface ImageUploadResult {
  hash: string
  url: string
}

export interface MockToggles {
  slowResponses: boolean
  failNextTx: boolean
  forceProposalPass: boolean
  forceClaimApprove: boolean
  networkFlaky: boolean
}

export interface AuditEvent {
  id: string
  time: string // ISO
  event: string
  details: string
  actor: Address
}

// Additional utility types
export interface MockDataSummary {
  users: number
  policies: number
  subscriptions: number
  claims: number
  proposals: number
  deposits: number
  auditEvents: number
  poolStats: {
    totalPool: string
    totalInvested: string
    yieldRate: number
  }
}

export interface MockDatabase {
  users: Record<Address, User>
  policies: Policy[]
  subscriptions: Subscription[]
  claims: Claim[]
  proposals: Proposal[]
  deposits: Deposit[]
  auditTrail: AuditEvent[]
  poolStats: PoolStats
  toggles: MockToggles
  daoMembers: Address[]
}
