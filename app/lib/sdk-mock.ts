'use client'

import {
  auditTrail,
  claims,
  daoMembers,
  delayMaybe,
  deposits,
  nextTxId,
  policies,
  poolStats,
  proposals,
  pushAudit,
  subscriptions,
  toggles,
  users
} from './mock-db'
import type {
  Address,
  Claim,
  Deposit,
  ImageUploadResult,
  Policy,
  PoolStats,
  Proposal,
  Subscription,
  TxResult,
  User,
  Vote
} from './types'
import { addDays } from './time'

// Simple event bus
const bus = new EventTarget()
function emit(topic: string) {
  bus.dispatchEvent(new Event(topic))
}
export function onChange(topic: string, cb: () => void) {
  const handler = () => cb()
  bus.addEventListener(topic, handler)
  return () => bus.removeEventListener(topic, handler)
}

export type Sdk = ReturnType<typeof createSdk>

export function createSdk(currentSender?: () => Address | null) {
  return {
    // User Management
    async getUser(address: Address): Promise<User> {
      await delayMaybe()
      const u = users[address]
      if (!u) {
        return {
          address,
          name: 'Guest',
          creditScore: 600,
          status: 'active',
          joinDate: new Date().toISOString(),
          isDaoMember: daoMembers.includes(address)
        }
      }
      return { ...u }
    },

    // Policies
    async getPolicies(): Promise<Policy[]> {
      await delayMaybe()
      return policies.map((p) => ({ ...p }))
    },
    async createPolicy(
      p: Omit<Policy, 'id' | 'status' | 'createdAt' | 'creator'>
    ): Promise<string> {
      await delayMaybe()
      const creator = currentSender?.() || 'UNKNOWN'
      const id = `pol-${Math.floor(Math.random() * 1e6)}`
      const policy: Policy = {
        id,
        title: p.title,
        description: p.description,
        params: p.params,
        status: 'pending',
        createdAt: new Date().toISOString(),
        creator
      }
      policies.unshift(policy)
      // create proposal
      const proposalId = `prp-${Math.floor(Math.random() * 1e6)}`
      proposals.unshift({
        id: proposalId,
        title: `Activate policy: ${p.title}`,
        summary: p.description.slice(0, 120),
        kind: 'policy_create',
        refId: id,
        createdAt: new Date().toISOString(),
        endsAt: addDays(new Date().toISOString(), 3),
        status: 'open',
        votes: { yes: 0, no: 0 }
      })
      pushAudit({
        actor: creator,
        event: 'policy_created',
        details: `Policy ${id} created`
      })
      pushAudit({
        actor: creator,
        event: 'proposal_created',
        details: `Proposal ${proposalId} created for policy ${id}`
      })
      emit('policies')
      emit('proposals')
      return id
    },

    // Subscriptions
    async getSubscriptions(user?: Address): Promise<Subscription[]> {
      await delayMaybe()
      if (user)
        return subscriptions
          .filter((s) => s.subscriber === user)
          .map((s) => ({ ...s }))
      return subscriptions.map((s) => ({ ...s }))
    },
    async subscribeToPolicy(policyId: string): Promise<TxResult> {
      await delayMaybe()
      const sender = currentSender?.()
      if (!sender) return { ok: false, error: 'No wallet connected' }
      const txId = nextTxId()
      if (toggles.failNextTx) {
        toggles.failNextTx = false
        pushAudit({
          actor: sender,
          event: 'error',
          details: `Subscription failed for policy ${policyId}`
        })
        return { ok: false, txId, error: 'Transaction failed' }
      }
      const id = `sub-${Math.floor(Math.random() * 1e6)}`
      const now = new Date().toISOString()
      const policy = policies.find((p) => p.id === policyId)!
      subscriptions.unshift({
        id,
        policyId,
        subscriber: sender,
        startDate: now,
        status: 'active',
        lastPaymentDate: now,
        nextPaymentDue: addDays(now, 7),
        gracePeriodDays: 3
      })
      pushAudit({
        actor: sender,
        event: 'subscription_created',
        details: `Subscription ${id} created for policy ${policyId}`
      })
      emit('subscriptions')
      return { ok: true, txId }
    },

    async payPremium(subscriptionId: string): Promise<TxResult> {
      await delayMaybe()
      const sender = currentSender?.() || 'UNKNOWN'
      const txId = nextTxId()
      if (toggles.failNextTx) {
        toggles.failNextTx = false
        pushAudit({
          actor: sender,
          event: 'error',
          details: `Premium payment failed for subscription ${subscriptionId}`
        })
        return { ok: false, txId, error: 'Premium payment failed' }
      }
      const sub = subscriptions.find((s) => s.id === subscriptionId)
      if (!sub) return { ok: false, txId, error: 'Subscription not found' }
      const now = new Date().toISOString()
      sub.lastPaymentDate = now
      sub.nextPaymentDue = addDays(now, 7)
      pushAudit({
        actor: sender,
        event: 'premium_paid',
        details: `Premium paid for subscription ${subscriptionId}`
      })
      emit('subscriptions')
      return { ok: true, txId }
    },

    // Claims
    async getClaims(): Promise<Claim[]> {
      await delayMaybe()
      return claims.map((c) => ({ ...c }))
    },
    async submitClaim(params: {
      subscriptionId: string
      amount: string
      imageHash: string
      description: string
    }): Promise<TxResult & { claimId?: string }> {
      await delayMaybe()
      const sender = currentSender?.() || 'UNKNOWN'
      const txId = nextTxId()
      const sub = subscriptions.find((s) => s.id === params.subscriptionId)
      if (!sub) return { ok: false, txId, error: 'Subscription not found' }

      // Cooldown enforcement
      const policy = policies.find((p) => p.id === sub.policyId)!
      const cooldownEnds = addDays(
        sub.startDate,
        policy.params.claimCooldownDays
      )
      if (Date.now() < Date.parse(cooldownEnds)) {
        return {
          ok: false,
          txId,
          error: `Cooldown active until ${cooldownEnds}`
        }
      }

      // Double-spend/conflict: if another open/voting exists for same sub
      const conflict = claims.some(
        (c) =>
          c.subscriptionId === params.subscriptionId &&
          (c.status === 'open' || c.status === 'voting')
      )

      const id = `clm-${Math.floor(Math.random() * 1e6)}`
      const newClaim: Claim = {
        id,
        subscriptionId: params.subscriptionId,
        claimer: sender,
        amount: params.amount,
        imageHash: params.imageHash,
        description: params.description,
        status: 'voting',
        votes: { yes: 0, no: 0 },
        createdAt: new Date().toISOString(),
        conflict
      }
      claims.unshift(newClaim)

      // Create proposal representing claim resolution
      proposals.unshift({
        id: `prp-${Math.floor(Math.random() * 1e6)}`,
        title: 'Claim Payout Request',
        summary: params.description.slice(0, 120),
        kind: 'claim_resolution',
        refId: id,
        createdAt: new Date().toISOString(),
        endsAt: addDays(new Date().toISOString(), 2),
        status: 'open',
        votes: { yes: 0, no: 0 }
      })

      pushAudit({
        actor: sender,
        event: 'claim_submitted',
        details: `Claim ${id} submitted`
      })
      emit('claims')
      emit('proposals')
      return { ok: true, txId, claimId: id }
    },
    async voteClaim(claimId: string, vote: Vote): Promise<TxResult> {
      await delayMaybe()
      const sender = currentSender?.() || 'UNKNOWN'
      const txId = nextTxId()
      const c = claims.find((x) => x.id === claimId)
      if (!c) return { ok: false, txId, error: 'Claim not found' }
      if (!daoMembers.includes(sender))
        return { ok: false, txId, error: 'Not a DAO member' }

      // naive dedupe: store a symbol on the claim-side for sender? Not persisting per-user here; simulate error randomly minimal
      if (Math.random() < 0.02)
        return { ok: false, txId, error: 'Duplicate vote' }

      c.votes[vote]++
      pushAudit({
        actor: sender,
        event: 'claim_voted',
        details: `Vote ${vote} cast on claim ${claimId}`
      })
      emit('claims')
      return { ok: true, txId }
    },
    async executePayout(claimId: string): Promise<TxResult> {
      await delayMaybe()
      const sender = currentSender?.() || 'UNKNOWN'
      const txId = nextTxId()
      const c = claims.find((x) => x.id === claimId)
      if (!c) return { ok: false, txId, error: 'Claim not found' }
      if (toggles.failNextTx) {
        toggles.failNextTx = false
        return { ok: false, txId, error: 'Payout failed' }
      }
      c.status = toggles.forceClaimApprove
        ? 'approved'
        : c.votes.yes >= c.votes.no
        ? 'approved'
        : 'rejected'
      pushAudit({
        actor: sender,
        event: 'payout_executed',
        details: `Payout ${c.status} for claim ${claimId}`
      })
      emit('claims')
      return { ok: true, txId }
    },
    async adjustCredit(addr: Address, delta: number): Promise<void> {
      await delayMaybe()
      const u = users[addr]
      if (!u) return
      u.creditScore = Math.max(300, Math.min(900, u.creditScore + delta))
      pushAudit({
        actor: addr,
        event: 'credit_adjusted',
        details: `Credit score adjusted by ${delta}`
      })
      emit('users')
    },

    // Proposals / DAO
    async getProposals(): Promise<Proposal[]> {
      await delayMaybe()
      return proposals.map((p) => ({ ...p }))
    },
    async voteProposal(proposalId: string, vote: Vote): Promise<TxResult> {
      await delayMaybe()
      const sender = currentSender?.() || 'UNKNOWN'
      const txId = nextTxId()
      if (!daoMembers.includes(sender))
        return { ok: false, txId, error: 'Not a DAO member' }
      const p = proposals.find((x) => x.id === proposalId)
      if (!p) return { ok: false, txId, error: 'Proposal not found' }
      p.votes[vote]++
      pushAudit({
        actor: sender,
        event: 'proposal_voted',
        details: `Vote ${vote} cast on proposal ${proposalId}`
      })
      emit('proposals')
      return { ok: true, txId }
    },
    async resolveProposal(proposalId: string): Promise<TxResult> {
      await delayMaybe()
      const txId = nextTxId()
      const p = proposals.find((x) => x.id === proposalId)
      if (!p) return { ok: false, txId, error: 'Proposal not found' }
      const pass = toggles.forceProposalPass ? true : p.votes.yes >= p.votes.no
      p.status = pass ? 'passed' : 'rejected'

      // Apply effects
      if (pass) {
        if (p.kind === 'policy_create' && p.refId) {
          const pol = policies.find((x) => x.id === p.refId)
          if (pol) pol.status = 'active'
        } else if (p.kind === 'policy_archive' && p.refId) {
          const pol = policies.find((x) => x.id === p.refId)
          if (pol) pol.status = 'archived'
        } else if (p.kind === 'policy_delete' && p.refId) {
          const pol = policies.find((x) => x.id === p.refId)
          if (pol) pol.status = 'deleted'
        } else if (p.kind === 'dao_ban_user' && p.refId) {
          const u = users[p.refId as Address]
          if (u) u.status = 'banned'
        }
      }
      pushAudit({
        actor: 'DAO',
        event: 'proposal_resolved',
        details: `Proposal ${proposalId} ${p.status}`
      })
      emit('policies')
      emit('proposals')
      emit('users')
      return { ok: true, txId }
    },
    async proposeDaoChange(
      kind: Proposal['kind'],
      payload: { title: string; summary: string; refId?: string }
    ) {
      await delayMaybe()
      const id = `prp-${Math.floor(Math.random() * 1e6)}`
      proposals.unshift({
        id,
        title: payload.title,
        summary: payload.summary,
        kind,
        refId: payload.refId,
        createdAt: new Date().toISOString(),
        endsAt: addDays(new Date().toISOString(), 2),
        status: 'open',
        votes: { yes: 0, no: 0 }
      })
      emit('proposals')
      pushAudit({
        actor: 'DAO',
        event: 'proposal_created',
        details: `Proposal ${id} created`
      })
      return id
    },

    // Investments
    async getPoolStats(): Promise<PoolStats> {
      await delayMaybe()
      return {
        ...poolStats,
        investorShares: poolStats.investorShares.map((x) => ({ ...x }))
      }
    },
    async getDeposits(addr?: Address): Promise<Deposit[]> {
      await delayMaybe()
      const all = deposits.map((d) => ({ ...d }))
      if (!addr) return all
      return all.filter((d) => d.investor === addr)
    },
    async depositInvestment(amount: string): Promise<TxResult> {
      await delayMaybe()
      const sender = currentSender?.() || 'UNKNOWN'
      const txId = nextTxId()
      if (toggles.failNextTx) {
        toggles.failNextTx = false
        pushAudit({
          actor: sender,
          event: 'error',
          details: `Deposit failed for amount ${amount}`
        })
        return { ok: false, txId, error: 'Deposit failed' }
      }
      const id = `dep-${Math.floor(Math.random() * 1e6)}`
      const now = new Date().toISOString()
      deposits.unshift({
        id,
        investor: sender,
        amount,
        createdAt: now,
        lockEndsAt: addDays(now, 30),
        status: 'locked'
      })
      pushAudit({
        actor: sender,
        event: 'deposit_made',
        details: `Deposit ${id} made for amount ${amount}`
      })
      emit('deposits')
      return { ok: true, txId }
    },
    async withdrawInvestment(depositId: string): Promise<TxResult> {
      await delayMaybe()
      const sender = currentSender?.() || 'UNKNOWN'
      const txId = nextTxId()
      const dep = deposits.find((d) => d.id === depositId)
      if (!dep) return { ok: false, txId, error: 'Deposit not found' }
      if (Date.now() < Date.parse(dep.lockEndsAt)) {
        return { ok: false, txId, error: 'Lock-in active' }
      }
      dep.status = 'withdrawn'
      pushAudit({
        actor: sender,
        event: 'withdrawal',
        details: `Withdrawal from deposit ${depositId}`
      })
      emit('deposits')
      return { ok: true, txId }
    },

    // Audit
    async getAuditTrail() {
      await delayMaybe()
      return auditTrail.map((e) => ({ ...e }))
    },

    // Image Upload (Off-chain Mock)
    async uploadImage(file: File): Promise<ImageUploadResult> {
      await delayMaybe()
      // compute SHA-256 hash
      const buf = await file.arrayBuffer()
      const digest = await crypto.subtle.digest('SHA-256', buf)
      const hashArray = Array.from(new Uint8Array(digest))
      const hash = hashArray
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('')
      const url = URL.createObjectURL(new Blob([buf], { type: file.type }))
      return { hash, url }
    },

    // Toggles / Dev
    toggles
  }
}

let currentAddress: Address | null = 'GABC...WXYZ'
export function setCurrentAddress(addr: Address | null) {
  currentAddress = addr
  emit('wallet')
}
export function getCurrentAddress() {
  return currentAddress
}

export const sdk = createSdk(() => currentAddress)
