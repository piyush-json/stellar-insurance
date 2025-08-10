import type {
  Address,
  AuditEvent,
  Claim,
  Deposit,
  MockToggles,
  Policy,
  PoolStats,
  Proposal,
  Subscription,
  User,
  MockDataSummary
} from './types'
import { addDays } from './time'

function nowIso() {
  return new Date().toISOString()
}
function idGen(prefix: string) {
  let i = 1
  return () => `${prefix}-${i++}`
}

const genPolicyId = idGen('pol')
const genSubId = idGen('sub')
const genClaimId = idGen('clm')
const genProposalId = idGen('prp')
const genDepositId = idGen('dep')
const genAuditId = idGen('evt')
const genTxId = idGen('tx')

export const toggles: MockToggles = {
  slowResponses: false,
  failNextTx: false,
  forceProposalPass: false,
  forceClaimApprove: false,
  networkFlaky: false
}

export const daoMembers: Address[] = [
  'GCYK...KLMN',
  'GDEF...OPQR',
  'GXYZ...ABCD',
  'G123...4567',
  'GABC...DEFG'
]

export const users: Record<Address, User> = {
  'GCYK...KLMN': {
    address: 'GCYK...KLMN',
    name: 'Lakshmi',
    creditScore: 720,
    status: 'active',
    joinDate: nowIso(),
    isDaoMember: true
  },
  'GABC...WXYZ': {
    address: 'GABC...WXYZ',
    name: 'Ravi',
    creditScore: 650,
    status: 'active',
    joinDate: nowIso(),
    isDaoMember: false
  },
  'GHIJ...STUV': {
    address: 'GHIJ...STUV',
    name: 'Meera',
    creditScore: 680,
    status: 'active',
    joinDate: nowIso(),
    isDaoMember: false
  },
  'GXYZ...ABCD': {
    address: 'GXYZ...ABCD',
    name: 'Arjun',
    creditScore: 750,
    status: 'active',
    joinDate: nowIso(),
    isDaoMember: true
  },
  'G123...4567': {
    address: 'G123...4567',
    name: 'Priya',
    creditScore: 690,
    status: 'active',
    joinDate: nowIso(),
    isDaoMember: true
  },
  'GABC...DEFG': {
    address: 'GABC...DEFG',
    name: 'Vikram',
    creditScore: 710,
    status: 'active',
    joinDate: nowIso(),
    isDaoMember: true
  },
  'G789...0123': {
    address: 'G789...0123',
    name: 'Anjali',
    creditScore: 635,
    status: 'active',
    joinDate: nowIso(),
    isDaoMember: false
  },
  'G456...7890': {
    address: 'G456...7890',
    name: 'Rajesh',
    creditScore: 695,
    status: 'active',
    joinDate: nowIso(),
    isDaoMember: false
  }
}

