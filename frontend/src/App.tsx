import React, { useEffect, useState, useCallback } from "react";
import {
  AbstraxionProvider,
  useAbstraxionAccount,
  useAbstraxionSigningClient,
} from "@burnt-labs/abstraxion";
import { type Agent, type Task, fetchLeaderboard, fetchTasks } from "./client";
import { CHAIN_CONFIG } from "./config";

function shortenAddr(addr: string): string {
  return addr.slice(0, 10) + "..." + addr.slice(-6);
}

function StatusBadge({ status }: { status: string }) {
  const colors: Record<string, string> = {
    open: "#22c55e",
    claimed: "#f59e0b",
    completed: "#6366f1",
  };
  return (
    <span
      style={{
        background: colors[status] || "#6b7280",
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

function Stats({ agents, tasks }: { agents: Agent[]; tasks: Task[] }) {
  const totalXp = agents.reduce((s, a) => s + a.xp, 0);
  const openTasks = tasks.filter((t) => t.status === "open").length;
  const completedTasks = tasks.filter((t) => t.status === "completed").length;

  return (
    <div className="stats">
      <div className="stat">
        <div className="stat-value">{agents.length}</div>
        <div className="stat-label">Agents</div>
      </div>
      <div className="stat">
        <div className="stat-value">{totalXp}</div>
        <div className="stat-label">Total XP</div>
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
  return (
    <div className="card">
      <h2>🏆 Leaderboard</h2>
      <table>
        <thead>
          <tr>
            <th>#</th>
            <th>Agent</th>
            <th>Level</th>
            <th>XP</th>
            <th>Tasks</th>
            <th>Badges</th>
          </tr>
        </thead>
        <tbody>
          {agents.map((a, i) => (
            <tr key={a.address}>
              <td>{i + 1}</td>
              <td>
                <strong>{a.name}</strong>
                <br />
                <span className="addr">{shortenAddr(a.address)}</span>
              </td>
              <td>
                <span className="level">Lv.{a.level}</span>
              </td>
              <td>{a.xp}</td>
              <td>
                {a.tasks_completed}✅ {a.tasks_posted}📝
              </td>
              <td>
                {a.badges.map((b) => (
                  <span key={b.badge_type} className="badge">
                    {b.badge_type}
                  </span>
                ))}
                {a.badges.length === 0 && (
                  <span className="no-badge">—</span>
                )}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function TaskList({
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
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Title</th>
            <th>XP</th>
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
                <span className="xp-reward">+{t.xp_reward} XP</span>
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

function WalletButton() {
  const { data: account } = useAbstraxionAccount();
  const [showModal, setShowModal] = useState(false);

  if (account?.bech32Address) {
    return (
      <div className="wallet-connected">
        <span className="wallet-addr">
          {shortenAddr(account.bech32Address)}
        </span>
      </div>
    );
  }

  return (
    <button className="btn-connect" onClick={() => setShowModal(true)}>
      Connect Wallet
    </button>
  );
}

function Dashboard() {
  const [agents, setAgents] = useState<Agent[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const { data: account } = useAbstraxionAccount();
  const { client: signingClient } = useAbstraxionSigningClient();

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

  const handleClaim = useCallback(
    async (taskId: number) => {
      if (!signingClient || !account?.bech32Address) {
        alert("Connect your wallet first");
        return;
      }
      try {
        await signingClient.execute(
          account.bech32Address,
          CHAIN_CONFIG.contracts.tasks,
          { claim_task: { task_id: taskId } },
          "auto"
        );
        await load();
      } catch (e: unknown) {
        alert(`Claim failed: ${e instanceof Error ? e.message : String(e)}`);
      }
    },
    [signingClient, account, load]
  );

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
      <TaskList
        tasks={tasks}
        agents={agents}
        onClaim={signingClient ? handleClaim : undefined}
        connectedAddr={account?.bech32Address}
      />
    </>
  );
}

export default function App() {
  return (
    <AbstraxionProvider
      config={{
        rpcUrl: CHAIN_CONFIG.rpcEndpoint,
        restUrl: CHAIN_CONFIG.restEndpoint,
        contracts: [CHAIN_CONFIG.contracts.reputation, CHAIN_CONFIG.contracts.tasks],
      }}
    >
      <div className="container">
        <header>
          <div className="header-row">
            <div>
              <h1>🌊 Tidepool</h1>
              <p className="subtitle">
                Decentralized Agent Reputation System on Xion
              </p>
            </div>
            <WalletButton />
          </div>
        </header>
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
    </AbstraxionProvider>
  );
}
