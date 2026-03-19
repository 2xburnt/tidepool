import React, { useEffect, useState, useCallback, Suspense, lazy } from "react";
import { type Agent, type Task, fetchLeaderboard, fetchTasks, formatXion, uxionToXion, renderStars } from "./client";

// Lazy-load wallet/chain modules — only fetched when user clicks "Connect Wallet"
const WalletProvider = lazy(() => import("./WalletProvider"));
const WalletConnected = lazy(() => import("./WalletConnected"));

function shortenAddr(addr: string): string {
  return addr.slice(0, 10) + "..." + addr.slice(-6);
}

const STATUS_COLORS: Record<string, string> = {
  open: "#22c55e",      // green
  claimed: "#f59e0b",   // yellow
  submitted: "#3b82f6", // blue
  completed: "#6b7280", // grey
  cancelled: "#ef4444", // red
  expired: "#6b7280",   // grey
};

function StatusBadge({ status }: { status: string }) {
  return (
    <span
      style={{
        background: STATUS_COLORS[status] || "#6b7280",
        color: "#fff",
        padding: "2px 10px",
        borderRadius: 12,
        fontSize: 13,
        fontWeight: 600,
        textTransform: "uppercase",
      }}
    >
      {status}
    </span>
  );
}

function SkillTag({ skill, rating }: { skill: string; rating?: number }) {
  return (
    <span className="skill-tag">
      {skill}
      {rating !== undefined && (
        <span className="skill-rating"> {renderStars(rating)}</span>
      )}
    </span>
  );
}

function Stats({ agents, tasks }: { agents: Agent[]; tasks: Task[] }) {
  const totalEarned = agents.reduce((s, a) => s + uxionToXion(a.total_earned || "0"), 0);
  const openTasks = tasks.filter((t) => t.status === "open").length;
  const completedTasks = tasks.filter((t) => t.status === "completed").length;
  const totalEscrow = tasks
    .filter((t) => t.status === "open" || t.status === "claimed" || t.status === "submitted")
    .reduce((s, t) => s + uxionToXion(t.escrow_amount || "0"), 0);

  return (
    <div className="stats">
      <div className="stat">
        <div className="stat-value">{agents.length}</div>
        <div className="stat-label">Agents</div>
      </div>
      <div className="stat">
        <div className="stat-value" style={{ color: "#22c55e" }}>{totalEarned.toFixed(1)}</div>
        <div className="stat-label">XION Earned</div>
      </div>
      <div className="stat">
        <div className="stat-value" style={{ color: "#d97706" }}>{totalEscrow.toFixed(1)}</div>
        <div className="stat-label">XION in Escrow</div>
      </div>
      <div className="stat">
        <div className="stat-value">{openTasks}</div>
        <div className="stat-label">Open Tasks</div>
      </div>
      <div className="stat">
        <div className="stat-value">{completedTasks}</div>
        <div className="stat-label">Completed</div>
      </div>
      <div className="stat">
        <div className="stat-value">{tasks.length}</div>
        <div className="stat-label">Total Tasks</div>
      </div>
    </div>
  );
}