export const policies: Policy[] = [
  {
    id: genPolicyId(),
    title: 'Crop Loss - Monsoon',
    description:
      'Comprehensive coverage for agricultural losses due to failed monsoon seasons, including crop damage, soil erosion, and irrigation system failures.',
    params: {
      maxClaimAmount: '5000000000',
      interestRate: 500,
      premiumAmount: '20000000',
      premiumCurrency: 'XLM',
      claimCooldownDays: 14,
      investorLockInDays: 30,
      requiresDaoApproval: true,
      creditSlashOnReject: 10
    },
    status: 'active',
    createdAt: nowIso(),
    creator: 'GCYK...KLMN'
  },
  {
    id: genPolicyId(),
    title: 'Livestock Insurance',
    description:
      'Protection for cattle, poultry, and other livestock against disease outbreaks, natural disasters, and theft.',
    params: {
      maxClaimAmount: '3000000000',
      interestRate: 450,
      premiumAmount: '15000000',
      premiumCurrency: 'XLM',
      claimCooldownDays: 10,
      investorLockInDays: 21,
      requiresDaoApproval: true,
      creditSlashOnReject: 8
    },
    status: 'archived',
    createdAt: nowIso(),
    creator: 'GCYK...KLMN'
  },
  {
    id: genPolicyId(),
    title: 'Property Damage - Natural Disasters',
    description:
      'Coverage for residential and commercial properties against earthquakes, floods, hurricanes, and other natural disasters.',
    params: {
      maxClaimAmount: '10000000000',
      interestRate: 600,
      premiumAmount: '50000000',
      premiumCurrency: 'XLM',
      claimCooldownDays: 21,
      investorLockInDays: 45,
      requiresDaoApproval: true,
      creditSlashOnReject: 15
    },
    status: 'active',
    createdAt: nowIso(),
    creator: 'GXYZ...ABCD'
  },
  {
    id: genPolicyId(),
    title: 'Vehicle Insurance - Comprehensive',
    description:
      'Full coverage for automobiles including collision, theft, vandalism, and natural disaster damage.',
    params: {
      maxClaimAmount: '8000000000',
      interestRate: 550,
      premiumAmount: '30000000',
      premiumCurrency: 'XLM',
      claimCooldownDays: 7,
      investorLockInDays: 30,
      requiresDaoApproval: false,
      creditSlashOnReject: 5
    },
    status: 'active',
    createdAt: nowIso(),
    creator: 'G123...4567'
  },
  {
    id: genPolicyId(),
    title: 'Health Insurance - Critical Illness',
    description:
      'Coverage for major medical conditions including cancer, heart disease, and organ transplants.',
    params: {
      maxClaimAmount: '15000000000',
      interestRate: 700,
      premiumAmount: '75000000',
      premiumCurrency: 'XLM',
      claimCooldownDays: 30,
      investorLockInDays: 60,
      requiresDaoApproval: true,
      creditSlashOnReject: 20
    },
    status: 'active',
    createdAt: nowIso(),
    creator: 'GABC...DEFG'
  },
  {
    id: genPolicyId(),
    title: 'Business Interruption',
    description:
      'Protection against business losses due to property damage, equipment failure, or supply chain disruptions.',
    params: {
      maxClaimAmount: '20000000000',
      interestRate: 650,
      premiumAmount: '100000000',
      premiumCurrency: 'XLM',
      claimCooldownDays: 14,
      investorLockInDays: 90,
      requiresDaoApproval: true,
      creditSlashOnReject: 25
    },
    status: 'active',
    createdAt: nowIso(),
    creator: 'GCYK...KLMN'
  },
  {
    id: genPolicyId(),
    title: 'Cyber Liability',
    description:
      'Coverage for data breaches, cyber attacks, and digital asset losses for businesses and individuals.',
    params: {
      maxClaimAmount: '12000000000',
      interestRate: 800,
      premiumAmount: '60000000',
      premiumCurrency: 'XLM',
      claimCooldownDays: 5,
      investorLockInDays: 45,
      requiresDaoApproval: true,
      creditSlashOnReject: 30
    },
    status: 'active',
    createdAt: nowIso(),
    creator: 'GXYZ...ABCD'
  },
  {
    id: genPolicyId(),
    title: 'Travel Insurance - International',
    description:
      'Comprehensive travel protection including medical emergencies, trip cancellation, and lost luggage.',
    params: {
      maxClaimAmount: '5000000000',
      interestRate: 400,
      premiumAmount: '25000000',
      premiumCurrency: 'XLM',
      claimCooldownDays: 3,
      investorLockInDays: 15,
      requiresDaoApproval: false,
      creditSlashOnReject: 3
    },
    status: 'active',
    createdAt: nowIso(),
    creator: 'G123...4567'
  }
]

