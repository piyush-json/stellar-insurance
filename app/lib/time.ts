const DEFAULT_TZ = "Asia/Kolkata"

export function formatDateTime(dateIso: string | number, tz: string = DEFAULT_TZ) {
  try {
    const date = new Date(typeof dateIso === "number" ? dateIso : Date.parse(dateIso))
    const fmt = new Intl.DateTimeFormat("en-IN", {
      timeZone: tz,
      year: "numeric",
      month: "short",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    } as Intl.DateTimeFormatOptions)
    return fmt.format(date)
  } catch {
    return String(dateIso)
  }
}

export function formatRelative(targetIso: string | number) {
  const now = Date.now()
  const target = typeof targetIso === "number" ? targetIso : Date.parse(targetIso)
  let diff = target - now // future positive
  const sign = diff >= 0 ? 1 : -1
  diff = Math.abs(diff)
  const sec = Math.floor(diff / 1000)
  const days = Math.floor(sec / 86400)
  const hours = Math.floor((sec % 86400) / 3600)
  const mins = Math.floor((sec % 3600) / 60)
  const parts: string[] = []
  if (days) parts.push(`${days}d`)
  if (hours) parts.push(`${hours}h`)
  if (mins && parts.length < 2) parts.push(`${mins}m`)
  const str = parts.length ? parts.join(" ") : "0m"
  return sign > 0 ? `in ${str}` : `${str} ago`
}

export function addDays(iso: string, days: number) {
  const t = Date.parse(iso)
  return new Date(t + days * 86400000).toISOString()
}