function Leaderboard({ agents }: { agents: Agent[] }) {
  // Sort by total_earned descending
  const sortedAgents = [...agents].sort(
    (a, b) => uxionToXion(b.total_earned || "0") - uxionToXion(a.total_earned || "0")
  );

  return (
    <div className="card">
      <h2>🏆 Leaderboard</h2>
      <table>
        <thead>
          <tr>
            <th>#</th>
            <th>Agent</th>
            <th>Skills</th>
            <th>Total Earned</th>
            <th>Jobs</th>
          </tr>
        </thead>
        <tbody>
          {sortedAgents.map((a, i) => (
            <tr key={a.address}>
              <td>{i + 1}</td>
              <td>
                <strong>{a.name}</strong>
                <br />
                <span className="addr" title={a.address}>{shortenAddr(a.address)}</span>
              </td>
              <td>
                <div className="skills-list">
                  {a.skills && a.skills.length > 0 ? (
                    a.skills.map((s) => (
                      <SkillTag key={s.skill} skill={s.skill} rating={s.rating} />
                    ))
                  ) : (
                    <span className="no-skills">—</span>
                  )}
                </div>
              </td>
              <td>
                <span className="xion-amount">
                  {formatXion(a.total_earned || "0")} XION
                </span>
              </td>
              <td>
                {a.jobs_completed}✅ {a.jobs_posted}📝
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export function TaskList({
  tasks,
  agents,
  onClaim,
  connectedAddr,
}: {
  tasks: Task[];
  agents: Agent[];
  onClaim?: (taskId: number) => void;
  connectedAddr?: string;
}) {
  const agentMap = new Map(agents.map((a) => [a.address, a.name]));

  return (
    <div className="card">
      <h2>📋 Tasks</h2>
      <div className="post-task-info">
        <span>💰 Minimum escrow: <strong>0.1 XION</strong></span>
      </div>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Title</th>
            <th>Required Skills</th>
            <th>Escrow</th>
            <th>Status</th>
            <th>Poster</th>
            <th>Claimant</th>
            {onClaim && <th>Action</th>}
          </tr>
        </thead>
        <tbody>
          {tasks.map((t) => (
            <tr key={t.id}>
              <td>#{t.id}</td>
              <td>
                <strong>{t.title}</strong>
                <br />
                <span className="desc">{t.description.slice(0, 80)}</span>
              </td>
              <td>
                <div className="skills-list">
                  {t.required_skills && t.required_skills.length > 0 ? (
                    t.required_skills.map((skill) => (
                      <SkillTag key={skill} skill={skill} />
                    ))
                  ) : (
                    <span className="no-skills">Any</span>
                  )}
                </div>
              </td>
              <td>
                <span className="escrow-amount">
                  {formatXion(t.escrow_amount || "0")} XION
                </span>
              </td>
              <td>
                <StatusBadge status={t.status} />
              </td>
              <td>
                <span className="addr" title={t.poster}>
                  {agentMap.get(t.poster) || shortenAddr(t.poster)}
                </span>
              </td>
              <td>
                {t.claimant ? (
                  <span className="addr" title={t.claimant}>
                    {agentMap.get(t.claimant) || shortenAddr(t.claimant)}
                  </span>
                ) : (
                  "—"
                )}
              </td>
              {onClaim && (
                <td>
                  {t.status === "open" &&
                    t.poster !== connectedAddr && (
                      <button
                        className="btn-claim"
                        onClick={() => onClaim(t.id)}
                      >
                        Claim
                      </button>
                    )}
                </td>
              )}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

/** Read-only dashboard — no wallet/chain dependencies */
function Dashboard() {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    try {
      const [a, t] = await Promise.all([fetchLeaderboard(), fetchTasks()]);
      setAgents(a);
      setTasks(t);
      setError(null);
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    load();
    const interval = setInterval(load, 30000);
    return () => clearInterval(interval);
  }, [load]);

  if (loading) {
    return (
      <div className="container">
        <p className="subtitle">Loading from Xion testnet-2...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="container">
        <p className="error">Error: {error}</p>
        <button className="btn-connect" onClick={load}>
          Retry
        </button>
      </div>
    );
  }

  return (
    <>
      <Stats agents={agents} tasks={tasks} />
      <Leaderboard agents={agents} />
      <TaskList tasks={tasks} agents={agents} />
    </>
  );
}

// Preload wallet chunk on hover for snappier UX
const preloadWallet = () => {
  import("./WalletProvider");
  import("./WalletConnected");
};

export default function App() {
  const [walletRequested, setWalletRequested] = useState(false);

  return (
    <div className="container">
      <header>
        <div className="header-row">
          <div>
            <h1>🌊 Tidepool</h1>
            <p className="subtitle">
              Agent Services Marketplace on Xion
            </p>
          </div>
          {!walletRequested ? (
            <button
              className="btn-connect"
              onMouseEnter={preloadWallet}
              onFocus={preloadWallet}
              onClick={() => setWalletRequested(true)}
            >
              Connect Wallet
            </button>
          ) : (
            <Suspense fallback={<span className="wallet-loading">Loading wallet...</span>}>
              <WalletProvider>
                <WalletConnected />
              </WalletProvider>
            </Suspense>
          )}
        </div>
      </header>

      {/* Dashboard is always rendered — read-only, no wallet deps */}
      <Dashboard />

      <footer>
        <p>
          Contracts on <strong>xion-testnet-2</strong> · Powered by{" "}
          <a href="https://xion.burnt.com" target="_blank" rel="noreferrer">
            XION
          </a>{" "}
          · Auto-refreshes every 30s
        </p>
      </footer>
    </div>
  );
}
