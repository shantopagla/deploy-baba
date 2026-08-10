import { describe, it, expect, vi } from 'vitest'
import { render, screen, waitFor } from '../../utils/test-render'
import ChallengeDetail from '../../../routes/dashboard/ChallengeDetail'
import DashboardLayout from '../../../routes/dashboard/Layout'

vi.mock('../../../hooks/useAuth', () => ({
  useAuth: () => ({ loading: false, authenticated: true, email: 'test@example.com' }),
}))

describe('ChallengeDetail', () => {
  it('renders a placeholder for the removed "new" form (ADR-036)', () => {
    render(
      <DashboardLayout>
        <ChallengeDetail />
      </DashboardLayout>,
      { router: 'memory', route: '/dashboard/challenges/new', routes: [{ path: '/dashboard/challenges/:id' }] }
    )
    expect(screen.getByText('New Challenge')).toBeInTheDocument()
    expect(screen.getByText(/content\/challenges\/<slug>\.md/)).toBeInTheDocument()
  })

  it('shows loading state for an existing challenge', () => {
    render(
      <DashboardLayout>
        <ChallengeDetail />
      </DashboardLayout>,
      { router: 'memory', route: '/dashboard/challenges/1', routes: [{ path: '/dashboard/challenges/:id' }] }
    )
    expect(screen.getByText('Loading…')).toBeInTheDocument()
  })

  it('renders challenge details read-only, with no edit/delete controls', async () => {
    render(
      <DashboardLayout>
        <ChallengeDetail />
      </DashboardLayout>,
      { router: 'memory', route: '/dashboard/challenges/1', routes: [{ path: '/dashboard/challenges/:id' }] }
    )

    await waitFor(() => {
      expect(screen.queryByText('Loading…')).not.toBeInTheDocument()
    })

    expect(screen.getByRole('heading', { name: 'Portfolio RAG System' })).toBeInTheDocument()
    expect(screen.getByText('portfolio-rag')).toBeInTheDocument()
    expect(screen.getByText(/content\/challenges\/portfolio-rag\.md/)).toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'Save' })).not.toBeInTheDocument()
    expect(screen.queryByRole('button', { name: 'Delete' })).not.toBeInTheDocument()
    expect(screen.queryByLabelText('slug')).not.toBeInTheDocument()
  })

  it('shows an error for an unknown challenge id', async () => {
    render(
      <DashboardLayout>
        <ChallengeDetail />
      </DashboardLayout>,
      { router: 'memory', route: '/dashboard/challenges/999', routes: [{ path: '/dashboard/challenges/:id' }] }
    )

    await waitFor(() => {
      expect(screen.getByText('Challenge not found')).toBeInTheDocument()
    })
  })
})
