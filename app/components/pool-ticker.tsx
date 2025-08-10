'use client'
import type { PoolStats } from '@/lib/types'
import { Card, CardContent } from '@/components/ui/card'
import { useStrings } from '@/context/app-context'

export interface PoolTickerProps {
  stats: PoolStats
}

export default function PoolTicker({ stats }: PoolTickerProps) {
  const S = useStrings()
  const availableToWithdraw =
    Number(stats.totalPool) - Number(stats.totalInvested)
  const fmtAmount = (n: string | number) => `${Number(n) / 1e7} XLM`
  const fmtYield = (bps: number) => `${(bps / 100).toFixed(2)}%`

  return (
    <Card className='w-full hover:shadow-md transition-shadow'>
      <CardContent className='p-6 grid grid-cols-1 sm:grid-cols-3 gap-6'>
        <div className='rounded-lg border p-4 bg-muted/30'>
          <div className='text-sm font-medium text-muted-foreground mb-2'>
            {S.pool_total}
          </div>
          <div className='text-2xl font-bold'>{fmtAmount(stats.totalPool)}</div>
        </div>
        <div className='rounded-lg border p-4 bg-muted/30'>
          <div className='text-sm font-medium text-muted-foreground mb-2'>
            {S.yield_rate}
          </div>
          <div className='text-2xl font-bold text-green-600 dark:text-green-400'>
            {fmtYield(stats.yieldRate)}
          </div>
        </div>
        <div className='rounded-lg border p-4 bg-muted/30'>
          <div className='text-sm font-medium text-muted-foreground mb-2'>
            {S.available_to_withdraw}
          </div>
          <div className='text-2xl font-bold'>
            {fmtAmount(availableToWithdraw)}
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