export const subscriptions: Subscription[] = [
  {
    id: genSubId(),
    policyId: policies[0].id,
    subscriber: 'GABC...WXYZ',
    startDate: nowIso(),
    status: 'active',
    lastPaymentDate: addDays(nowIso(), -7),
    nextPaymentDue: addDays(nowIso(), 0), // due now
    gracePeriodDays: 3
  },
  {
    id: genSubId(),
    policyId: policies[2].id,
    subscriber: 'GHIJ...STUV',
    startDate: addDays(nowIso(), -30),
    status: 'active',
    lastPaymentDate: addDays(nowIso(), -30),
    nextPaymentDue: addDays(nowIso(), 0),
    gracePeriodDays: 3
  },
  {
    id: genSubId(),
    policyId: policies[3].id,
    subscriber: 'G789...0123',
    startDate: addDays(nowIso(), -15),
    status: 'active',
    lastPaymentDate: addDays(nowIso(), -15),
    nextPaymentDue: addDays(nowIso(), 15),
    gracePeriodDays: 3
  },
  {
    id: genSubId(),
    policyId: policies[4].id,
    subscriber: 'G456...7890',
    startDate: addDays(nowIso(), -45),
    status: 'active',
    lastPaymentDate: addDays(nowIso(), -15),
    nextPaymentDue: addDays(nowIso(), 15),
    gracePeriodDays: 3
  },
  {
    id: genSubId(),
    policyId: policies[5].id,
    subscriber: 'GABC...WXYZ',
    startDate: addDays(nowIso(), -60),
    status: 'active',
    lastPaymentDate: addDays(nowIso(), -30),
    nextPaymentDue: addDays(nowIso(), 0),
    gracePeriodDays: 3
  },
  {
    id: genSubId(),
    policyId: policies[6].id,
    subscriber: 'GHIJ...STUV',
    startDate: addDays(nowIso(), -20),
    status: 'active',
    lastPaymentDate: addDays(nowIso(), -20),
    nextPaymentDue: addDays(nowIso(), 10),
    gracePeriodDays: 3
  },
  {
    id: genSubId(),
    policyId: policies[7].id,
    subscriber: 'G789...0123',
    startDate: addDays(nowIso(), -5),
    status: 'active',
    lastPaymentDate: addDays(nowIso(), -5),
    nextPaymentDue: addDays(nowIso(), 25),
    gracePeriodDays: 3
  }
]

export const claims: Claim[] = [
  {
    id: genClaimId(),
    subscriptionId: subscriptions[0].id,
    claimer: 'GABC...WXYZ',
    amount: '1000000000',
    imageHash: 'QmSample',
    description:
      'Crop damage due to insufficient rainfall during monsoon season. Approximately 40% of wheat crop lost.',
    status: 'voting',
    votes: { yes: 3, no: 1 },
    createdAt: nowIso()
  },
  {
    id: genClaimId(),
    subscriptionId: subscriptions[1].id,
    claimer: 'GHIJ...STUV',
    amount: '5000000000',
    imageHash: 'QmSample2',
    description:
      'Property severely damaged by earthquake. Foundation cracked, walls collapsed, roof partially destroyed.',
    status: 'approved',
    votes: { yes: 5, no: 0 },
    createdAt: addDays(nowIso(), -5)
  },
  {
    id: genClaimId(),
    subscriptionId: subscriptions[2].id,
    claimer: 'G789...0123',
    amount: '3000000000',
    imageHash: 'QmSample3',
    description:
      'Vehicle totaled in multi-car accident. Airbags deployed, frame bent, engine compartment damaged.',
    status: 'voting',
    votes: { yes: 2, no: 2 },
    createdAt: addDays(nowIso(), -2)
  },
  {
    id: genClaimId(),
    subscriptionId: subscriptions[3].id,
    claimer: 'G456...7890',
    amount: '8000000000',
    imageHash: 'QmSample4',
    description:
      'Diagnosed with stage 3 cancer requiring immediate surgery and chemotherapy treatment.',
    status: 'approved',
    votes: { yes: 5, no: 0 },
    createdAt: addDays(nowIso(), -10)
  },
  {
    id: genClaimId(),
    subscriptionId: subscriptions[4].id,
    claimer: 'GABC...WXYZ',
    amount: '15000000000',
    imageHash: 'QmSample5',
    description:
      'Factory fire destroyed 60% of production capacity. Equipment damaged, inventory lost, operations halted.',
    status: 'voting',
    votes: { yes: 4, no: 1 },
    createdAt: addDays(nowIso(), -1)
  },
  {
    id: genClaimId(),
    subscriptionId: subscriptions[5].id,
    claimer: 'GHIJ...STUV',
    amount: '7000000000',
    imageHash: 'QmSample6',
    description:
      'Ransomware attack encrypted all company data. IT systems compromised, customer data at risk.',
    status: 'pending',
    votes: { yes: 0, no: 0 },
    createdAt: nowIso()
  }
]

