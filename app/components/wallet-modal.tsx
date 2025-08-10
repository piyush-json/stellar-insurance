'use client'

import React from 'react'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { useApp } from '@/context/app-context'

export interface WalletModalProps {
  trigger?: React.ReactNode
}

export default function WalletModal({ trigger }: WalletModalProps) {
  const { address, setAddress } = useApp()
  const [open, setOpen] = React.useState(false)
  const [custom, setCustom] = React.useState('')
  const accounts = ['GABC...WXYZ', 'GCYK...KLMN', 'GHIJ...STUV']

  const handleConnect = (selectedAddress: string) => {
    setAddress(selectedAddress)
    setOpen(false)
  }

  const handleCustomConnect = () => {
    if (custom.trim()) {
      handleConnect(custom.trim())
      setCustom('')
    }
  }

  const handleDisconnect = () => {
    setAddress(null)
    setOpen(false)
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        {trigger || (
          <Button variant='outline' aria-label='Connect wallet'>
            {address
              ? `${address.slice(0, 6)}...${address.slice(-4)}`
              : 'Connect Wallet'}
          </Button>
        )}
      </DialogTrigger>
      <DialogContent className='sm:max-w-md'>
        <DialogHeader>
          <DialogTitle>Select Wallet</DialogTitle>
        </DialogHeader>
        <div className='grid gap-4'>
          <div className='grid gap-2'>
            {accounts.map((acc) => (
              <Button
                key={acc}
                variant={address === acc ? 'default' : 'outline'}
                onClick={() => handleConnect(acc)}
                className='justify-start'
                aria-label={`Connect to ${acc}`}
              >
                {acc}
              </Button>
            ))}
          </div>

          <div className='space-y-2'>
            <Label htmlFor='custom-address'>Custom Address</Label>
            <div className='flex gap-2'>
              <Input
                id='custom-address'
                placeholder='Enter custom address'
                value={custom}
                onChange={(e) => setCustom(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleCustomConnect()}
              />
              <Button onClick={handleCustomConnect} disabled={!custom.trim()}>
                Connect
              </Button>
            </div>
          </div>

          {address && (
            <Button
              variant='ghost'
              onClick={handleDisconnect}
              className='w-full'
            >
              Disconnect Wallet
            </Button>
          )}
        </div>
      </DialogContent>
    </Dialog>
  )
}
