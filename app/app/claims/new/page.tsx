"use client"

import React from "react"
import { useApp, useStrings } from "@/context/app-context"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import TxStatusToast from "@/components/tx-status-toast"
import { useToast } from "@/hooks/use-toast"

async function compressImage(file: File, maxW = 800, quality = 0.8): Promise<Blob> {
  const img = new Image()
  img.crossOrigin = "anonymous"
  const buf = await file.arrayBuffer()
  const blobUrl = URL.createObjectURL(new Blob([buf], { type: file.type }))
  img.src = blobUrl
  await new Promise((res, rej) => {
    img.onload = () => res(null)
    img.onerror = rej
  })
  const scale = Math.min(1, maxW / img.width)
  const w = Math.round(img.width * scale)
  const h = Math.round(img.height * scale)
  const canvas = document.createElement("canvas")
  canvas.width = w
  canvas.height = h
  const ctx = canvas.getContext("2d")!
  ctx.drawImage(img, 0, 0, w, h)
  const compressed = await new Promise<Blob>((resolve) => canvas.toBlob((b) => resolve(b!), "image/jpeg", quality))
  URL.revokeObjectURL(blobUrl)
  return compressed
}

export default function Page() {
  const { sdk } = useApp()
  const S = useStrings()
  const { toast } = useToast()
  const [file, setFile] = React.useState<File | null>(null)
  const [preview, setPreview] = React.useState<string | null>(null)
  const [amount, setAmount] = React.useState("10000000") // 1 XLM default
  const [description, setDescription] = React.useState("")
  const [subs, setSubs] = React.useState<{ id: string; label: string }[]>([])
  const [selectedSub, setSelectedSub] = React.useState<string>("")
  const [status, setStatus] = React.useState<"idle" | "pending" | "success" | "error">("idle")
  const [txId, setTxId] = React.useState<string | undefined>(undefined)

  React.useEffect(() => {
    ;(async () => {
      const all = await sdk.getSubscriptions()
      setSubs(all.map((s) => ({ id: s.id, label: `${s.id} (${s.policyId})` })))
      if (all[0]) setSelectedSub(all[0].id)
    })()
  }, [sdk])

  const onFile = async (f: File) => {
    try {
      const blob = await compressImage(f)
      const fileCompressed = new File([blob], f.name.replace(/\.\w+$/, ".jpg"), { type: "image/jpeg" })
      setFile(fileCompressed)
      setPreview(URL.createObjectURL(fileCompressed))
    } catch (e) {
      toast({ title: S.image_upload_failed, description: (e as any)?.message })
    }
  }

  const submit = async () => {
    if (!file) return
    setStatus("pending")
    const up = await sdk.uploadImage(file)
    // verify hash vs client file (simple rehash, same as upload uses)
    const buf = await file.arrayBuffer()
    const digest = await crypto.subtle.digest("SHA-256", buf)
    const hex = Array.from(new Uint8Array(digest))
      .map((b) => b.toString(16).padStart(2, "0"))
      .join("")
    if (hex !== up.hash) {
      setStatus("error")
      toast({ title: "Hash mismatch", description: "Please retry upload." })
      return
    }
    const res = await sdk.submitClaim({
      subscriptionId: selectedSub,
      amount,
      imageHash: up.hash,
      description,
    })
    setTxId(res.txId)
    if (!res.ok) {
      setStatus("error")
      toast({ title: S.failed, description: res.error })
      return
    }
    setStatus("success")
  }

  return (
    <main className="container max-w-3xl mx-auto p-4">
      <TxStatusToast status={status} txId={txId} />
      <Card>
        <CardHeader>
          <CardTitle>{S.claim_new}</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="grid gap-2">
            <Label htmlFor="sub">Subscription</Label>
            <select
              id="sub"
              value={selectedSub}
              onChange={(e) => setSelectedSub(e.target.value)}
              className="border rounded px-3 py-2"
            >
              {subs.map((s) => (
                <option key={s.id} value={s.id}>
                  {s.label}
                </option>
              ))}
            </select>
          </div>
          <div className="grid gap-2">
            <Label htmlFor="file">{S.upload_image}</Label>
            <Input
              id="file"
              type="file"
              accept="image/*"
              onChange={(e) => {
                const f = e.target.files?.[0]
                if (f) void onFile(f)
              }}
            />
            {preview && (
              <img
                src={preview || "/placeholder.svg"}
                alt={S.file_alt}
                className="w-48 h-48 object-cover rounded border"
              />
            )}
          </div>
          <div className="grid gap-2">
            <Label htmlFor="amt">{S.amount}</Label>
            <Input id="amt" type="number" value={amount} onChange={(e) => setAmount(e.target.value)} />
          </div>
          <div className="grid gap-2">
            <Label htmlFor="desc">{S.claim_description}</Label>
            <Textarea id="desc" value={description} onChange={(e) => setDescription(e.target.value)} />
          </div>
          <Button className="bg-emerald-600 hover:bg-emerald-700" onClick={() => void submit()}>
            {S.submit_claim}
          </Button>
        </CardContent>
      </Card>
    </main>
  )
}
