'use client'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { useApp } from '@/context/app-context'
import { useToast } from '@/hooks/use-toast'
import {
  CreditCard,
  Calendar,
  DollarSign,
  Shield,
  AlertTriangle
} from 'lucide-react'
import { useState } from 'react'

interface SubscriptionCardProps {
  subscription: {
    id: string
    policyId: string
    policyName: string
    premium: string // Changed from number to string (i128 format)
    coverage: string // Changed from number to string (i128 format)
    startDate: string
    endDate: string
    status: 'active' | 'expired' | 'cancelled' | 'forfeited' | 'ended' // Added actual subscription statuses
    nextPaymentDate: string
    claims: number
    maxClaims: number
  }
}

export default function SubscriptionCard({
  subscription
}: SubscriptionCardProps) {
  const { sdk } = useApp()
  const { toast } = useToast()
  const [isPaying, setIsPaying] = useState(false)

  const handlePayPremium = async () => {
    if (!sdk) return

    setIsPaying(true)
    try {
      await sdk.payPremium(subscription.id)
      toast({
        title: 'Premium Paid',
        description: 'Your premium payment has been processed successfully.'
      })
    } catch (error) {
      toast({
        title: 'Payment Failed',
        description: 'There was an error processing your premium payment.',
        variant: 'destructive'
      })
    } finally {
      setIsPaying(false)
    }
  }

  const formatCurrency = (amount: string) => {
    // Convert i128 string to number (divide by 1e7 for Stellar amounts)
    const numericAmount = Number(amount) / 1e7
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD'
    }).format(numericAmount)
  }

  const getStatusVariant = (status: string) => {
    switch (status) {
      case 'active':
        return 'default'
      case 'expired':
      case 'ended':
        return 'destructive'
      case 'cancelled':
      case 'forfeited':
        return 'secondary'
      default:
        return 'secondary'
    }
  }

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active':
        return <Shield className='size-4' />
      case 'expired':
      case 'ended':
        return <AlertTriangle className='size-4' />
      case 'cancelled':
      case 'forfeited':
        return <AlertTriangle className='size-4' />
      default:
        return <AlertTriangle className='size-4' />
    }
  }

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    })
  }

  return (
    <Card className='group hover:shadow-md transition-shadow'>
      <CardHeader className='pb-3'>
        <div className='flex items-start justify-between'>
          <div className='space-y-1'>
            <CardTitle className='text-lg font-semibold text-foreground'>
              {subscription.policyName}
            </CardTitle>
            <div className='flex items-center gap-2'>
              <Badge
                variant={getStatusVariant(subscription.status)}
                className='gap-1'
              >
                {getStatusIcon(subscription.status)}
                {subscription.status.charAt(0).toUpperCase() +
                  subscription.status.slice(1)}
              </Badge>
              <span className='text-sm text-muted-foreground'>
                ID: {subscription.id.slice(0, 8)}...
              </span>
            </div>
          </div>
        </div>
      </CardHeader>

      <CardContent className='space-y-4'>
        <div className='grid grid-cols-2 gap-4'>
          <div className='space-y-2'>
            <div className='flex items-center gap-2 text-sm text-muted-foreground'>
              <DollarSign className='size-4' />
              <span>Premium</span>
            </div>
            <p className='text-lg font-semibold text-foreground'>
              {formatCurrency(subscription.premium)}
            </p>
          </div>

          <div className='space-y-2'>
            <div className='flex items-center gap-2 text-sm text-muted-foreground'>
              <Shield className='size-4' />
              <span>Coverage</span>
            </div>
            <p className='text-lg font-semibold text-foreground'>
              {formatCurrency(subscription.coverage)}
            </p>
          </div>
        </div>

        <div className='grid grid-cols-2 gap-4'>
          <div className='space-y-2'>
            <div className='flex items-center gap-2 text-sm text-muted-foreground'>
              <Calendar className='size-4' />
              <span>Start Date</span>
            </div>
            <p className='text-sm font-medium text-foreground'>
              {formatDate(subscription.startDate)}
            </p>
          </div>

          <div className='space-y-2'>
            <div className='flex items-center gap-2 text-sm text-muted-foreground'>
              <Calendar className='size-4' />
              <span>End Date</span>
            </div>
            <p className='text-sm font-medium text-foreground'>
              {formatDate(subscription.endDate)}
            </p>
          </div>
        </div>

        <div className='space-y-3'>
          <div className='flex items-center justify-between text-sm'>
            <span className='text-muted-foreground'>Claims Used</span>
            <span className='font-medium text-foreground'>
              {subscription.claims} / {subscription.maxClaims}
            </span>
          </div>

          <div className='w-full bg-muted rounded-full h-2'>
            <div
              className='bg-primary h-2 rounded-full transition-all duration-300'
              style={{
                width: `${
                  (subscription.claims / subscription.maxClaims) * 100
                }%`
              }}
            />
          </div>
        </div>

        {subscription.status === 'active' && (
          <div className='space-y-3 pt-2'>
            <div className='flex items-center justify-between text-sm'>
              <span className='text-muted-foreground'>Next Payment</span>
              <span className='font-medium text-foreground'>
                {formatDate(subscription.nextPaymentDate)}
              </span>
            </div>

            <Button
              onClick={handlePayPremium}
              disabled={isPaying}
              className='w-full'
              aria-label={`Pay premium for ${subscription.policyName}`}
            >
              <CreditCard className='size-4 mr-2' />
              {isPaying ? 'Processing...' : 'Pay Premium'}
            </Button>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
