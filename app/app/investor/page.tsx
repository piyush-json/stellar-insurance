'use client'

import React, { useState } from 'react'
import { useApp } from '@/context/app-context'
import { usePolling } from '@/hooks/use-polling'
import { useToast } from '@/hooks/use-toast'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Badge } from '@/components/ui/badge'
import { Skeleton } from '@/components/ui/skeleton'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Progress } from '@/components/ui/progress'
import {
  TrendingUp,
  TrendingDown,
  DollarSign,
  Lock,
  Unlock,
  RefreshCw,
  Plus,
  Minus,
  Calendar,
  BarChart3,
  PieChart,
  Activity,
  Target,
  Zap
} from 'lucide-react'
import { formatDateTime, formatRelative } from '@/lib/time'

export default function Page() {
  const { sdk, address } = useApp()
  const { toast } = useToast()
  const [amt, setAmt] = useState('100000000') // 10 XLM
  const [isDepositing, setIsDepositing] = useState(false)

  const pool = usePolling(() => sdk.getPoolStats(), [], 15000)
  const deps = usePolling(
    () => (address ? sdk.getDeposits(address) : sdk.getDeposits()),
    [address],
    15000
  )

  const handleDeposit = async () => {
    if (!sdk || !amt) return

    setIsDepositing(true)
    try {
      const res = await sdk.depositInvestment(amt)
      if (res.ok) {
        toast({
          title: 'Investment Successful',
          description: `Successfully deposited ${Number(amt) / 1e7} XLM`
        })
        void deps.refresh()
        setAmt('100000000')
      } else {
        toast({
          title: 'Investment Failed',
          description: res.error || 'Failed to process investment',
          variant: 'destructive'
        })
      }
    } catch (error) {
      toast({
        title: 'Investment Failed',
        description: 'An unexpected error occurred',
        variant: 'destructive'
      })
    } finally {
      setIsDepositing(false)
    }
  }

  const handleWithdraw = async (id: string) => {
    if (!sdk) return

    try {
      const res = await sdk.withdrawInvestment(id)
      if (res.ok) {
        toast({
          title: 'Withdrawal Successful',
          description: 'Your investment has been withdrawn successfully'
        })
        void deps.refresh()
      } else {
        toast({
          title: 'Withdrawal Failed',
          description: res.error || 'Failed to process withdrawal',
          variant: 'destructive'
        })
      }
    } catch (error) {
      toast({
        title: 'Withdrawal Failed',
        description: 'An unexpected error occurred',
        variant: 'destructive'
      })
    }
  }

  const formatCurrency = (amount: string) => {
    return (Number(amount) / 1e7).toFixed(2)
  }

  const calculateTotalInvested = () => {
    return (deps.data || []).reduce((total, dep) => {
      if (dep.status === 'locked') {
        return total + Number(dep.amount)
      }
      return total
    }, 0)
  }

  const calculateTotalEarnings = () => {
    const totalInvested = calculateTotalInvested()
    const yieldRate = pool.data?.yieldRate || 0
    return (totalInvested * yieldRate) / 10000 // Convert basis points to percentage
  }

  if (!pool.data) {
    return (
      <main className='container max-w-7xl mx-auto p-4 space-y-6'>
        <div className='space-y-4'>
          <Skeleton className='h-8 w-48' />
          <Skeleton className='h-32 w-full' />
        </div>
        <div className='grid grid-cols-1 md:grid-cols-3 gap-6'>
          {[1, 2, 3].map((i) => (
            <Skeleton key={i} className='h-32 w-full' />
          ))}
        </div>
      </main>
    )
  }

  return (
    <main className='container max-w-7xl mx-auto p-4 space-y-6'>
      {/* Header */}
      <div className='space-y-2'>
        <h1 className='text-3xl font-bold text-foreground'>
          Investment Dashboard
        </h1>
        <p className='text-muted-foreground'>
          Manage your investments and track your returns in the Stellar
          Insurance Pool
        </p>
      </div>

      {/* Pool Overview Cards */}
      <div className='grid grid-cols-1 md:grid-cols-3 gap-6'>
        <Card className='hover:shadow-md transition-shadow'>
          <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
            <CardTitle className='text-sm font-medium text-muted-foreground'>
              Total Pool Value
            </CardTitle>
            <DollarSign className='size-4 text-muted-foreground' />
          </CardHeader>
          <CardContent>
            <div className='text-2xl font-bold text-foreground'>
              {formatCurrency(pool.data.totalPool)} XLM
            </div>
            <p className='text-xs text-muted-foreground'>
              Total assets under management
            </p>
          </CardContent>
        </Card>

        <Card className='hover:shadow-md transition-shadow'>
          <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
            <CardTitle className='text-sm font-medium text-muted-foreground'>
              Current Yield Rate
            </CardTitle>
            <TrendingUp className='size-4 text-green-600' />
          </CardHeader>
          <CardContent>
            <div className='text-2xl font-bold text-green-600'>
              {(pool.data.yieldRate / 100).toFixed(2)}%
            </div>
            <p className='text-xs text-muted-foreground'>
              Annual percentage yield
            </p>
          </CardContent>
        </Card>

        <Card className='hover:shadow-md transition-shadow'>
          <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
            <CardTitle className='text-sm font-medium text-muted-foreground'>
              Pool Utilization
            </CardTitle>
            <BarChart3 className='size-4 text-muted-foreground' />
          </CardHeader>
          <CardContent>
            <div className='text-2xl font-bold text-foreground'>
              {(
                (Number(pool.data.totalInvested) /
                  Number(pool.data.totalPool)) *
                100
              ).toFixed(1)}
              %
            </div>
            <p className='text-xs text-muted-foreground'>
              {formatCurrency(pool.data.totalInvested)} /{' '}
              {formatCurrency(pool.data.totalPool)} XLM
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Investment Actions */}
      <Card className='hover:shadow-md transition-shadow'>
        <CardHeader>
          <CardTitle className='flex items-center gap-2'>
            <Plus className='size-5' />
            New Investment
          </CardTitle>
        </CardHeader>
        <CardContent className='space-y-4'>
          <div className='grid grid-cols-1 md:grid-cols-3 gap-4'>
            <div className='space-y-2'>
              <label className='text-sm font-medium text-foreground'>
                Investment Amount (XLM)
              </label>
              <Input
                type='number'
                value={Number(amt) / 1e7}
                onChange={(e) =>
                  setAmt((Number(e.target.value) * 1e7).toString())
                }
                placeholder='10.00'
                min='0'
                step='0.01'
              />
            </div>
            <div className='space-y-2'>
              <label className='text-sm font-medium text-foreground'>
                Lock Period
              </label>
              <div className='text-sm text-muted-foreground p-3 bg-muted/30 rounded-md'>
                30 days minimum
              </div>
            </div>
            <div className='space-y-2'>
              <label className='text-sm font-medium text-foreground'>
                Expected Return
              </label>
              <div className='text-sm text-green-600 font-medium p-3 bg-green-50 dark:bg-green-950/20 rounded-md'>
                {((Number(amt) * pool.data.yieldRate) / 10000 / 1e7).toFixed(4)}{' '}
                XLM/year
              </div>
            </div>
          </div>
          <Button
            onClick={handleDeposit}
            disabled={isDepositing || !amt || Number(amt) <= 0}
            className='w-full md:w-auto'
          >
            {isDepositing ? (
              <>
                <RefreshCw className='size-4 mr-2 animate-spin' />
                Processing...
              </>
            ) : (
              <>
                <Plus className='size-4 mr-2' />
                Invest Now
              </>
            )}
          </Button>
        </CardContent>
      </Card>

      {/* Investment Details Tabs */}
      <Tabs defaultValue='overview' className='space-y-4'>
        <TabsList className='grid w-full grid-cols-3'>
          <TabsTrigger value='overview'>Overview</TabsTrigger>
          <TabsTrigger value='deposits'>My Deposits</TabsTrigger>
          <TabsTrigger value='analytics'>Analytics</TabsTrigger>
        </TabsList>

        <TabsContent value='overview' className='space-y-4'>
          <div className='grid grid-cols-1 md:grid-cols-2 gap-6'>
            <Card className='hover:shadow-md transition-shadow'>
              <CardHeader>
                <CardTitle className='flex items-center gap-2'>
                  <Target className='size-5' />
                  Investment Summary
                </CardTitle>
              </CardHeader>
              <CardContent className='space-y-4'>
                <div className='space-y-3'>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Total Invested
                    </span>
                    <span className='font-semibold text-foreground'>
                      {formatCurrency(calculateTotalInvested().toString())} XLM
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Total Earnings
                    </span>
                    <span className='font-semibold text-green-600'>
                      {formatCurrency(calculateTotalEarnings().toString())} XLM
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Active Deposits
                    </span>
                    <span className='font-semibold text-foreground'>
                      {
                        (deps.data || []).filter((d) => d.status === 'locked')
                          .length
                      }
                    </span>
                  </div>
                </div>
                <Progress
                  value={
                    (calculateTotalInvested() / Number(pool.data.totalPool)) *
                    100
                  }
                  className='w-full'
                />
                <p className='text-xs text-muted-foreground text-center'>
                  Your share of the total pool
                </p>
              </CardContent>
            </Card>

            <Card className='hover:shadow-md transition-shadow'>
              <CardHeader>
                <CardTitle className='flex items-center gap-2'>
                  <Activity className='size-5' />
                  Performance Metrics
                </CardTitle>
              </CardHeader>
              <CardContent className='space-y-4'>
                <div className='space-y-3'>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Current APY
                    </span>
                    <span className='font-semibold text-green-600'>
                      {(pool.data.yieldRate / 100).toFixed(2)}%
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Risk Level
                    </span>
                    <Badge
                      variant='secondary'
                      className='bg-yellow-100 text-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-400'
                    >
                      Medium
                    </Badge>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Liquidity
                    </span>
                    <Badge
                      variant='default'
                      className='bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400'
                    >
                      High
                    </Badge>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>

        <TabsContent value='deposits' className='space-y-4'>
          <Card className='hover:shadow-md transition-shadow'>
            <CardHeader className='flex flex-row items-center justify-between'>
              <CardTitle className='flex items-center gap-2'>
                <Lock className='size-5' />
                Active Deposits
              </CardTitle>
              <Button
                variant='ghost'
                onClick={() => void deps.refresh()}
                disabled={deps.loading}
              >
                <RefreshCw
                  className={`size-4 mr-2 ${
                    deps.loading ? 'animate-spin' : ''
                  }`}
                />
                Refresh
              </Button>
            </CardHeader>
            <CardContent className='space-y-4'>
              {deps.loading ? (
                <div className='space-y-3'>
                  {[1, 2, 3].map((i) => (
                    <Skeleton key={i} className='h-20 w-full' />
                  ))}
                </div>
              ) : (deps.data || []).length === 0 ? (
                <div className='text-center py-8 text-muted-foreground'>
                  <Lock className='size-12 mx-auto mb-4 opacity-50' />
                  <p>No active deposits found</p>
                  <p className='text-sm'>
                    Start investing to see your deposits here
                  </p>
                </div>
              ) : (
                <div className='space-y-3'>
                  {(deps.data || []).map((d) => {
                    const lockActive = Date.now() < Date.parse(d.lockEndsAt)
                    const lockEndDate = new Date(d.lockEndsAt)
                    const now = new Date()
                    const daysRemaining = Math.ceil(
                      (lockEndDate.getTime() - now.getTime()) /
                        (1000 * 60 * 60 * 24)
                    )

                    return (
                      <div
                        key={d.id}
                        className='border rounded-lg p-4 hover:shadow-sm transition-shadow'
                      >
                        <div className='flex flex-col lg:flex-row lg:items-center justify-between gap-4'>
                          <div className='space-y-3 flex-1'>
                            <div className='flex items-center gap-3'>
                              <Badge
                                variant={lockActive ? 'secondary' : 'default'}
                              >
                                {lockActive ? (
                                  <>
                                    <Lock className='size-3 mr-1' />
                                    Locked
                                  </>
                                ) : (
                                  <>
                                    <Unlock className='size-3 mr-1' />
                                    Available
                                  </>
                                )}
                              </Badge>
                              <span className='text-sm text-muted-foreground'>
                                ID: {d.id.slice(0, 8)}...
                              </span>
                            </div>

                            <div className='grid grid-cols-2 md:grid-cols-4 gap-4'>
                              <div>
                                <p className='text-sm text-muted-foreground'>
                                  Amount
                                </p>
                                <p className='font-semibold text-foreground'>
                                  {formatCurrency(d.amount)} XLM
                                </p>
                              </div>
                              <div>
                                <p className='text-sm text-muted-foreground'>
                                  Created
                                </p>
                                <p className='text-sm text-foreground'>
                                  {formatDateTime(d.createdAt, 'UTC')}
                                </p>
                              </div>
                              <div>
                                <p className='text-sm text-muted-foreground'>
                                  Lock Ends
                                </p>
                                <p className='text-sm text-foreground'>
                                  {formatDateTime(d.lockEndsAt, 'UTC')}
                                </p>
                              </div>
                              <div>
                                <p className='text-sm text-muted-foreground'>
                                  {lockActive ? 'Days Remaining' : 'Unlocked'}
                                </p>
                                <p
                                  className={`text-sm font-medium ${
                                    lockActive
                                      ? 'text-orange-600'
                                      : 'text-green-600'
                                  }`}
                                >
                                  {lockActive
                                    ? `${daysRemaining} days`
                                    : 'Ready'}
                                </p>
                              </div>
                            </div>
                          </div>

                          <div className='flex flex-col gap-2'>
                            <Button
                              disabled={lockActive || d.status === 'withdrawn'}
                              onClick={() => void handleWithdraw(d.id)}
                              variant='outline'
                              className='w-full lg:w-auto'
                            >
                              <Minus className='size-4 mr-2' />
                              Withdraw
                            </Button>
                            {lockActive && (
                              <p className='text-xs text-muted-foreground text-center'>
                                Available in {daysRemaining} days
                              </p>
                            )}
                          </div>
                        </div>
                      </div>
                    )
                  })}
                </div>
              )}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value='analytics' className='space-y-4'>
          <div className='grid grid-cols-1 md:grid-cols-2 gap-6'>
            <Card className='hover:shadow-md transition-shadow'>
              <CardHeader>
                <CardTitle className='flex items-center gap-2'>
                  <TrendingUp className='size-5' />
                  Yield Projection
                </CardTitle>
              </CardHeader>
              <CardContent className='space-y-4'>
                <div className='space-y-3'>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      30 Days
                    </span>
                    <span className='font-semibold text-foreground'>
                      {(
                        (calculateTotalInvested() * pool.data.yieldRate) /
                        10000 /
                        12 /
                        1e7
                      ).toFixed(4)}{' '}
                      XLM
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      90 Days
                    </span>
                    <span className='font-semibold text-foreground'>
                      {(
                        (calculateTotalInvested() * pool.data.yieldRate) /
                        10000 /
                        4 /
                        1e7
                      ).toFixed(4)}{' '}
                      XLM
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      1 Year
                    </span>
                    <span className='font-semibold text-green-600'>
                      {formatCurrency(calculateTotalEarnings().toString())} XLM
                    </span>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card className='hover:shadow-md transition-shadow'>
              <CardHeader>
                <CardTitle className='flex items-center gap-2'>
                  <PieChart className='size-5' />
                  Pool Distribution
                </CardTitle>
              </CardHeader>
              <CardContent className='space-y-4'>
                <div className='space-y-3'>
                  {pool.data.investorShares.map((share, index) => (
                    <div
                      key={index}
                      className='flex justify-between items-center'
                    >
                      <span className='text-sm text-muted-foreground'>
                        {share.investor.slice(0, 8)}...
                      </span>
                      <span className='font-semibold text-foreground'>
                        {(share.sharePercent / 100).toFixed(2)}%
                      </span>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
      </Tabs>
    </main>
  )
}
