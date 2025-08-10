'use client'

import React from 'react'
import type { Claim } from '@/lib/types'
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { useApp, useStrings } from '@/context/app-context'
import { useToast } from '@/hooks/use-toast'

export interface ClaimCardProps {
  claim: Claim
  imageUrl?: string
  isDaoMember?: boolean
  onRefresh?: () => void
}

export default function ClaimCard({
  claim,
  imageUrl,
  isDaoMember,
  onRefresh
}: ClaimCardProps) {
  const { sdk } = useApp()
  const S = useStrings()
  const { toast } = useToast()
  const [optimistic, setOptimistic] = React.useState(claim)
  const [isVoting, setIsVoting] = React.useState(false)

  React.useEffect(() => setOptimistic(claim), [claim])

  const vote = async (v: 'yes' | 'no') => {
    if (isVoting) return

    setIsVoting(true)
    setOptimistic((c) => ({ ...c, votes: { ...c.votes, [v]: c.votes[v] + 1 } }))

    try {
      const res = await sdk.voteClaim(claim.id, v)
      if (!res.ok) {
        setOptimistic(claim)
        toast({
          title: 'Vote failed',
          description: res.error,
          variant: 'destructive'
        })
      } else {
        toast({
          title: 'Vote submitted',
          description: 'Your vote has been recorded successfully'
        })
      }
    } catch (error) {
      setOptimistic(claim)
      toast({
        title: 'Vote failed',
        description: 'An unexpected error occurred',
        variant: 'destructive'
      })
    } finally {
      setIsVoting(false)
      onRefresh?.()
    }
  }

  const votePercentage =
    optimistic.votes.yes + optimistic.votes.no === 0
      ? 0
      : Math.round(
          (optimistic.votes.yes /
            (optimistic.votes.yes + optimistic.votes.no)) *
            100
        )

  return (
    <Card className='hover:shadow-md transition-shadow'>
      <CardHeader>
        <CardTitle className='flex items-center justify-between gap-3'>
          <span className='text-lg font-semibold'>Claim #{claim.id}</span>
          <Badge variant={claim.status === 'open' ? 'default' : 'secondary'}>
            {claim.status}
          </Badge>
        </CardTitle>
      </CardHeader>
      <CardContent className='space-y-4'>
        <div className='flex flex-col sm:flex-row gap-4'>
          <img
            src={
              imageUrl ||
              '/placeholder.svg?height=128&width=128&query=claim+image+preview'
            }
            alt={S.file_alt}
            className='w-32 h-32 object-cover rounded-lg border flex-shrink-0'
          />
          <div className='flex-1 space-y-2'>
            <div className='text-sm text-muted-foreground'>
              {claim.description}
            </div>
            <div className='font-semibold'>
              Amount: {Number(claim.amount) / 1e7} XLM
            </div>
            {claim.conflict && (
              <div className='text-destructive text-sm font-medium'>
                ⚠️ {S.double_spend_risk}
              </div>
            )}
          </div>
        </div>

        <div className='space-y-2'>
          <div className='w-full h-2 bg-muted rounded-full overflow-hidden'>
            <div
              className='h-full bg-primary rounded-full transition-all duration-300'
              style={{ width: `${votePercentage}%` }}
              aria-label={`Vote progress: ${votePercentage}% yes votes`}
            />
          </div>
          <div className='text-xs text-muted-foreground text-center'>
            {optimistic.votes.yes} yes / {optimistic.votes.no} no
          </div>
        </div>
      </CardContent>
      <CardFooter className='gap-2'>
        <Button
          disabled={!isDaoMember || isVoting}
          onClick={() => void vote('yes')}
          aria-label='Vote yes on this claim'
        >
          {isVoting ? 'Voting...' : S.vote_yes}
        </Button>
        <Button
          variant='outline'
          disabled={!isDaoMember || isVoting}
          onClick={() => void vote('no')}
          aria-label='Vote no on this claim'
        >
          {isVoting ? 'Voting...' : S.vote_no}
        </Button>
      </CardFooter>
    </Card>
  )
}
