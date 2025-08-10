'use client'
import { Badge } from '@/components/ui/badge'

export interface CreditScorePillProps {
  score: number
  trend?: number[] // last 10 values for sparkline
}

export default function CreditScorePill({
  score,
  trend = [600, 610, 620, 630, 640, 650, 660, 670, 680, score]
}: CreditScorePillProps) {
  const min = Math.min(...trend)
  const max = Math.max(...trend)
  const range = Math.max(1, max - min)
  const points = trend
    .map((v, i) => {
      const x = (i / (trend.length - 1)) * 100
      const y = 20 - ((v - min) / range) * 20
      return `${x},${y}`
    })
    .join(' ')

  // Determine score color based on ranges
  const getScoreColor = (score: number) => {
    if (score >= 750) return 'bg-green-500'
    if (score >= 650) return 'bg-yellow-500'
    return 'bg-red-500'
  }

  const getScoreVariant = (score: number) => {
    if (score >= 750) return 'default'
    if (score >= 650) return 'secondary'
    return 'destructive'
  }

  return (
    <div className='flex items-center gap-3'>
      <Badge variant={getScoreVariant(score)} className='font-medium'>
        Score: {score}
      </Badge>
      <div className='relative'>
        <svg
          viewBox='0 0 100 20'
          width='100'
          height='20'
          aria-hidden='true'
          className='text-muted-foreground'
        >
          <polyline
            fill='none'
            stroke='currentColor'
            className={getScoreColor(score)}
            strokeWidth='2'
            points={points}
          />
        </svg>
      </div>
    </div>
  )
}