export const proposals: Proposal[] = [
  {
    id: genProposalId(),
    title: 'Approve Crop Loss - Monsoon policy',
    summary:
      'Activate the proposed Crop Loss policy for agricultural insurance coverage',
    kind: 'policy_create',
    refId: policies[0].id,
    createdAt: nowIso(),
    endsAt: addDays(nowIso(), 3),
    status: 'open',
    votes: { yes: 5, no: 2 }
  },
  {
    id: genProposalId(),
    title: 'Increase Pool Yield Rate',
    summary:
      'Proposal to increase the annual yield rate from 7.5% to 8.5% to attract more investors',
    kind: 'pool_config',
    refId: 'pool-yield',
    createdAt: addDays(nowIso(), -2),
    endsAt: addDays(nowIso(), 1),
    status: 'open',
    votes: { yes: 3, no: 4 }
  },
  {
    id: genProposalId(),
    title: 'Approve Property Damage Policy',
    summary:
      'Activate comprehensive property insurance against natural disasters',
    kind: 'policy_create',
    refId: policies[2].id,
    createdAt: addDays(nowIso(), -5),
    endsAt: addDays(nowIso(), -2),
    status: 'passed',
    votes: { yes: 6, no: 1 }
  },
  {
    id: genProposalId(),
    title: 'Reduce Minimum Investment Amount',
    summary:
      'Lower the minimum investment amount from 10 XLM to 5 XLM to increase accessibility',
    kind: 'pool_config',
    refId: 'min-investment',
    createdAt: addDays(nowIso(), -7),
    endsAt: addDays(nowIso(), -4),
    status: 'failed',
    votes: { yes: 2, no: 5 }
  },
  {
    id: genProposalId(),
    title: 'Approve Health Insurance Policy',
    summary: 'Activate critical illness health insurance coverage for members',
    kind: 'policy_create',
    refId: policies[4].id,
    createdAt: addDays(nowIso(), -10),
    endsAt: addDays(nowIso(), -7),
    status: 'passed',
    votes: { yes: 7, no: 0 }
  },
  {
    id: genProposalId(),
    title: 'Implement Credit Score Requirements',
    summary:
      'Add minimum credit score requirements for policy subscriptions to reduce risk',
    kind: 'pool_config',
    refId: 'credit-requirements',
    createdAt: addDays(nowIso(), -3),
    endsAt: addDays(nowIso(), 0),
    status: 'open',
    votes: { yes: 4, no: 3 }
  }
]

export const poolStats: PoolStats = {
  totalPool: '100000000000',
  totalInvested: '95000000000',
  yieldRate: 750,
  investorShares: [
    { investor: 'GDEF...OPQR', sharePercent: 2500 },
    { investor: 'GHIJ...STUV', sharePercent: 7500 },
    { investor: 'GXYZ...ABCD', sharePercent: 1500 },
    { investor: 'G123...4567', sharePercent: 1200 },
    { investor: 'GABC...DEFG', sharePercent: 800 },
    { investor: 'G789...0123', sharePercent: 600 },
    { investor: 'G456...7890', sharePercent: 400 }
  ]
}

export const deposits: Deposit[] = [
  {
    id: genDepositId(),
    investor: 'GDEF...OPQR',
    amount: '50000000000',
    createdAt: addDays(nowIso(), -45),
    lockEndsAt: addDays(nowIso(), -15),
    status: 'locked'
  },
  {
    id: genDepositId(),
    investor: 'GHIJ...STUV',
    amount: '15000000000',
    createdAt: addDays(nowIso(), -30),
    lockEndsAt: addDays(nowIso(), 0),
    status: 'locked'
  },
  {
    id: genDepositId(),
    investor: 'GXYZ...ABCD',
    amount: '30000000000',
    createdAt: addDays(nowIso(), -60),
    lockEndsAt: addDays(nowIso(), -30),
    status: 'locked'
  },
  {
    id: genDepositId(),
    investor: 'G123...4567',
    amount: '24000000000',
    createdAt: addDays(nowIso(), -40),
    lockEndsAt: addDays(nowIso(), -10),
    status: 'locked'
  },
  {
    id: genDepositId(),
    investor: 'GABC...DEFG',
    amount: '16000000000',
    createdAt: addDays(nowIso(), -25),
    lockEndsAt: addDays(nowIso(), 5),
    status: 'locked'
  },
  {
    id: genDepositId(),
    investor: 'G789...0123',
    amount: '12000000000',
    createdAt: addDays(nowIso(), -20),
    lockEndsAt: addDays(nowIso(), 10),
    status: 'locked'
  },
  {
    id: genDepositId(),
    investor: 'G456...7890',
    amount: '8000000000',
    createdAt: addDays(nowIso(), -15),
    lockEndsAt: addDays(nowIso(), 15),
    status: 'locked'
  },
  {
    id: genDepositId(),
    investor: 'GDEF...OPQR',
    amount: '25000000000',
    createdAt: addDays(nowIso(), -10),
    lockEndsAt: addDays(nowIso(), 20),
    status: 'locked'
  },
  {
    id: genDepositId(),
    investor: 'GHIJ...STUV',
    amount: '18000000000',
    createdAt: addDays(nowIso(), -5),
    lockEndsAt: addDays(nowIso(), 25),
    status: 'locked'
  }
]

