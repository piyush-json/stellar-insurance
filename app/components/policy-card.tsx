'use client'
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import type { Policy } from '@/lib/types'
import Link from 'next/link'
import { useStrings } from '@/context/app-context'

export interface PolicyCardProps {
  policy: Policy
  onSubscribe?: (policyId: string) => void
}

export default function PolicyCard({ policy, onSubscribe }: PolicyCardProps) {
  const S = useStrings()
  const isArchived = policy.status === 'archived'

  return (
    <Card className='h-full flex flex-col hover:shadow-md transition-shadow'>
      <CardHeader>
        <CardTitle className='flex items-center justify-between gap-2'>
          <span className='text-lg font-semibold'>{policy.title}</span>
          <Badge variant={isArchived ? 'secondary' : 'default'}>
            {isArchived ? S.archive : S.active}
          </Badge>
        </CardTitle>
      </CardHeader>
      <CardContent className='text-sm text-muted-foreground flex-1 space-y-3'>
        <p className='line-clamp-3'>{policy.description}</p>
        <div className='grid grid-cols-1 sm:grid-cols-2 gap-3'>
          <div className='rounded-lg border p-3 bg-muted/30'>
            <div className='text-xs font-medium text-muted-foreground mb-1'>
              Premium
            </div>
            <div className='font-semibold'>
              {Number(policy.params.premiumAmount) / 1e7}{' '}
              {policy.params.premiumCurrency}
            </div>
          </div>
          <div className='rounded-lg border p-3 bg-muted/30'>
            <div className='text-xs font-medium text-muted-foreground mb-1'>
              Max Claim
            </div>
            <div className='font-semibold'>
              {Number(policy.params.maxClaimAmount) / 1e7}{' '}
              {policy.params.premiumCurrency}
            </div>
          </div>
        </div>
      </CardContent>
      <CardFooter className='justify-between pt-4'>
        <Link
          href={`/policies/${policy.id}`}
          className='text-sm text-primary hover:underline transition-colors'
        >
          View Details
        </Link>
        <Button
          disabled={isArchived}
          onClick={() => onSubscribe?.(policy.id)}
          aria-label={`Subscribe to ${policy.title}`}
        >
          {S.subscribe}
        </Button>
      </CardFooter>
    </Card>
  )
}
