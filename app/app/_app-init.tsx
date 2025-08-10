"use client"

import type React from "react"
import { AppProvider } from "@/context/app-context"

// This wrapper ensures AppProvider is available across pages when imported in layout
export default function AppInit({ children }: { children: React.ReactNode }) {
  return <AppProvider>{children}</AppProvider>
}
