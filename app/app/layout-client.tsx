'use client'

import type React from 'react'
import AppInit from './_app-init'
import Navigation from './navigation'

export default function LayoutClient({
  children
}: {
  children: React.ReactNode
}) {
  return (
    <AppInit>
      <Navigation />
      {children}
    </AppInit>
  )
}
