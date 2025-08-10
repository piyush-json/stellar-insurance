"use client"

import React from "react"

export function usePolling<T>(fetcher: () => Promise<T>, deps: any[] = [], intervalMs = 15000) {
  const [data, setData] = React.useState<T | null>(null)
  const [loading, setLoading] = React.useState(false)
  const [error, setError] = React.useState<string | null>(null)

  const refresh = React.useCallback(async () => {
    try {
      setLoading(true)
      setError(null)
      const res = await fetcher()
      setData(res)
    } catch (e: any) {
      setError(e?.message || "Error")
    } finally {
      setLoading(false)
    }
  }, [fetcher])

  React.useEffect(() => {
    void refresh()
    const id = setInterval(refresh, intervalMs)
    return () => clearInterval(id)
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, deps)

  return { data, loading, error, refresh, setData }
}
