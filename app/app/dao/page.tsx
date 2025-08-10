'use client'

import React from 'react'
import ProposalCard from '@/components/proposal-card'
import ClaimCard from '@/components/claim-card'
import { useApp, useStrings } from '@/context/app-context'
import { usePolling } from '@/hooks/use-polling'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { Label } from '@/components/ui/label'
import { Separator } from '@/components/ui/separator'
import { Skeleton } from '@/components/ui/skeleton'
import { RefreshCw, Plus } from 'lucide-react'
import { useToast } from '@/hooks/use-toast'

export default function Page() {
  const { sdk, address } = useApp()
  const S = useStrings()
  const { toast } = useToast()
  const proposals = usePolling(() => sdk.getProposals(), [], 15000)
  const claims = usePolling(() => sdk.getClaims(), [], 15000)
  const [title, setTitle] = React.useState('')
  const [summary, setSummary] = React.useState('')
  const [isDaoMember, setIsDaoMember] = React.useState(false)
  const [isCreating, setIsCreating] = React.useState(false)

  React.useEffect(() => {
    if (address) {
      sdk.getUser(address).then((user) => setIsDaoMember(user.isDaoMember))
    }
  }, [address, sdk])

  const createProposal = async () => {
    if (!title.trim() || !summary.trim()) return

    setIsCreating(true)
    try {
      await sdk.proposeDaoChange('dao_add_member', {
        title,
        summary,
        refId: undefined
      })
      setTitle('')
      setSummary('')
      toast({ title: 'Success', description: 'Proposal created successfully' })
      void proposals.refresh()
    } catch (error) {
      toast({
        title: 'Error',
        description: 'Failed to create proposal',
        variant: 'destructive'
      })
    } finally {
      setIsCreating(false)
    }
  }

  return (
    <main className='container max-w-6xl mx-auto p-4 space-y-6'>
      <div className='flex items-center justify-between'>
        <h1 className='text-3xl font-bold'>{S.dao_console}</h1>
      </div>

      <div className='grid grid-cols-1 lg:grid-cols-2 gap-6'>
        <Card className='hover:shadow-md transition-shadow'>
          <CardHeader className='flex-row items-center justify-between'>
            <CardTitle className='text-xl'>{S.proposals}</CardTitle>
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
            ) : proposals.data && proposals.data.length > 0 ? (
              proposals.data.map((p) => (
                <ProposalCard
                  key={p.id}
                  proposal={p}
                  onAfterAction={() => void proposals.refresh()}
                />
              ))
            ) : (
              <div className='text-center py-8 text-muted-foreground'>
                No proposals found
              </div>
            )}

            <Separator />
            <div className='space-y-4'>
              <div className='font-medium text-lg'>{S.create_proposal}</div>
              <div className='space-y-3'>
                <Label htmlFor='proposal-title'>{S.policy_title}</Label>
                <Input
                  id='proposal-title'
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  placeholder='Enter proposal title'
                  disabled={isCreating}
                />
              </div>
              <div className='space-y-3'>
                <Label htmlFor='proposal-summary'>{S.description}</Label>
                <Textarea
                  id='proposal-summary'
                  value={summary}
                  onChange={(e) => setSummary(e.target.value)}
                  placeholder='Enter proposal description'
                  disabled={isCreating}
                  rows={3}
                />
              </div>
              <Button
                onClick={() => void createProposal()}
                disabled={!title.trim() || !summary.trim() || isCreating}
                className='w-full'
              >
                <Plus className='h-4 w-4 mr-2' />
                {isCreating ? 'Creating...' : S.submit}
              </Button>
            </div>
          </CardContent>
        </Card>

        <Card className='hover:shadow-md transition-shadow'>
          <CardHeader className='flex-row items-center justify-between'>
            <CardTitle className='text-xl'>Claims</CardTitle>
            <Button
              variant='ghost'
              size='icon'
              onClick={() => void claims.refresh()}
              disabled={claims.loading}
              aria-label='Refresh claims'
            >
              <RefreshCw
                className={`h-4 w-4 ${claims.loading ? 'animate-spin' : ''}`}
              />
            </Button>
          </CardHeader>
          <CardContent className='space-y-4'>
            {claims.loading ? (
              Array.from({ length: 3 }).map((_, i) => (
                <Skeleton key={i} className='h-40 w-full' />
              ))
            ) : claims.data && claims.data.length > 0 ? (
              claims.data.map((c) => (
                <ClaimCard
                  key={c.id}
                  claim={c}
                  isDaoMember={isDaoMember}
                  onRefresh={() => void claims.refresh()}
                />
              ))
            ) : (
              <div className='text-center py-8 text-muted-foreground'>
                No claims found
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </main>
  )
}
