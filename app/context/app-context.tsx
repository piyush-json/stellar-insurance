"use client"

import React from "react"
import { STRINGS, type LocaleKey } from "@/lib/strings"
import { sdk as defaultSdk, getCurrentAddress, setCurrentAddress, onChange, type Sdk } from "@/lib/sdk-mock"

type AppContextState = {
  sdk: Sdk
  address: string | null
  locale: LocaleKey
  timezone: string
  setTimezone: (tz: string) => void
  setAddress: (addr: string | null) => void
}
const AppCtx = React.createContext<AppContextState | null>(null)

export function AppProvider({ children }: { children: React.ReactNode }) {
  const [timezone, setTimezone] = React.useState("Asia/Kolkata")
  const [address, setAddr] = React.useState<string | null>(getCurrentAddress())
  React.useEffect(() => {
    const off = onChange("wallet", () => setAddr(getCurrentAddress()))
    return off
  }, [])

  const setAddress = (addr: string | null) => {
    setCurrentAddress(addr)
    setAddr(addr)
  }

  const value: AppContextState = {
    sdk: defaultSdk,
    address,
    locale: "en-IN",
    timezone,
    setTimezone,
    setAddress,
  }
  return <AppCtx.Provider value={value}>{children}</AppCtx.Provider>
}

export function useApp() {
  const ctx = React.useContext(AppCtx)
  if (!ctx) throw new Error("useApp must be used within AppProvider")
  return ctx
}

export function useStrings() {
  const { locale } = useApp()
  return STRINGS[locale]
}
