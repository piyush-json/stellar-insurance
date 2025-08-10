'use client'

import { useApp } from '@/context/app-context'
import { useParams, useRouter } from 'next/navigation'
import { useState } from 'react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Skeleton } from '@/components/ui/skeleton'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import { Progress } from '@/components/ui/progress'
import { Separator } from '@/components/ui/separator'
import { usePolling } from '@/hooks/use-polling'
import { useToast } from '@/hooks/use-toast'
import {
  Shield,
  Calendar,
  DollarSign,
  AlertTriangle,
  CheckCircle,
  Clock,
  Users,
  TrendingUp,
  FileText,
  ArrowLeft,
  RefreshCw,
  Star,
  Zap,
  Target,
  BarChart3
} from 'lucide-react'
import { formatDateTime } from '@/lib/time'

export default function Page() {
  const { id } = useParams<{ id: string }>()
  const { sdk } = useApp()
  const router = useRouter()
  const { toast } = useToast()
  const [isSubscribing, setIsSubscribing] = useState(false)

  const policies = usePolling(() => sdk.getPolicies(), [], 30000)
  const policy = (policies.data || []).find((p) => p.id === id)

  const handleSubscribe = async () => {
    if (!sdk || !policy) return

    setIsSubscribing(true)
    try {
      const res = await sdk.subscribeToPolicy(id)
      if (res.ok) {
        toast({
          title: 'Subscription Successful',
          description: `Successfully subscribed to ${policy.title}`
        })
        router.push(`/subscribe/${id}`)
      } else {
        toast({
          title: 'Subscription Failed',
          description: res.error || 'Failed to subscribe to policy',
          variant: 'destructive'
        })
      }
    } catch (error) {
      toast({
        title: 'Subscription Failed',
        description: 'An unexpected error occurred',
        variant: 'destructive'
      })
    } finally {
      setIsSubscribing(false)
    }
  }

  const formatCurrency = (amount: string) => {
    return (Number(amount) / 1e7).toFixed(2)
  }

  const getStatusVariant = (status: string) => {
    switch (status) {
      case 'active':
        return 'default'
      case 'archived':
        return 'secondary'
      case 'pending':
        return 'outline'
      default:
        return 'secondary'
    }
  }

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active':
        return <CheckCircle className='size-4' />
      case 'archived':
        return <AlertTriangle className='size-4' />
      case 'pending':
        return <Clock className='size-4' />
      default:
        return <AlertTriangle className='size-4' />
    }
  }

  const calculateRiskLevel = (policy: any) => {
    const maxClaim = Number(policy.params.maxClaimAmount)
    const premium = Number(policy.params.premiumAmount)
    const ratio = maxClaim / premium

    if (ratio > 50)
      return {
        level: 'High',
        color: 'text-red-600',
        bg: 'bg-red-100 dark:bg-red-950/20'
      }
    if (ratio > 25)
      return {
        level: 'Medium',
        color: 'text-yellow-600',
        bg: 'bg-yellow-100 dark:bg-yellow-950/20'
      }
    return {
      level: 'Low',
      color: 'text-green-600',
      bg: 'bg-green-100 dark:bg-green-950/20'
    }
  }

  if (policies.loading) {
    return (
      <main className='container max-w-6xl mx-auto p-4 space-y-6'>
        <div className='space-y-4'>
          <Skeleton className='h-8 w-64' />
          <Skeleton className='h-4 w-96' />
        </div>
        <div className='grid grid-cols-1 md:grid-cols-3 gap-6'>
          {[1, 2, 3].map((i) => (
            <Skeleton key={i} className='h-32 w-full' />
          ))}
        </div>
        <Skeleton className='h-96 w-full' />
      </main>
    )
  }

  if (!policy) {
    return (
      <main className='container max-w-6xl mx-auto p-4 space-y-6'>
        <div className='text-center py-12'>
          <AlertTriangle className='size-16 mx-auto mb-4 text-muted-foreground' />
          <h1 className='text-2xl font-bold text-foreground mb-2'>
            Policy Not Found
          </h1>
          <p className='text-muted-foreground mb-6'>
            The policy you're looking for doesn't exist or has been removed.
          </p>
          <Button onClick={() => router.push('/policies')} variant='outline'>
            <ArrowLeft className='size-4 mr-2' />
            Back to Policies
          </Button>
        </div>
      </main>
    )
  }

  const riskLevel = calculateRiskLevel(policy)

  return (
    <main className='container max-w-6xl mx-auto p-4 space-y-6'>
      {/* Header */}
      <div className='space-y-4'>
        <Button
          variant='ghost'
          onClick={() => router.push('/policies')}
          className='mb-4'
        >
          <ArrowLeft className='size-4 mr-2' />
          Back to Policies
        </Button>

        <div className='space-y-2'>
          <div className='flex items-center gap-3'>
            <h1 className='text-3xl font-bold text-foreground'>
              {policy.title}
            </h1>
            <Badge variant={getStatusVariant(policy.status)} className='gap-1'>
              {getStatusIcon(policy.status)}
              {policy.status.charAt(0).toUpperCase() + policy.status.slice(1)}
            </Badge>
          </div>
          <p className='text-lg text-muted-foreground max-w-3xl'>
            {policy.description}
          </p>
        </div>
      </div>

      {/* Key Metrics Cards */}
      <div className='grid grid-cols-1 md:grid-cols-4 gap-6'>
        <Card className='hover:shadow-md transition-shadow'>
          <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
            <CardTitle className='text-sm font-medium text-muted-foreground'>
              Premium Amount
            </CardTitle>
            <DollarSign className='size-4 text-muted-foreground' />
          </CardHeader>
          <CardContent>
            <div className='text-2xl font-bold text-foreground'>
              {formatCurrency(policy.params.premiumAmount)}{' '}
              {policy.params.premiumCurrency}
            </div>
            <p className='text-xs text-muted-foreground'>Per coverage period</p>
          </CardContent>
        </Card>

        <Card className='hover:shadow-md transition-shadow'>
          <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
            <CardTitle className='text-sm font-medium text-muted-foreground'>
              Max Claim
            </CardTitle>
            <Shield className='size-4 text-muted-foreground' />
          </CardHeader>
          <CardContent>
            <div className='text-2xl font-bold text-foreground'>
              {formatCurrency(policy.params.maxClaimAmount)}{' '}
              {policy.params.premiumCurrency}
            </div>
            <p className='text-xs text-muted-foreground'>
              Maximum payout per claim
            </p>
          </CardContent>
        </Card>

        <Card className='hover:shadow-md transition-shadow'>
          <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
            <CardTitle className='text-sm font-medium text-muted-foreground'>
              Interest Rate
            </CardTitle>
            <TrendingUp className='size-4 text-muted-foreground' />
          </CardHeader>
          <CardContent>
            <div className='text-2xl font-bold text-foreground'>
              {(policy.params.interestRate / 100).toFixed(2)}%
            </div>
            <p className='text-xs text-muted-foreground'>
              Annual percentage rate
            </p>
          </CardContent>
        </Card>

        <Card className='hover:shadow-md transition-shadow'>
          <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
            <CardTitle className='text-sm font-medium text-muted-foreground'>
              Risk Level
            </CardTitle>
            <Target className='size-4 text-muted-foreground' />
          </CardHeader>
          <CardContent>
            <div className={`text-2xl font-bold ${riskLevel.color}`}>
              {riskLevel.level}
            </div>
            <p className='text-xs text-muted-foreground'>
              Based on claim/premium ratio
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Policy Details Tabs */}
      <Tabs defaultValue='overview' className='space-y-4'>
        <TabsList className='grid w-full grid-cols-4'>
          <TabsTrigger value='overview'>Overview</TabsTrigger>
          <TabsTrigger value='terms'>Terms & Conditions</TabsTrigger>
          <TabsTrigger value='coverage'>Coverage Details</TabsTrigger>
          <TabsTrigger value='analytics'>Analytics</TabsTrigger>
        </TabsList>

        <TabsContent value='overview' className='space-y-6'>
          <div className='grid grid-cols-1 lg:grid-cols-2 gap-6'>
            <Card className='hover:shadow-md transition-shadow'>
              <CardHeader>
                <CardTitle className='flex items-center gap-2'>
                  <FileText className='size-5' />
                  Policy Information
                </CardTitle>
              </CardHeader>
              <CardContent className='space-y-4'>
                <div className='space-y-3'>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Policy ID
                    </span>
                    <span className='font-mono text-sm text-foreground'>
                      {policy.id}
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Creator
                    </span>
                    <span className='text-sm text-foreground'>
                      {policy.creator.slice(0, 8)}...
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Created
                    </span>
                    <span className='text-sm text-foreground'>
                      {formatDateTime(policy.createdAt, 'UTC')}
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Status
                    </span>
                    <Badge variant={getStatusVariant(policy.status)}>
                      {policy.status.charAt(0).toUpperCase() +
                        policy.status.slice(1)}
                    </Badge>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card className='hover:shadow-md transition-shadow'>
              <CardHeader>
                <CardTitle className='flex items-center gap-2'>
                  <Shield className='size-5' />
                  Coverage Summary
                </CardTitle>
              </CardHeader>
              <CardContent className='space-y-4'>
                <div className='space-y-3'>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Coverage Ratio
                    </span>
                    <span className='font-semibold text-foreground'>
                      {(
                        Number(policy.params.maxClaimAmount) /
                        Number(policy.params.premiumAmount)
                      ).toFixed(1)}
                      x
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Claim Cooldown
                    </span>
                    <span className='font-semibold text-foreground'>
                      {policy.params.claimCooldownDays} days
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Investor Lock-in
                    </span>
                    <span className='font-semibold text-foreground'>
                      {policy.params.investorLockInDays} days
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      DAO Approval
                    </span>
                    <Badge
                      variant={
                        policy.params.requiresDaoApproval
                          ? 'default'
                          : 'secondary'
                      }
                    >
                      {policy.params.requiresDaoApproval
                        ? 'Required'
                        : 'Not Required'}
                    </Badge>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Risk Assessment */}
          <Card className='hover:shadow-md transition-shadow'>
            <CardHeader>
              <CardTitle className='flex items-center gap-2'>
                <BarChart3 className='size-5' />
                Risk Assessment
              </CardTitle>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='space-y-3'>
                <div className='flex justify-between items-center'>
                  <span className='text-sm text-muted-foreground'>
                    Risk Level
                  </span>
                  <Badge className={`${riskLevel.bg} ${riskLevel.color}`}>
                    {riskLevel.level} Risk
                  </Badge>
                </div>
                <div className='space-y-2'>
                  <div className='flex justify-between items-center text-sm'>
                    <span className='text-muted-foreground'>
                      Claim to Premium Ratio
                    </span>
                    <span className='font-medium text-foreground'>
                      {(
                        Number(policy.params.maxClaimAmount) /
                        Number(policy.params.premiumAmount)
                      ).toFixed(1)}
                      :1
                    </span>
                  </div>
                  <Progress
                    value={Math.min(
                      (Number(policy.params.maxClaimAmount) /
                        Number(policy.params.premiumAmount)) *
                        10,
                      100
                    )}
                    className='w-full'
                  />
                </div>
                <p className='text-xs text-muted-foreground'>
                  Higher ratios indicate higher risk policies with larger
                  potential payouts relative to premiums.
                </p>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value='terms' className='space-y-6'>
          <Card className='hover:shadow-md transition-shadow'>
            <CardHeader>
              <CardTitle className='flex items-center gap-2'>
                <FileText className='size-5' />
                Terms & Conditions
              </CardTitle>
            </CardHeader>
            <CardContent className='space-y-6'>
              <div className='space-y-4'>
                <div>
                  <h4 className='font-semibold text-foreground mb-2'>
                    Eligibility Requirements
                  </h4>
                  <ul className='space-y-2 text-sm text-muted-foreground'>
                    <li className='flex items-start gap-2'>
                      <CheckCircle className='size-4 text-green-600 mt-0.5 flex-shrink-0' />
                      <span>
                        Must be a registered user with valid credit score
                      </span>
                    </li>
                    <li className='flex items-start gap-2'>
                      <CheckCircle className='size-4 text-green-600 mt-0.5 flex-shrink-0' />
                      <span>Minimum credit score requirements may apply</span>
                    </li>
                    <li className='flex items-start gap-2'>
                      <CheckCircle className='size-4 text-green-600 mt-0.5 flex-shrink-0' />
                      <span>Must provide valid documentation for claims</span>
                    </li>
                  </ul>
                </div>

                <Separator />

                <div>
                  <h4 className='font-semibold text-foreground mb-2'>
                    Coverage Period
                  </h4>
                  <div className='grid grid-cols-1 md:grid-cols-2 gap-4 text-sm'>
                    <div>
                      <span className='text-muted-foreground'>
                        Premium Payment:
                      </span>
                      <span className='ml-2 font-medium text-foreground'>
                        Monthly
                      </span>
                    </div>
                    <div>
                      <span className='text-muted-foreground'>
                        Coverage Duration:
                      </span>
                      <span className='ml-2 font-medium text-foreground'>
                        12 months
                      </span>
                    </div>
                    <div>
                      <span className='text-muted-foreground'>Renewal:</span>
                      <span className='ml-2 font-medium text-foreground'>
                        Automatic
                      </span>
                    </div>
                    <div>
                      <span className='text-muted-foreground'>
                        Grace Period:
                      </span>
                      <span className='ml-2 font-medium text-foreground'>
                        3 days
                      </span>
                    </div>
                  </div>
                </div>

                <Separator />

                <div>
                  <h4 className='font-semibold text-foreground mb-2'>
                    Claims Process
                  </h4>
                  <div className='space-y-3 text-sm text-muted-foreground'>
                    <div className='flex items-start gap-2'>
                      <div className='w-6 h-6 rounded-full bg-primary text-primary-foreground text-xs flex items-center justify-center font-medium mt-0.5'>
                        1
                      </div>
                      <span>
                        Submit claim with supporting documentation within 30
                        days of incident
                      </span>
                    </div>
                    <div className='flex items-start gap-2'>
                      <div className='w-6 h-6 rounded-full bg-primary text-primary-foreground text-xs flex items-center justify-center font-medium mt-0.5'>
                        2
                      </div>
                      <span>
                        Claims are reviewed by DAO members for approval
                      </span>
                    </div>
                    <div className='flex items-start gap-2'>
                      <div className='w-6 h-6 rounded-full bg-primary text-primary-foreground text-xs flex items-center justify-center font-medium mt-0.5'>
                        3
                      </div>
                      <span>
                        Approved claims are processed within 5 business days
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value='coverage' className='space-y-6'>
          <div className='grid grid-cols-1 lg:grid-cols-2 gap-6'>
            <Card className='hover:shadow-md transition-shadow'>
              <CardHeader>
                <CardTitle className='flex items-center gap-2'>
                  <Shield className='size-5' />
                  What's Covered
                </CardTitle>
              </CardHeader>
              <CardContent className='space-y-4'>
                <div className='space-y-3'>
                  <div className='flex items-start gap-2'>
                    <CheckCircle className='size-4 text-green-600 mt-0.5 flex-shrink-0' />
                    <span className='text-sm text-foreground'>
                      Direct losses from covered events
                    </span>
                  </div>
                  <div className='flex items-start gap-2'>
                    <CheckCircle className='size-4 text-green-600 mt-0.5 flex-shrink-0' />
                    <span className='text-sm text-foreground'>
                      Documented damage assessments
                    </span>
                  </div>
                  <div className='flex items-start gap-2'>
                    <CheckCircle className='size-4 text-green-600 mt-0.5 flex-shrink-0' />
                    <span className='text-sm text-foreground'>
                      Emergency response costs
                    </span>
                  </div>
                  <div className='flex items-start gap-2'>
                    <CheckCircle className='size-4 text-green-600 mt-0.5 flex-shrink-0' />
                    <span className='text-sm text-foreground'>
                      Temporary replacement costs
                    </span>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card className='hover:shadow-md transition-shadow'>
              <CardHeader>
                <CardTitle className='flex items-center gap-2'>
                  <AlertTriangle className='size-5' />
                  What's Not Covered
                </CardTitle>
              </CardHeader>
              <CardContent className='space-y-4'>
                <div className='space-y-3'>
                  <div className='flex items-start gap-2'>
                    <AlertTriangle className='size-4 text-red-600 mt-0.5 flex-shrink-0' />
                    <span className='text-sm text-foreground'>
                      Pre-existing conditions
                    </span>
                  </div>
                  <div className='flex items-start gap-2'>
                    <AlertTriangle className='size-4 text-red-600 mt-0.5 flex-shrink-0' />
                    <span className='text-sm text-foreground'>
                      Intentional damage or fraud
                    </span>
                  </div>
                  <div className='flex items-start gap-2'>
                    <AlertTriangle className='size-4 text-red-600 mt-0.5 flex-shrink-0' />
                    <span className='text-sm text-foreground'>
                      Maintenance and wear costs
                    </span>
                  </div>
                  <div className='flex items-start gap-2'>
                    <AlertTriangle className='size-4 text-red-600 mt-0.5 flex-shrink-0' />
                    <span className='text-sm text-foreground'>
                      Acts of war or terrorism
                    </span>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Coverage Limits */}
          <Card className='hover:shadow-md transition-shadow'>
            <CardHeader>
              <CardTitle className='flex items-center gap-2'>
                <Target className='size-5' />
                Coverage Limits & Deductibles
              </CardTitle>
            </CardHeader>
            <CardContent className='space-y-4'>
              <div className='grid grid-cols-1 md:grid-cols-3 gap-6'>
                <div className='space-y-2'>
                  <h4 className='font-medium text-foreground'>
                    Maximum Coverage
                  </h4>
                  <div className='text-2xl font-bold text-foreground'>
                    {formatCurrency(policy.params.maxClaimAmount)}{' '}
                    {policy.params.premiumCurrency}
                  </div>
                  <p className='text-xs text-muted-foreground'>
                    Per claim maximum
                  </p>
                </div>
                <div className='space-y-2'>
                  <h4 className='font-medium text-foreground'>Deductible</h4>
                  <div className='text-2xl font-bold text-foreground'>
                    {formatCurrency(
                      (Number(policy.params.premiumAmount) * 0.1).toString()
                    )}{' '}
                    {policy.params.premiumCurrency}
                  </div>
                  <p className='text-xs text-muted-foreground'>
                    10% of premium amount
                  </p>
                </div>
                <div className='space-y-2'>
                  <h4 className='font-medium text-foreground'>Annual Limit</h4>
                  <div className='text-2xl font-bold text-foreground'>
                    {formatCurrency(
                      (Number(policy.params.maxClaimAmount) * 2).toString()
                    )}{' '}
                    {policy.params.premiumCurrency}
                  </div>
                  <p className='text-xs text-muted-foreground'>
                    Maximum per year
                  </p>
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value='analytics' className='space-y-6'>
          <div className='grid grid-cols-1 lg:grid-cols-2 gap-6'>
            <Card className='hover:shadow-md transition-shadow'>
              <CardHeader>
                <CardTitle className='flex items-center gap-2'>
                  <TrendingUp className='size-5' />
                  Policy Performance
                </CardTitle>
              </CardHeader>
              <CardContent className='space-y-4'>
                <div className='space-y-3'>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Success Rate
                    </span>
                    <span className='font-semibold text-green-600'>94.2%</span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Average Claim Time
                    </span>
                    <span className='font-semibold text-foreground'>
                      3.2 days
                    </span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Total Claims
                    </span>
                    <span className='font-semibold text-foreground'>127</span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Total Payouts
                    </span>
                    <span className='font-semibold text-foreground'>
                      {formatCurrency(
                        (Number(policy.params.maxClaimAmount) * 0.8).toString()
                      )}{' '}
                      {policy.params.premiumCurrency}
                    </span>
                  </div>
                </div>
              </CardContent>
            </Card>

            <Card className='hover:shadow-md transition-shadow'>
              <CardHeader>
                <CardTitle className='flex items-center gap-2'>
                  <Users className='size-5' />
                  Subscriber Statistics
                </CardTitle>
              </CardHeader>
              <CardContent className='space-y-4'>
                <div className='space-y-3'>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Active Subscribers
                    </span>
                    <span className='font-semibold text-foreground'>1,247</span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      New This Month
                    </span>
                    <span className='font-semibold text-green-600'>+89</span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Retention Rate
                    </span>
                    <span className='font-semibold text-foreground'>87.3%</span>
                  </div>
                  <div className='flex justify-between items-center'>
                    <span className='text-sm text-muted-foreground'>
                      Avg. Credit Score
                    </span>
                    <span className='font-semibold text-foreground'>712</span>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
      </Tabs>

      {/* Action Section */}
      {policy.status === 'active' && (
        <Card className='hover:shadow-md transition-shadow border-primary/20 bg-primary/5'>
          <CardContent className='pt-6'>
            <div className='text-center space-y-4'>
              <div className='space-y-2'>
                <h3 className='text-xl font-semibold text-foreground'>
                  Ready to Get Protected?
                </h3>
                <p className='text-muted-foreground'>
                  Subscribe to this policy and start your coverage today
                </p>
              </div>

              <div className='flex flex-col sm:flex-row gap-3 justify-center'>
                <Button
                  onClick={handleSubscribe}
                  disabled={isSubscribing}
                  size='lg'
                  className='bg-primary hover:bg-primary/90'
                >
                  {isSubscribing ? (
                    <>
                      <RefreshCw className='size-4 mr-2 animate-spin' />
                      Processing...
                    </>
                  ) : (
                    <>
                      <Shield className='size-4 mr-2' />
                      Subscribe Now
                    </>
                  )}
                </Button>

                <Button variant='outline' size='lg'>
                  <FileText className='size-4 mr-2' />
                  Download Policy
                </Button>
              </div>

              <div className='flex items-center justify-center gap-2 text-sm text-muted-foreground'>
                <Star className='size-4 text-yellow-500' />
                <span>4.8/5 rating from 1,247 subscribers</span>
              </div>
            </div>
          </CardContent>
        </Card>
      )}
    </main>
  )
}
