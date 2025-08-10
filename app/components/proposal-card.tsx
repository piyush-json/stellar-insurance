'use client'

import React from 'react'
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import type { Proposal } from '@/lib/types'
import { formatDateTime, formatRelative } from '@/lib/time'
import { useApp, useStrings } from '@/context/app-context'
import { useToast } from '@/hooks/use-toast'

export interface ProposalCardProps {
  proposal: Proposal
  onAfterAction?: () => void
}

export default function ProposalCard({
  proposal,
  onAfterAction
}: ProposalCardProps) {
  const { sdk, timezone } = useApp()
  const S = useStrings()
  const { toast } = useToast()
  const [optimistic, setOptimistic] = React.useState(proposal)
  const [isVoting, setIsVoting] = React.useState(false)

  React.useEffect(() => setOptimistic(proposal), [proposal])

  const handleVote = async (vote: 'yes' | 'no') => {
    if (isVoting) return

    setIsVoting(true)
    setOptimistic((p) => ({
      ...p,
      votes: { ...p.votes, [vote]: p.votes[vote] + 1 }
    }))

    try {
      const res = await sdk.voteProposal(proposal.id, vote)
      if (!res.ok) {
        setOptimistic(proposal)
        toast({
          title: 'Vote failed',
          description: res.error || 'An error occurred while voting',
          variant: 'destructive'
        })
      } else {
        toast({
          title: 'Vote submitted',
          description: 'Your vote has been recorded successfully'
        })
      }
    } catch (error) {
      setOptimistic(proposal)
      toast({
        title: 'Vote failed',
        description: 'An unexpected error occurred',
        variant: 'destructive'
      })
    } finally {
      setIsVoting(false)
      onAfterAction?.()
    }
  }

  const progressYes =
    optimistic.votes.yes + optimistic.votes.no === 0
      ? 0
      : Math.round(
          (optimistic.votes.yes /
            (optimistic.votes.yes + optimistic.votes.no)) *
            100
        )

  return (
    <Card className='w-full hover:shadow-md transition-shadow'>
      <CardHeader>
        <CardTitle className='flex flex-col sm:flex-row sm:items-center justify-between gap-3'>
          <span className='text-lg font-semibold'>{optimistic.title}</span>
          <Badge variant='outline' className='text-xs'>
            {S.voting_ends}: {formatDateTime(optimistic.endsAt, timezone)} —{' '}
            {formatRelative(optimistic.endsAt)}
          </Badge>
        </CardTitle>
      </CardHeader>
      <CardContent className='space-y-4'>
        <div className='text-sm text-muted-foreground'>
          {optimistic.summary}
        </div>

        <div className='space-y-2'>
          <div className='w-full h-2 bg-muted rounded-full overflow-hidden'>
            <div
              className='h-full bg-primary rounded-full transition-all duration-300'
              style={{ width: `${progressYes}%` }}
              aria-label={`Vote progress: ${progressYes}% yes votes`}
            />
          </div>
          <div className='text-xs text-muted-foreground text-center'>
            {optimistic.votes.yes} yes / {optimistic.votes.no} no
          </div>
        </div>
      </CardContent>
      <CardFooter className='gap-2'>
        <Button
          variant='default'
          disabled={isVoting}
          onClick={() => void handleVote('yes')}
          aria-label='Vote yes on this proposal'
        >
          {isVoting ? 'Voting...' : S.vote_yes}
        </Button>
        <Button
          variant='outline'
          disabled={isVoting}
          onClick={() => void handleVote('no')}
          aria-label='Vote no on this proposal'
        >
          {isVoting ? 'Voting...' : S.vote_no}
        </Button>
      </CardFooter>
    </Card>
  )
}
