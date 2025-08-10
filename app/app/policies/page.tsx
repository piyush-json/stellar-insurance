'use client'
import PolicyCard from '@/components/policy-card'
import { useApp, useStrings } from '@/context/app-context'
import { usePolling } from '@/hooks/use-polling'
import { Button } from '@/components/ui/button'
import { Skeleton } from '@/components/ui/skeleton'
import { useToast } from '@/hooks/use-toast'
import { useRouter } from 'next/navigation'
import { RefreshCw } from 'lucide-react'

export default function Page() {
  const { sdk } = useApp()
  const S = useStrings()
  const router = useRouter()
  const { data, refresh, loading } = usePolling(
    () => sdk.getPolicies(),
    [],
    15000
  )
  const { toast } = useToast()

  const subscribe = async (policyId: string) => {
    try {
      const res = await sdk.subscribeToPolicy(policyId)
      if (res.ok) {
        toast({
          title: S.success,
          description: `Successfully subscribed! Transaction: ${res.txId}`
        })
        router.push(`/subscribe/${policyId}`)
      } else {
        toast({
          title: S.failed,
          description: res.error,
          variant: 'destructive'
        })
      }
    } catch (error) {
      toast({
        title: S.failed,
        description: 'An unexpected error occurred',
        variant: 'destructive'
      })
    }
  }

  return (
    <main className='container max-w-6xl mx-auto p-4 space-y-6'>
      <div className='flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4'>
        <h1 className='text-3xl font-bold'>{S.policies}</h1>
        <Button
          variant='outline'
          onClick={() => void refresh()}
          disabled={loading}
          className='flex items-center gap-2'
        >
          <RefreshCw className={`h-4 w-4 ${loading ? 'animate-spin' : ''}`} />
          {S.refresh}
        </Button>
      </div>

      {loading ? (
        <div className='grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6'>
          {Array.from({ length: 6 }).map((_, i) => (
            <Skeleton key={i} className='h-64 w-full rounded-lg' />
          ))}
        </div>
      ) : data && data.length > 0 ? (
        <div className='grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6'>
          {data.map((policy) => (
            <PolicyCard
              key={policy.id}
              policy={policy}
              onSubscribe={subscribe}
            />
          ))}
        </div>
      ) : (
        <div className='text-center py-16'>
          <div className='text-muted-foreground text-lg mb-2'>
            No policies found
          </div>
          <p className='text-muted-foreground'>
            Check back later for new insurance policies.
          </p>
        </div>
      )}
    </main>
  )
}
