import { useState, useCallback, useRef } from 'react'

export type AgentName = 'preground' | 'cover_letter_writer' | 'pdf_uploader' | 'link_generator'
export type AgentStatus = 'pending' | 'running' | 'completed' | 'failed'

export interface AgentState {
  name: AgentName
  label: string
  description: string
  status: AgentStatus
  detail?: string
}

export interface WorkflowResult {
  download_url: string
  preview_html: string
  summary: string
}

interface JobCreateResponse {
  job_id: string
  status: AgentStatus
}

interface JobStatusResponse {
  job_id: string
  status: AgentStatus
  events: Array<{ agent: AgentName; status: AgentStatus; detail?: string }>
  result?: WorkflowResult | null
  error?: string | null
}

const INITIAL_AGENTS: AgentState[] = [
  {
    name: 'preground',
    label: 'Context Loader',
    description: 'Fetch resume data and match keywords locally',
    status: 'pending',
  },
  {
    name: 'cover_letter_writer',
    label: 'Cover Letter Writer',
    description: 'Generate tailored cover letter with grounded context',
    status: 'pending',
  },
  {
    name: 'pdf_uploader',
    label: 'PDF Converter & Uploader',
    description: 'Convert to PDF and upload to S3',
    status: 'pending',
  },
  {
    name: 'link_generator',
    label: 'Link Generator',
    description: 'Generate secure download link',
    status: 'pending',
  },
]

export function useAgentStream() {
  const [agents, setAgents] = useState<AgentState[]>(INITIAL_AGENTS)
  const [result, setResult] = useState<WorkflowResult | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [isStreaming, setIsStreaming] = useState(false)
  const abortRef = useRef<AbortController | null>(null)

  const reset = useCallback(() => {
    setAgents(INITIAL_AGENTS)
    setResult(null)
    setError(null)
  }, [])

  const generate = useCallback(async (jobDescription: string) => {
    if (abortRef.current) abortRef.current.abort()
    const controller = new AbortController()
    abortRef.current = controller

    reset()
    setIsStreaming(true)

    try {
      const createRes = await fetch('/api/v1/agent/cover-letter/jobs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ job_description: jobDescription }),
        signal: controller.signal,
      })

      if (!createRes.ok) {
        const body = await createRes.json().catch(() => ({}))
        throw new Error(body.detail ?? `HTTP ${createRes.status}`)
      }

      const created = (await createRes.json()) as JobCreateResponse
      const started = Date.now()
      const maxWaitMs = 5 * 60 * 1000

      while (true) {
        if (Date.now() - started > maxWaitMs) {
          throw new Error('Cover letter generation did not finish within 5 minutes')
        }

        await new Promise(resolve => setTimeout(resolve, 1500))
        if (controller.signal.aborted) return

        const statusRes = await fetch(`/api/v1/agent/cover-letter/jobs/${created.job_id}`, {
          signal: controller.signal,
        })

        if (!statusRes.ok) {
          const body = await statusRes.json().catch(() => ({}))
          throw new Error(body.detail ?? `HTTP ${statusRes.status}`)
        }

        const status = (await statusRes.json()) as JobStatusResponse
        setAgents(prev =>
          prev.map(agent => {
            const latest = [...status.events].reverse().find(event => event.agent === agent.name)
            return latest ? { ...agent, status: latest.status, detail: latest.detail } : agent
          })
        )

        if (status.status === 'completed' && status.result) {
          setResult(status.result)
          break
        }
        if (status.status === 'failed') {
          throw new Error(status.error ?? 'Cover letter generation failed')
        }
      }
    } catch (err) {
      if (err instanceof Error && err.name !== 'AbortError') {
        setError(err.message)
      }
    } finally {
      setIsStreaming(false)
      abortRef.current = null
    }
  }, [reset])

  const cancel = useCallback(() => {
    if (abortRef.current) abortRef.current.abort()
  }, [])

  return { agents, result, error, isStreaming, generate, cancel, reset }
}
