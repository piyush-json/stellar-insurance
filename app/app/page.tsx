'use client'
import PoolTicker from '@/components/pool-ticker'
import ProposalCard from '@/components/proposal-card'
import SubscriptionCard from '@/components/subscription-card'
import { useApp, useStrings } from '@/context/app-context'
import { usePolling } from '@/hooks/use-polling'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Skeleton } from '@/components/ui/skeleton'
import WalletModal from '@/components/wallet-modal'
import { RefreshCw } from 'lucide-react'

export default function Page() {
  const { sdk, address } = useApp()
  const S = useStrings()
  const pool = usePolling(() => sdk.getPoolStats(), [], 15000)
  const proposals = usePolling(() => sdk.getProposals(), [], 15000)
  const policies = usePolling(() => sdk.getPolicies(), [], 30000)
  const subs = usePolling(
    () => (address ? sdk.getSubscriptions(address) : sdk.getSubscriptions()),
    [address],
    15000
  )

  const pendingProposals = (proposals.data || [])
    .filter((p) => p.status === 'open')
    .slice(0, 3)
  const userSubscriptions = (subs.data || []).filter(
    (s) => !address || s.subscriber === address
  )

  return (
    <main className='container max-w-6xl mx-auto p-4 space-y-6'>
      <div className='flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4'>
        <h1 className='text-3xl font-bold'>{S.dashboard_title}</h1>
        <WalletModal />
      </div>

      {pool.loading ? (
        <Skeleton className='h-24 w-full rounded-lg' />
      ) : pool.data ? (
        <PoolTicker stats={pool.data} />
      ) : null}

      <div className='grid grid-cols-1 lg:grid-cols-2 gap-6'>
        <Card className='hover:shadow-md transition-shadow'>
          <CardHeader className='flex-row items-center justify-between'>
            <CardTitle className='text-xl'>{S.pending_votes}</CardTitle>
            <Button
              variant='ghost'
              size='icon'
              onClick={() => void proposals.refresh()}
              disabled={proposals.loading}
              aria-label='Refresh proposals'
            >
              <RefreshCw
                className={`h-4 w-4 ${proposals.loading ? 'animate-spin' : ''}`}
              />
            </Button>
          </CardHeader>
          <CardContent className='space-y-4'>
            {proposals.loading ? (
              Array.from({ length: 3 }).map((_, i) => (
                <Skeleton key={i} className='h-32 w-full' />
              ))
            ) : pendingProposals.length > 0 ? (
              pendingProposals.map((p) => (
                <ProposalCard
                  key={p.id}
                  proposal={p}
                  onAfterAction={() => void proposals.refresh()}
                />
              ))
            ) : (
              <div className='text-center py-8 text-muted-foreground'>
                No pending proposals
              </div>
            )}
          </CardContent>
        </Card>

        <Card className='hover:shadow-md transition-shadow'>
          <CardHeader className='flex-row items-center justify-between'>
            <CardTitle className='text-xl'>{S.my_subscriptions}</CardTitle>
            <Button
              variant='ghost'
              size='icon'
              onClick={() => void subs.refresh()}
              disabled={subs.loading}
              aria-label='Refresh subscriptions'
            >
              <RefreshCw
                className={`h-4 w-4 ${subs.loading ? 'animate-spin' : ''}`}
              />
            </Button>
          </CardHeader>
          <CardContent className='space-y-4'>
            {subs.loading ? (
              Array.from({ length: 2 }).map((_, i) => (
                <Skeleton key={i} className='h-24 w-full' />
              ))
            ) : userSubscriptions.length > 0 ? (
              userSubscriptions.map((s) => {
                const policy = (policies.data || []).find(
                  (p) => p.id === s.policyId
                )
                const enrichedSubscription = {
                  ...s,
                  policyName: policy?.title || 'Unknown Policy',
                  premium: policy?.params.premiumAmount || '0',
                  coverage: policy?.params.maxClaimAmount || '0',
                  endDate: s.nextPaymentDue || s.startDate,
                  nextPaymentDate: s.nextPaymentDue || s.startDate,
                  claims: 0,
                  maxClaims: 1,
                  status: (s.status === 'active' ? 'active' : 'expired') as
                    | 'active'
                    | 'expired'
                    | 'cancelled'
                    | 'forfeited'
                    | 'ended'
                }
                return (
                  <SubscriptionCard
                    key={s.id}
                    subscription={enrichedSubscription}
                  />
                )
              })
            ) : (
              <div className='text-center py-8 text-muted-foreground'>
                {address
                  ? 'No subscriptions found'
                  : 'Connect wallet to view subscriptions'}
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </main>
  )
}
