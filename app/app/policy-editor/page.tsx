"use client"

import React from "react"
import { useApp, useStrings } from "@/context/app-context"
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { Switch } from "@/components/ui/switch"
import { Button } from "@/components/ui/button"
import { useToast } from "@/hooks/use-toast"

export default function Page() {
  const { sdk, address } = useApp()
  const S = useStrings()
  const { toast } = useToast()
  const [form, setForm] = React.useState({
    title: "",
    description: "",
    maxClaimAmount: "0",
    interestRate: 500,
    premiumAmount: "0",
    premiumCurrency: "XLM",
    claimCooldownDays: 14,
    investorLockInDays: 30,
    requiresDaoApproval: true,
    creditSlashOnReject: 10,
  })
  const [isDaoMember, setIsDaoMember] = React.useState(false)
  React.useEffect(() => {
    ;(async () => {
      if (address) {
        const u = await sdk.getUser(address)
        setIsDaoMember(u.isDaoMember)
      } else {
        setIsDaoMember(false)
      }
    })()
  }, [address, sdk])

  const submit = async () => {
    const id = await sdk.createPolicy({
      title: form.title,
      description: form.description,
      params: {
        maxClaimAmount: form.maxClaimAmount,
        interestRate: form.interestRate,
        premiumAmount: form.premiumAmount,
        premiumCurrency: form.premiumCurrency,
        claimCooldownDays: form.claimCooldownDays,
        investorLockInDays: form.investorLockInDays,
        requiresDaoApproval: form.requiresDaoApproval,
        creditSlashOnReject: form.creditSlashOnReject,
      },
    } as any)
    toast({ title: "Policy proposed", description: id })
  }

  return (
    <main className="container max-w-3xl mx-auto p-4">
      <Card>
        <CardHeader>
          <CardTitle>{S.policy_editor}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          {!isDaoMember && <div className="text-sm text-amber-600">{S.dao_only_action}</div>}
          <div className="grid gap-2">
            <Label htmlFor="t">{S.policy_title}</Label>
            <Input id="t" value={form.title} onChange={(e) => setForm({ ...form, title: e.target.value })} />
          </div>
          <div className="grid gap-2">
            <Label htmlFor="d">{S.description}</Label>
            <Textarea
              id="d"
              value={form.description}
              onChange={(e) => setForm({ ...form, description: e.target.value })}
            />
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div className="grid gap-2">
              <Label htmlFor="prem">{S.premium_amount}</Label>
              <Input
                id="prem"
                type="number"
                value={form.premiumAmount}
                onChange={(e) => setForm({ ...form, premiumAmount: e.target.value })}
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="cur">{S.currency}</Label>
              <Input
                id="cur"
                value={form.premiumCurrency}
                onChange={(e) => setForm({ ...form, premiumCurrency: e.target.value })}
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="max">{S.max_claim}</Label>
              <Input
                id="max"
                type="number"
                value={form.maxClaimAmount}
                onChange={(e) => setForm({ ...form, maxClaimAmount: e.target.value })}
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="ir">{S.interest_rate}</Label>
              <Input
                id="ir"
                type="number"
                value={form.interestRate}
                onChange={(e) => setForm({ ...form, interestRate: Number(e.target.value) })}
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="cd">{S.claim_cooldown}</Label>
              <Input
                id="cd"
                type="number"
                value={form.claimCooldownDays}
                onChange={(e) => setForm({ ...form, claimCooldownDays: Number(e.target.value) })}
              />
            </div>
            <div className="grid gap-2">
              <Label htmlFor="li">{S.investor_lockin}</Label>
              <Input
                id="li"
                type="number"
                value={form.investorLockInDays}
                onChange={(e) => setForm({ ...form, investorLockInDays: Number(e.target.value) })}
              />
            </div>
          </div>
          <div className="flex items-center gap-2">
            <Switch
              checked={form.requiresDaoApproval}
              onCheckedChange={(v) => setForm({ ...form, requiresDaoApproval: v })}
            />
            <Label>{S.requires_dao_approval}</Label>
          </div>
          <div className="grid gap-2">
            <Label htmlFor="cs">{S.credit_slash}</Label>
            <Input
              id="cs"
              type="number"
              value={form.creditSlashOnReject}
              onChange={(e) => setForm({ ...form, creditSlashOnReject: Number(e.target.value) })}
            />
          </div>
        </CardContent>
        <CardFooter>
          <Button disabled={!isDaoMember} className="bg-emerald-600 hover:bg-emerald-700" onClick={() => void submit()}>
            {S.submit}
          </Button>
        </CardFooter>
      </Card>
    </main>
  )
}
