"use client"

import type React from "react"
import Navigation from "./navigation"

export default function Template({ children }: { children: React.ReactNode }) {
  return (
    <>
      <Navigation />
      {children}
    </>
  )
}
