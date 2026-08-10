import { useEffect, useState } from 'react'
import { useParams, Link } from 'react-router-dom'

interface Challenge {
  id: number
  slug: string
  title: string
  job_id: number | null
  description: string
  short_description: string | null
  tech_stack: string[] | null
  category: string | null
  url: string | null
  image_url: string | null
  problem: string | null
  constraints: string | null
  decisions: string | null
  implementation: string | null
  outcomes: string | null
  metrics: string | null
  related_job_slug: string | null
  related_plan_module: string | null
  related_adr: string | null
  featured: boolean
  sort_order: number
}

interface Job {
  id: number
  slug: string
  company: string
  title: string
}

const LONG_TEXT_FIELDS = [
  ['description', 'Description'],
  ['problem', 'Problem'],
  ['constraints', 'Constraints'],
  ['decisions', 'Decisions'],
  ['implementation', 'Implementation'],
  ['outcomes', 'Outcomes'],
  ['metrics', 'Metrics'],
] as const

export default function ChallengeDetail() {
  const { id } = useParams<{ id: string }>()

  const [challenge, setChallenge] = useState<Challenge | null>(null)
  const [jobs, setJobs] = useState<Job[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    if (id === 'new') {
      setLoading(false)
      return
    }

    Promise.all([
      fetch('/api/jobs').then(r => r.json()).catch(() => []),
      fetch('/api/challenges').then(r => r.json()),
    ])
      .then(([jobData, chData]: [Job[], Challenge[]]) => {
        setJobs(Array.isArray(jobData) ? jobData : [])
        const ch = chData.find(c => c.id === Number(id))
        if (!ch) {
          setError('Challenge not found')
          return
        }
        setChallenge(ch)
      })
      .catch(() => setError('Failed to load challenge'))
      .finally(() => setLoading(false))
  }, [id])

  const sourceBanner = (slug: string) => (
    <div className="mb-6 rounded-lg border border-cyan-800 bg-cyan-950/40 px-4 py-3 text-sm text-cyan-200">
      Source of truth: <code className="text-cyan-100">content/challenges/{slug}.md</code>.
      Edit that file and run <code className="text-cyan-100">just challenges-migration</code> to
      publish changes — this page is read-only.
    </div>
  )

  if (id === 'new') {
    return (
      <div className="p-8 max-w-2xl">
        <Link to="/dashboard/challenges" className="text-sm text-gray-400 hover:text-gray-200 transition">
          ← Challenges
        </Link>
        <h1 className="text-2xl font-bold text-white mt-4 mb-6">New Challenge</h1>
        <div className="rounded-lg border border-gray-700 bg-gray-800 px-4 py-3 text-sm text-gray-300">
          New challenges are created by adding a file under{' '}
          <code className="text-gray-100">content/challenges/&lt;slug&gt;.md</code> and running{' '}
          <code className="text-gray-100">just challenges-migration</code> — there is no dashboard
          form for this anymore (ADR-036).
        </div>
      </div>
    )
  }

  if (loading) return <p className="p-8 text-gray-500 text-sm">Loading…</p>
  if (error || !challenge) {
    return (
      <div className="p-8 max-w-2xl">
        <Link to="/dashboard/challenges" className="text-sm text-gray-400 hover:text-gray-200 transition">
          ← Challenges
        </Link>
        <p className="text-red-400 text-sm mt-4">{error ?? 'Challenge not found'}</p>
      </div>
    )
  }

  const job = jobs.find(j => j.id === challenge.job_id)

  return (
    <div className="p-8 max-w-2xl">
      <Link to="/dashboard/challenges" className="text-sm text-gray-400 hover:text-gray-200 transition">
        ← Challenges
      </Link>
      <div className="flex items-center gap-2 mt-4 mb-6">
        <h1 className="text-2xl font-bold text-white">{challenge.title}</h1>
        {challenge.featured && (
          <span className="text-[10px] font-semibold px-1.5 py-0.5 rounded bg-cyan-900 text-cyan-300">
            Featured
          </span>
        )}
      </div>

      {sourceBanner(challenge.slug)}

      <dl className="space-y-4 text-sm">
        <div>
          <dt className="text-gray-400">Slug</dt>
          <dd className="text-white">{challenge.slug}</dd>
        </div>
        <div>
          <dt className="text-gray-400">Job</dt>
          <dd className="text-white">{job ? `${job.company} — ${job.title}` : challenge.related_job_slug ?? '—'}</dd>
        </div>
        {challenge.short_description && (
          <div>
            <dt className="text-gray-400">Short description</dt>
            <dd className="text-white">{challenge.short_description}</dd>
          </div>
        )}
        {challenge.tech_stack && challenge.tech_stack.length > 0 && (
          <div>
            <dt className="text-gray-400">Tech stack</dt>
            <dd className="text-white">{challenge.tech_stack.join(', ')}</dd>
          </div>
        )}
        {challenge.category && (
          <div>
            <dt className="text-gray-400">Category</dt>
            <dd className="text-white">{challenge.category}</dd>
          </div>
        )}
        {challenge.url && (
          <div>
            <dt className="text-gray-400">URL</dt>
            <dd>
              <a href={challenge.url} className="text-cyan-400 hover:text-cyan-300 transition" target="_blank" rel="noreferrer">
                {challenge.url}
              </a>
            </dd>
          </div>
        )}

        {LONG_TEXT_FIELDS.map(([key, label]) =>
          challenge[key] ? (
            <div key={key}>
              <dt className="text-gray-400">{label}</dt>
              <dd className="text-white whitespace-pre-wrap">{challenge[key]}</dd>
            </div>
          ) : null
        )}

        <div>
          <dt className="text-gray-400">Related plan module / ADR</dt>
          <dd className="text-white">
            {[challenge.related_plan_module, challenge.related_adr].filter(Boolean).join(' / ') || '—'}
          </dd>
        </div>
        <div>
          <dt className="text-gray-400">Sort order</dt>
          <dd className="text-white">{challenge.sort_order}</dd>
        </div>
      </dl>
    </div>
  )
}
