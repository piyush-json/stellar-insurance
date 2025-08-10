"use client"

import React from "react"
import WalletModal from "@/components/wallet-modal"
import { useApp, useStrings } from "@/context/app-context"
import { useParams } from "next/navigation"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import TxStatusToast from "@/components/tx-status-toast"
import { useToast } from "@/hooks/use-toast"

export default function Page() {
  const { policyId } = useParams<{ policyId: string }>()
  const { sdk } = useApp()
  const S = useStrings()
  const { toast } = useToast()
  const [step, setStep] = React.useState<1 | 2 | 3>(1)
  const [txStatus, setTxStatus] = React.useState<"idle" | "pending" | "success" | "error">("idle")
  const [txId, setTxId] = React.useState<string | undefined>(undefined)
  const pols = React.useRef<{ title: string; premium: string } | null>(null)

  React.useEffect(() => {
    ;(async () => {
      const p = (await sdk.getPolicies()).find((x) => x.id === policyId)
      if (p) pols.current = { title: p.title, premium: p.params.premiumAmount }
    })()
  }, [policyId, sdk])

  const pay = async () => {
    setTxStatus("pending")
    const subs = await sdk.getSubscriptions()
    const sub = subs.find((s) => s.policyId === policyId)
    if (!sub) {
      toast({ title: "No subscription found", description: "Subscribe first from Policies page" })
      setTxStatus("error")
      return
    }
    const res = await sdk.payPremium(sub.id)
    setTxId(res.txId)
    setTxStatus(res.ok ? "success" : "error")
    if (res.ok) setStep(3)
  }

  return (
    <main className="container max-w-2xl mx-auto p-4 space-y-4">
      <TxStatusToast status={txStatus} txId={txId} />
      <Card>
        <CardHeader>
          <CardTitle>{S.subscribe_to_policy}</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4">
          {step === 1 && (
            <div className="space-y-3">
              <div className="font-medium">{S.step_review}</div>
              <div className="text-sm">
                Policy: <strong>{pols.current?.title || policyId}</strong>
              </div>
              <WalletModal />
              <Button className="bg-emerald-600 hover:bg-emerald-700" onClick={() => setStep(2)}>
                Continue
              </Button>
            </div>
          )}
          {step === 2 && (
            <div className="space-y-3">
              <div className="font-medium">{S.step_pay}</div>
              <div className="text-sm">First premium: {Number(pols.current?.premium || 0) / 1e7} XLM</div>
              <Button onClick={() => void pay()}>{S.pay_premium}</Button>
            </div>
          )}
          {step === 3 && (
            <div className="space-y-3">
              <div className="font-medium">{S.step_done}</div>
              <div className="text-sm">
                {S.start_date}: {new Date().toDateString()}
              </div>
            </div>
          )}
        </CardContent>
      </Card>
    </main>
  )
}