export const auditTrail: AuditEvent[] = [
  {
    id: genAuditId(),
    time: addDays(nowIso(), -1),
    event: 'policy_created',
    details: "New policy 'Cyber Liability' created by GXYZ...ABCD",
    actor: 'GXYZ...ABCD'
  },
  {
    id: genAuditId(),
    time: addDays(nowIso(), -2),
    event: 'claim_approved',
    details: 'Claim CLM-2 approved for 5.0 XLM payout',
    actor: 'GCYK...KLMN'
  },
  {
    id: genAuditId(),
    time: addDays(nowIso(), -3),
    event: 'investment_deposited',
    details: 'New investment of 18.0 XLM by GHIJ...STUV',
    actor: 'GHIJ...STUV'
  },
  {
    id: genAuditId(),
    time: addDays(nowIso(), -4),
    event: 'proposal_passed',
    details:
      "Proposal 'Approve Property Damage Policy' passed with 6 yes votes",
    actor: 'DAO'
  },
  {
    id: genAuditId(),
    time: addDays(nowIso(), -5),
    event: 'subscription_created',
    details: "New subscription to 'Travel Insurance' by G789...0123",
    actor: 'G789...0123'
  }
]

export async function delayMaybe() {
  if (toggles.networkFlaky && Math.random() < 0.1) {
    throw new Error('Network error')
  }
  if (toggles.slowResponses) {
    await new Promise((r) => setTimeout(r, 1200))
  } else {
    await new Promise((r) => setTimeout(r, 120))
  }
}

export function pushAudit(event: Omit<AuditEvent, 'id' | 'time'>) {
  const evt: AuditEvent = {
    id: genAuditId(),
    time: nowIso(),
    ...event
  }
  auditTrail.unshift(evt)
}

export function nextTxId() {
  return genTxId()
}

// Additional utility functions for testing and development
export function resetMockData() {
  // Reset all ID generators to start from 1
  const resetIdGen = (prefix: string) => {
    let i = 1
    return () => `${prefix}-${i++}`
  }

  // Reassign the global generators
  Object.assign(globalThis, {
    genPolicyId: resetIdGen('pol'),
    genSubId: resetIdGen('sub'),
    genClaimId: resetIdGen('clm'),
    genProposalId: resetIdGen('prp'),
    genDepositId: resetIdGen('dep'),
    genAuditId: resetIdGen('evt'),
    genTxId: resetIdGen('tx')
  })
}

export function getMockDataSummary(): MockDataSummary {
  return {
    users: Object.keys(users).length,
    policies: policies.length,
    subscriptions: subscriptions.length,
    claims: claims.length,
    proposals: proposals.length,
    deposits: deposits.length,
    auditEvents: auditTrail.length,
    poolStats: {
      totalPool: poolStats.totalPool,
      totalInvested: poolStats.totalInvested,
      yieldRate: poolStats.yieldRate
    }
  }
}

export function findUserByAddress(address: Address): User | undefined {
  return users[address]
}

export function findPolicyById(id: string): Policy | undefined {
  return policies.find((p) => p.id === id)
}

export function findSubscriptionById(id: string): Subscription | undefined {
  return subscriptions.find((s) => s.id === id)
}

export function findClaimById(id: string): Claim | undefined {
  return claims.find((c) => c.id === id)
}

export function findProposalById(id: string): Proposal | undefined {
  return proposals.find((p) => p.id === id)
}

export function getActivePolicies(): Policy[] {
  return policies.filter((p) => p.status === 'active')
}

export function getPendingClaims(): Claim[] {
  return claims.filter((c) => c.status === 'voting' || c.status === 'pending')
}

export function getOpenProposals(): Proposal[] {
  return proposals.filter((p) => p.status === 'open')
}

export function getDaoMembers(): User[] {
  return Object.values(users).filter((u) => u.isDaoMember)
}

export function getAllMockData() {
  return {
    users,
    policies,
    subscriptions,
    claims,
    proposals,
    deposits,
    auditTrail,
    poolStats,
    toggles,
    daoMembers
  }
}
