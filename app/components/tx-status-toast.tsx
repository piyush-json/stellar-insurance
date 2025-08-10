"use client"

import React from "react"
import { useToast } from "@/hooks/use-toast"
import { useStrings } from "@/context/app-context"

export interface TxStatusToastProps {
  status: "idle" | "pending" | "success" | "error"
  txId?: string
  error?: string
}

export default function TxStatusToast({ status, txId, error }: TxStatusToastProps) {
  const { toast } = useToast()
  const S = useStrings()
  React.useEffect(() => {
    if (status === "success") {
      toast({ title: S.success, description: txId })
    } else if (status === "error") {
      toast({ title: S.failed, description: error })
    }
  }, [status, txId, error, toast, S.success, S.failed])
  return null
}

TxStatusToast.defaultProps = { status: "idle" }
