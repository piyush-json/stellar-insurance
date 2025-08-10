"use client"

import type React from "react"
import LayoutClient from "../layout-client"

export default function RoutesLayout({ children }: { children: React.ReactNode }) {
  return <LayoutClient>{children}</LayoutClient>
}
