'use client'
import CreditScorePill from '@/components/credit-score-pill'
import SubscriptionCard from '@/components/subscription-card'
import { useApp, useStrings } from '@/context/app-context'
import { usePolling } from '@/hooks/use-polling'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

export default function Page() {
  const { sdk, address } = useApp()
  const S = useStrings()
  const user = usePolling(
    () => (address ? sdk.getUser(address) : sdk.getUser('GABC...WXYZ')),
    [address],
    30000
  )
  const subs = usePolling(
    () => (address ? sdk.getSubscriptions(address) : sdk.getSubscriptions()),
    [address],
    15000
  )
  const policies = usePolling(() => sdk.getPolicies(), [], 30000)

  if (!address)
    return <main className='p-4'>Connect wallet to view profile.</main>

  // Transform subscription data to include policy information
  const enrichedSubscriptions = (subs.data || []).map((subscription) => {
    const policy = (policies.data || []).find(
      (p) => p.id === subscription.policyId
    )
    return {
      ...subscription,
      policyName: policy?.title || 'Unknown Policy',
      premium: policy?.params.premiumAmount || '0',
      coverage: policy?.params.maxClaimAmount || '0',
      endDate: subscription.nextPaymentDue || subscription.startDate, // Use next payment due as end date
      nextPaymentDate: subscription.nextPaymentDue || subscription.startDate,
      claims: 0, // Default values since claims are tracked separately
      maxClaims: 1, // Default value
      status: (subscription.status === 'active' ? 'active' : 'expired') as
        | 'active'
        | 'expired'
        | 'cancelled'
        | 'forfeited'
        | 'ended'
    }
  })

  return (
    <main className='container max-w-5xl mx-auto p-4 space-y-6'>
      <Card>
        <CardHeader className='flex-row items-center justify-between'>
          <CardTitle className='flex items-center gap-2'>
            {user.data?.name}{' '}
            <span className='text-sm text-muted-foreground'>{address}</span>
          </CardTitle>
          {user.data?.status === 'banned' && (
            <Badge variant='destructive'>{S.banned}</Badge>
          )}
        </CardHeader>
        <CardContent className='space-y-3'>
          {user.data && <CreditScorePill score={user.data.creditScore} />}
        </CardContent>
      </Card>
      <div className='grid grid-cols-1 md:grid-cols-2 gap-4'>
        {enrichedSubscriptions.map((subscription) => (
          <SubscriptionCard key={subscription.id} subscription={subscription} />
        ))}
      </div>
    </main>
  )
}
